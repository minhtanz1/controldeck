use crate::controller::ControllerState;
use crate::detector::get_active_window;
use crate::injection::{
    execute_action, release_action, send_mouse_down, send_mouse_move, send_mouse_up,
    send_mouse_wheel,
};
use crate::profile::ProfileStore;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct MappingEngine {
    pub enabled: Arc<AtomicBool>,
    store: Arc<Mutex<ProfileStore>>,
    sim_state: Arc<Mutex<ControllerState>>,
}

impl MappingEngine {
    pub fn new(store: Arc<Mutex<ProfileStore>>, sim_state: Arc<Mutex<ControllerState>>) -> Self {
        Self {
            enabled: Arc::new(AtomicBool::new(true)),
            store,
            sim_state,
        }
    }

    pub fn set_enabled(&self, val: bool) {
        self.enabled.store(val, Ordering::SeqCst);
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::SeqCst)
    }

    pub fn start_background_loop(&self) {
        let enabled = self.enabled.clone();
        let store = self.store.clone();
        let sim_state = self.sim_state.clone();

        thread::spawn(move || {
            let mut last_buttons: HashMap<String, bool> = HashMap::new();
            let mut last_lt_active = false;
            let mut last_rt_active = false;

            // Stick processing sub-pixel accumulators and drag states
            let mut ls_sub_x = 0.0f32;
            let mut ls_sub_y = 0.0f32;
            let mut ls_scroll_acc = 0.0f32;
            let mut ls_is_dragging = false;

            let mut rs_sub_x = 0.0f32;
            let mut rs_sub_y = 0.0f32;
            let mut rs_scroll_acc = 0.0f32;
            let mut rs_is_dragging = false;

            let mut poller = crate::controller::XInputPoller::new(0);
            let mut win_check_counter = 0u32;

            loop {
                thread::sleep(Duration::from_millis(4));

                // 1. Auto Profile Switch Check (every ~500ms = 125 ticks)
                win_check_counter += 1;
                if win_check_counter >= 125 {
                    win_check_counter = 0;
                    if enabled.load(Ordering::Relaxed) {
                        let active_win = get_active_window();
                        if !active_win.title.is_empty() {
                            let mut st = store.lock().unwrap();
                            let t_lower = active_win.title.to_lowercase();
                            let p_lower = active_win.process_name.to_lowercase();

                            for p in st.get_profiles() {
                                for rule in &p.app_rules {
                                    let pat = rule.pattern.to_lowercase();
                                    if pat.is_empty() {
                                        continue;
                                    }
                                    let matches = match rule.match_type.as_str() {
                                        "process_name" => p_lower.contains(&pat),
                                        _ => t_lower.contains(&pat),
                                    };
                                    if matches {
                                        if let Some(curr) = st.get_active_profile() {
                                            if curr.id != p.id {
                                                st.set_active_profile(&p.id);
                                            }
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }

                if !enabled.load(Ordering::Relaxed) {
                    continue;
                }

                // 2. Poll Real Controller, fallback to Simulated
                let real_state = poller.poll();
                let state = if real_state.connected {
                    real_state
                } else {
                    sim_state.lock().unwrap().clone()
                };

                let active_profile = {
                    let st = store.lock().unwrap();
                    st.get_active_profile()
                };

                let profile = match active_profile {
                    Some(p) => p,
                    None => continue,
                };

                // 3. Diff Buttons
                let button_names = [
                    "A", "B", "X", "Y", "LB", "RB", "Start", "Back",
                    "LeftThumb", "RightThumb", "DPadUp", "DPadDown", "DPadLeft", "DPadRight",
                ];

                // Check Combos First
                let mut combo_handled = false;
                for combo in &profile.mappings.combos {
                    if combo.buttons.len() == 2 {
                        let b1 = state.buttons.get(&combo.buttons[0]).copied().unwrap_or(false);
                        let b2 = state.buttons.get(&combo.buttons[1]).copied().unwrap_or(false);
                        let was_b1 = last_buttons.get(&combo.buttons[0]).copied().unwrap_or(false);
                        let was_b2 = last_buttons.get(&combo.buttons[1]).copied().unwrap_or(false);

                        if b1 && b2 && (!was_b1 || !was_b2) {
                            execute_action(&combo.action);
                            combo_handled = true;
                            break;
                        }
                    }
                }

                if !combo_handled {
                    for &btn in &button_names {
                        let is_down = state.buttons.get(btn).copied().unwrap_or(false);
                        let was_down = last_buttons.get(btn).copied().unwrap_or(false);

                        if is_down && !was_down {
                            // Button pressed!
                            if let Some(action) = profile.mappings.buttons.get(btn) {
                                execute_action(action);
                            }
                        } else if !is_down && was_down {
                            // Button released!
                            if let Some(action) = profile.mappings.buttons.get(btn) {
                                release_action(action);
                            }
                        }

                        last_buttons.insert(btn.to_string(), is_down);
                    }
                }

                // 4. Triggers
                if let Some(lt_cfg) = profile.mappings.triggers.get("LeftTrigger") {
                    let is_active = state.left_trigger >= lt_cfg.threshold;
                    if is_active && !last_lt_active {
                        execute_action(&lt_cfg.action);
                    } else if !is_active && last_lt_active {
                        release_action(&lt_cfg.action);
                    }
                    last_lt_active = is_active;
                }

                if let Some(rt_cfg) = profile.mappings.triggers.get("RightTrigger") {
                    let is_active = state.right_trigger >= rt_cfg.threshold;
                    if is_active && !last_rt_active {
                        execute_action(&rt_cfg.action);
                    } else if !is_active && last_rt_active {
                        release_action(&rt_cfg.action);
                    }
                    last_rt_active = is_active;
                }

                // 5. Analog Sticks (Left Stick & Right Stick)
                if let Some(ls_cfg) = profile.mappings.sticks.get("LeftStick") {
                    process_stick(
                        ls_cfg,
                        state.left_stick_x,
                        state.left_stick_y,
                        &mut ls_sub_x,
                        &mut ls_sub_y,
                        &mut ls_scroll_acc,
                        &mut ls_is_dragging,
                    );
                } else if state.left_stick_x.abs() > 0.15 || state.left_stick_y.abs() > 0.15 {
                    // Default fallback: Left Stick controls mouse
                    let default_cfg = crate::profile::schema::StickAction {
                        deadzone: 0.15,
                        mode: "mouse".to_string(),
                        sensitivity: 1.0,
                        drag_button: None,
                        up: None,
                        down: None,
                        left: None,
                        right: None,
                    };
                    process_stick(
                        &default_cfg,
                        state.left_stick_x,
                        state.left_stick_y,
                        &mut ls_sub_x,
                        &mut ls_sub_y,
                        &mut ls_scroll_acc,
                        &mut ls_is_dragging,
                    );
                }

                if let Some(rs_cfg) = profile.mappings.sticks.get("RightStick") {
                    process_stick(
                        rs_cfg,
                        state.right_stick_x,
                        state.right_stick_y,
                        &mut rs_sub_x,
                        &mut rs_sub_y,
                        &mut rs_scroll_acc,
                        &mut rs_is_dragging,
                    );
                } else if state.right_stick_y.abs() > 0.15 {
                    // Default fallback: Right Stick controls scroll wheel
                    let default_cfg = crate::profile::schema::StickAction {
                        deadzone: 0.15,
                        mode: "scroll".to_string(),
                        sensitivity: 1.0,
                        drag_button: None,
                        up: None,
                        down: None,
                        left: None,
                        right: None,
                    };
                    process_stick(
                        &default_cfg,
                        state.right_stick_x,
                        state.right_stick_y,
                        &mut rs_sub_x,
                        &mut rs_sub_y,
                        &mut rs_scroll_acc,
                        &mut rs_is_dragging,
                    );
                }
            }
        });
    }
}

fn process_stick(
    cfg: &crate::profile::schema::StickAction,
    x: f32,
    y: f32,
    sub_x: &mut f32,
    sub_y: &mut f32,
    scroll_acc: &mut f32,
    is_dragging: &mut bool,
) {
    let deadzone = cfg.deadzone.clamp(0.01, 0.95);
    let mag = (x * x + y * y).sqrt();

    if mag < deadzone {
        if *is_dragging {
            if let Some(ref btn) = cfg.drag_button {
                if btn == "left" || btn == "right" {
                    send_mouse_up(btn);
                }
            }
            *is_dragging = false;
        }
        *sub_x = 0.0;
        *sub_y = 0.0;
        return;
    }

    let norm_mag = ((mag - deadzone) / (1.0 - deadzone)).clamp(0.0, 1.0);
    let nx = x / mag;
    let ny = y / mag;

    match cfg.mode.as_str() {
        "mouse" => {
            if let Some(ref btn) = cfg.drag_button {
                if (btn == "left" || btn == "right") && !*is_dragging {
                    send_mouse_down(btn);
                    *is_dragging = true;
                }
            }

            let speed = norm_mag.powf(1.4) * 12.0 * cfg.sensitivity.max(0.1);
            let move_x = nx * speed + *sub_x;
            let move_y = (-ny) * speed + *sub_y; // Invert Y for screen coordinates

            let int_x = move_x as i32;
            let int_y = move_y as i32;

            if int_x != 0 || int_y != 0 {
                send_mouse_move(int_x, int_y);
                *sub_x = move_x - (int_x as f32);
                *sub_y = move_y - (int_y as f32);
            } else {
                *sub_x = move_x;
                *sub_y = move_y;
            }
        }
        "scroll" => {
            *scroll_acc += ny * norm_mag * 12.0 * cfg.sensitivity.max(0.1);
            if scroll_acc.abs() >= 8.0 {
                let ticks = (*scroll_acc / 8.0) as i32;
                send_mouse_wheel(ticks * 20);
                *scroll_acc -= (ticks as f32) * 8.0;
            }
        }
        _ => {}
    }
}
