use crate::profile::schema::{Action, MacroStep};
use std::collections::HashSet;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

static TOGGLED_KEYS: Mutex<Option<HashSet<String>>> = Mutex::new(None);
static TOGGLED_MOUSE: Mutex<Option<HashSet<String>>> = Mutex::new(None);

pub fn string_to_vk(name: &str) -> Option<VIRTUAL_KEY> {
    let upper = name.trim().to_uppercase();
    match upper.as_str() {
        // Letters A-Z
        "A" => Some(VK_A),
        "B" => Some(VK_B),
        "C" => Some(VK_C),
        "D" => Some(VK_D),
        "E" => Some(VK_E),
        "F" => Some(VK_F),
        "G" => Some(VK_G),
        "H" => Some(VK_H),
        "I" => Some(VK_I),
        "J" => Some(VK_J),
        "K" => Some(VK_K),
        "L" => Some(VK_L),
        "M" => Some(VK_M),
        "N" => Some(VK_N),
        "O" => Some(VK_O),
        "P" => Some(VK_P),
        "Q" => Some(VK_Q),
        "R" => Some(VK_R),
        "S" => Some(VK_S),
        "T" => Some(VK_T),
        "U" => Some(VK_U),
        "V" => Some(VK_V),
        "W" => Some(VK_W),
        "X" => Some(VK_X),
        "Y" => Some(VK_Y),
        "Z" => Some(VK_Z),

        // Numbers 0-9
        "0" => Some(VK_0),
        "1" => Some(VK_1),
        "2" => Some(VK_2),
        "3" => Some(VK_3),
        "4" => Some(VK_4),
        "5" => Some(VK_5),
        "6" => Some(VK_6),
        "7" => Some(VK_7),
        "8" => Some(VK_8),
        "9" => Some(VK_9),

        // Navigation & Editing
        "SPACE" | " " => Some(VK_SPACE),
        "ENTER" | "RETURN" => Some(VK_RETURN),
        "TAB" => Some(VK_TAB),
        "ESCAPE" | "ESC" => Some(VK_ESCAPE),
        "BACKSPACE" | "BACK" => Some(VK_BACK),
        "DELETE" | "DEL" => Some(VK_DELETE),
        "INSERT" | "INS" => Some(VK_INSERT),
        "HOME" => Some(VK_HOME),
        "END" => Some(VK_END),
        "PAGEUP" | "PGUP" => Some(VK_PRIOR),
        "PAGEDOWN" | "PGDN" => Some(VK_NEXT),

        // Arrows
        "UP" => Some(VK_UP),
        "DOWN" => Some(VK_DOWN),
        "LEFT" => Some(VK_LEFT),
        "RIGHT" => Some(VK_RIGHT),

        // Modifiers
        "CTRL" | "CONTROL" | "LCTRL" => Some(VK_LCONTROL),
        "RCTRL" => Some(VK_RCONTROL),
        "ALT" | "LALT" | "MENU" => Some(VK_LMENU),
        "RALT" => Some(VK_RMENU),
        "SHIFT" | "LSHIFT" => Some(VK_LSHIFT),
        "RSHIFT" => Some(VK_RSHIFT),
        "WIN" | "WINDOWS" | "LWIN" => Some(VK_LWIN),
        "RWIN" => Some(VK_RWIN),

        // Function keys F1 - F12
        "F1" => Some(VK_F1),
        "F2" => Some(VK_F2),
        "F3" => Some(VK_F3),
        "F4" => Some(VK_F4),
        "F5" => Some(VK_F5),
        "F6" => Some(VK_F6),
        "F7" => Some(VK_F7),
        "F8" => Some(VK_F8),
        "F9" => Some(VK_F9),
        "F10" => Some(VK_F10),
        "F11" => Some(VK_F11),
        "F12" => Some(VK_F12),

        // Punctuation
        "," | "COMMA" => Some(VK_OEM_COMMA),
        "." | "PERIOD" => Some(VK_OEM_PERIOD),
        "/" | "SLASH" => Some(VK_OEM_2),
        ";" | "SEMICOLON" => Some(VK_OEM_1),
        "'" | "QUOTE" => Some(VK_OEM_7),
        "[" | "LBRACKET" => Some(VK_OEM_4),
        "]" | "RBRACKET" => Some(VK_OEM_6),
        "-" | "MINUS" => Some(VK_OEM_MINUS),
        "=" | "PLUS" => Some(VK_OEM_PLUS),
        "`" | "TILDE" => Some(VK_OEM_3),

        _ => None,
    }
}

