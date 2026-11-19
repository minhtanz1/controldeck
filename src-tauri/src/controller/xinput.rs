use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use windows::Win32::UI::Input::XboxController::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ControllerState {
    pub connected: bool,
    pub player_index: u32,
    pub buttons: HashMap<String, bool>,
    pub left_trigger: f32,
    pub right_trigger: f32,
    pub left_stick_x: f32,
    pub left_stick_y: f32,
    pub right_stick_x: f32,
    pub right_stick_y: f32,
}

pub struct XInputPoller {
    player_index: u32,
    last_state: ControllerState,
}

impl XInputPoller {
    pub fn new(player_index: u32) -> Self {
        Self {
            player_index,
            last_state: ControllerState::default(),
        }
    }

    pub fn poll(&mut self) -> ControllerState {
        let mut state = XINPUT_STATE::default();
        let result = unsafe { XInputGetState(self.player_index, &mut state) };

        if result != 0 {
            let mut disconnected = ControllerState::default();
            disconnected.player_index = self.player_index;
            self.last_state = disconnected.clone();
            return disconnected;
        }

        let gp = state.Gamepad;
        let mut buttons = HashMap::new();

        let wb = gp.wButtons;
        buttons.insert("DPadUp".to_string(), (wb & XINPUT_GAMEPAD_DPAD_UP) == XINPUT_GAMEPAD_DPAD_UP);
        buttons.insert("DPadDown".to_string(), (wb & XINPUT_GAMEPAD_DPAD_DOWN) == XINPUT_GAMEPAD_DPAD_DOWN);
        buttons.insert("DPadLeft".to_string(), (wb & XINPUT_GAMEPAD_DPAD_LEFT) == XINPUT_GAMEPAD_DPAD_LEFT);
        buttons.insert("DPadRight".to_string(), (wb & XINPUT_GAMEPAD_DPAD_RIGHT) == XINPUT_GAMEPAD_DPAD_RIGHT);
        buttons.insert("Start".to_string(), (wb & XINPUT_GAMEPAD_START) == XINPUT_GAMEPAD_START);
        buttons.insert("Back".to_string(), (wb & XINPUT_GAMEPAD_BACK) == XINPUT_GAMEPAD_BACK);
        buttons.insert("LeftThumb".to_string(), (wb & XINPUT_GAMEPAD_LEFT_THUMB) == XINPUT_GAMEPAD_LEFT_THUMB);
        buttons.insert("RightThumb".to_string(), (wb & XINPUT_GAMEPAD_RIGHT_THUMB) == XINPUT_GAMEPAD_RIGHT_THUMB);
        buttons.insert("LB".to_string(), (wb & XINPUT_GAMEPAD_LEFT_SHOULDER) == XINPUT_GAMEPAD_LEFT_SHOULDER);
        buttons.insert("RB".to_string(), (wb & XINPUT_GAMEPAD_RIGHT_SHOULDER) == XINPUT_GAMEPAD_RIGHT_SHOULDER);
        buttons.insert("A".to_string(), (wb & XINPUT_GAMEPAD_A) == XINPUT_GAMEPAD_A);
        buttons.insert("B".to_string(), (wb & XINPUT_GAMEPAD_B) == XINPUT_GAMEPAD_B);
        buttons.insert("X".to_string(), (wb & XINPUT_GAMEPAD_X) == XINPUT_GAMEPAD_X);
        buttons.insert("Y".to_string(), (wb & XINPUT_GAMEPAD_Y) == XINPUT_GAMEPAD_Y);

        let lt = (gp.bLeftTrigger as f32) / 255.0;
        let rt = (gp.bRightTrigger as f32) / 255.0;

        // Default deadzones: Left thumb = 7849, Right thumb = 8689
        let lx = apply_deadzone(gp.sThumbLX, 7849);
        let ly = apply_deadzone(gp.sThumbLY, 7849);
        let rx = apply_deadzone(gp.sThumbRX, 8689);
        let ry = apply_deadzone(gp.sThumbRY, 8689);

        let new_state = ControllerState {
            connected: true,
            player_index: self.player_index,
            buttons,
            left_trigger: lt,
            right_trigger: rt,
            left_stick_x: lx,
            left_stick_y: ly,
            right_stick_x: rx,
            right_stick_y: ry,
        };

        self.last_state = new_state.clone();
        new_state
    }
}

fn apply_deadzone(value: i16, deadzone: i16) -> f32 {
    let abs_val = (value as i32).abs();
    let dz = deadzone as i32;

    if abs_val < dz {
        0.0
    } else {
        let max_val = 32767.0;
        let norm = (abs_val - dz) as f32 / (max_val - dz as f32);
        if value < 0 {
            -norm.clamp(0.0, 1.0)
        } else {
            norm.clamp(0.0, 1.0)
        }
    }
}