pub fn send_key_down(vk: VIRTUAL_KEY) {
    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: 0,
                dwFlags: KEYBD_EVENT_FLAGS(0),
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    unsafe {
        let _ = SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

pub fn send_key_up(vk: VIRTUAL_KEY) {
    let input = INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: vk,
                wScan: 0,
                dwFlags: KEYEVENTF_KEYUP,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    unsafe {
        let _ = SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

pub fn tap_key(vk: VIRTUAL_KEY) {
    send_key_down(vk);
    thread::sleep(Duration::from_millis(15));
    send_key_up(vk);
}

// ================= MOUSE INJECTION =================

pub fn send_mouse_move(dx: i32, dy: i32) {
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx,
                dy,
                mouseData: 0,
                dwFlags: MOUSEEVENTF_MOVE,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    unsafe {
        let _ = SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

pub fn send_mouse_down(btn: &str) {
    let flags = match btn {
        "right" => MOUSEEVENTF_RIGHTDOWN,
        "middle" => MOUSEEVENTF_MIDDLEDOWN,
        _ => MOUSEEVENTF_LEFTDOWN,
    };

    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: 0,
                dy: 0,
                mouseData: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    unsafe {
        let _ = SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

pub fn send_mouse_up(btn: &str) {
    let flags = match btn {
        "right" => MOUSEEVENTF_RIGHTUP,
        "middle" => MOUSEEVENTF_MIDDLEUP,
        _ => MOUSEEVENTF_LEFTUP,
    };

    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: 0,
                dy: 0,
                mouseData: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    unsafe {
        let _ = SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

pub fn send_mouse_click(btn: &str) {
    send_mouse_down(btn);
    thread::sleep(Duration::from_millis(15));
    send_mouse_up(btn);
}

pub fn send_mouse_wheel(delta: i32) {
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: 0,
                dy: 0,
                mouseData: delta as u32,
                dwFlags: MOUSEEVENTF_WHEEL,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    unsafe {
        let _ = SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

pub fn send_text_unicode(text: &str) {
    for c in text.encode_utf16() {
        let down = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: c,
                    dwFlags: KEYEVENTF_UNICODE,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        let up = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(0),
                    wScan: c,
                    dwFlags: KEYEVENTF_UNICODE | KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        unsafe {
            let _ = SendInput(&[down, up], std::mem::size_of::<INPUT>() as i32);
        }
    }
}

// ================= ACTIONS =================

pub fn execute_action(action: &Action) {
    match action {
        Action::Key { key, modifiers, on } => {
            if let Some(vk) = string_to_vk(key) {
                let mod_vks: Vec<VIRTUAL_KEY> = modifiers.iter().filter_map(|m| string_to_vk(m)).collect();

                // Press modifiers
                for &m in &mod_vks {
                    send_key_down(m);
                }

                if on == "hold" {
                    send_key_down(vk);
                } else {
                    tap_key(vk);
                    // Release modifiers
                    for &m in mod_vks.iter().rev() {
                        send_key_up(m);
                    }
                }
            }
        }

        Action::ComboKey { key, modifiers } => {
            if let Some(vk) = string_to_vk(key) {
                let mod_vks: Vec<VIRTUAL_KEY> = modifiers.iter().filter_map(|m| string_to_vk(m)).collect();

                // Hold down modifiers
                for &m in &mod_vks {
                    send_key_down(m);
                }
                thread::sleep(Duration::from_millis(10));

                // Tap main key
                tap_key(vk);
                thread::sleep(Duration::from_millis(10));

                // Release modifiers
                for &m in mod_vks.iter().rev() {
                    send_key_up(m);
                }
            }
        }

        Action::Toggle { key, modifiers: _, state: _ } => {
            if let Some(vk) = string_to_vk(key) {
                let mut guard = TOGGLED_KEYS.lock().unwrap();
                let set = guard.get_or_insert_with(HashSet::new);
                if set.contains(key) {
                    send_key_up(vk);
                    set.remove(key);
                } else {
                    send_key_down(vk);
                    set.insert(key.clone());
                }
            }
        }

        Action::Mouse { button, on } => {
            match on.as_str() {
                "hold" => send_mouse_down(button),
                "release" => send_mouse_up(button),
                "toggle" => {
                    let mut guard = TOGGLED_MOUSE.lock().unwrap();
                    let set = guard.get_or_insert_with(HashSet::new);
                    if set.contains(button) {
                        send_mouse_up(button);
                        set.remove(button);
                    } else {
                        send_mouse_down(button);
                        set.insert(button.clone());
                    }
                }
                _ => send_mouse_click(button),
            }
        }

        Action::Macro { sequence, repeat: _ } => {
            let seq = sequence.clone();
            thread::spawn(move || {
                for step in seq {
                    execute_macro_step(&step);
                }
            });
        }
    }
}

pub fn release_action(action: &Action) {
    match action {
        Action::Key { key, modifiers, on } => {
            if on == "hold" {
                if let Some(vk) = string_to_vk(key) {
                    send_key_up(vk);
                }
                for m in modifiers.iter().rev() {
                    if let Some(mod_vk) = string_to_vk(m) {
                        send_key_up(mod_vk);
                    }
                }
            }
        }
        Action::Mouse { button, on } => {
            if on == "hold" {
                send_mouse_up(button);
            }
        }
        _ => {}
    }
}

fn execute_macro_step(step: &MacroStep) {
    if let Some(vk) = string_to_vk(&step.key) {
        let mod_vks: Vec<VIRTUAL_KEY> = step.modifiers.iter().filter_map(|m| string_to_vk(m)).collect();
        for &m in &mod_vks {
            send_key_down(m);
        }
        tap_key(vk);
        for &m in mod_vks.iter().rev() {
            send_key_up(m);
        }
        if step.delay_ms > 0 {
            thread::sleep(Duration::from_millis(step.delay_ms));
        }
    }
}
