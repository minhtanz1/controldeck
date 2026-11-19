use super::schema::*;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct ProfileStore {
    base_dir: PathBuf,
    profiles: HashMap<String, Profile>,
    active_profile_id: String,
}

impl ProfileStore {
    pub fn new() -> Self {
        let base_dir = dirs_base();
        let _ = fs::create_dir_all(&base_dir);

        let mut store = Self {
            base_dir,
            profiles: HashMap::new(),
            active_profile_id: "default".to_string(),
        };

        store.load_all();
        if store.profiles.is_empty() {
            store.install_defaults();
        }

        store
    }

    pub fn get_profiles(&self) -> Vec<Profile> {
        let mut list: Vec<Profile> = self.profiles.values().cloned().collect();
        list.sort_by(|a, b| a.name.cmp(&b.name));
        list
    }

    pub fn get_active_profile(&self) -> Option<Profile> {
        self.profiles.get(&self.active_profile_id).cloned()
    }

    pub fn set_active_profile(&mut self, id: &str) -> bool {
        if self.profiles.contains_key(id) {
            self.active_profile_id = id.to_string();
            true
        } else {
            false
        }
    }

    pub fn save_profile(&mut self, profile: Profile) -> Result<(), String> {
        let path = self.base_dir.join(format!("{}.json", profile.id));
        let content = serde_json::to_string_pretty(&profile).map_err(|e| e.to_string())?;
        fs::write(path, content).map_err(|e| e.to_string())?;
        self.profiles.insert(profile.id.clone(), profile);
        Ok(())
    }

    pub fn delete_profile(&mut self, id: &str) -> Result<(), String> {
        if id == "default" {
            return Err("Cannot delete default profile".to_string());
        }
        let path = self.base_dir.join(format!("{}.json", id));
        let _ = fs::remove_file(path);
        self.profiles.remove(id);
        if self.active_profile_id == id {
            self.active_profile_id = "default".to_string();
        }
        Ok(())
    }

    fn load_all(&mut self) {
        if let Ok(entries) = fs::read_dir(&self.base_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(profile) = serde_json::from_str::<Profile>(&content) {
                            self.profiles.insert(profile.id.clone(), profile);
                        }
                    }
                }
            }
        }
    }

    fn install_defaults(&mut self) {
        let default_profile = Profile {
            id: "default".to_string(),
            name: "Default Desktop".to_string(),
            description: "General navigation & media controls for Windows".to_string(),
            version: 1,
            app_rules: vec![],
            mappings: Mappings {
                buttons: {
                    let mut b = HashMap::new();
                    b.insert("A".to_string(), Action::Key { key: "Space".to_string(), modifiers: vec![], on: "press".to_string() });
                    b.insert("B".to_string(), Action::Key { key: "Escape".to_string(), modifiers: vec![], on: "press".to_string() });
                    b.insert("X".to_string(), Action::Key { key: "Enter".to_string(), modifiers: vec![], on: "press".to_string() });
                    b.insert("Y".to_string(), Action::Key { key: "Tab".to_string(), modifiers: vec![], on: "press".to_string() });
                    b.insert("LB".to_string(), Action::ComboKey { key: "Left".to_string(), modifiers: vec!["Alt".to_string()] });
                    b.insert("RB".to_string(), Action::ComboKey { key: "Right".to_string(), modifiers: vec!["Alt".to_string()] });
                    b.insert("DPadUp".to_string(), Action::Key { key: "Up".to_string(), modifiers: vec![], on: "press".to_string() });
                    b.insert("DPadDown".to_string(), Action::Key { key: "Down".to_string(), modifiers: vec![], on: "press".to_string() });
                    b.insert("DPadLeft".to_string(), Action::Key { key: "Left".to_string(), modifiers: vec![], on: "press".to_string() });
                    b.insert("DPadRight".to_string(), Action::Key { key: "Right".to_string(), modifiers: vec![], on: "press".to_string() });
                    b.insert("LeftThumb".to_string(), Action::Mouse { button: "left".to_string(), on: "hold".to_string() });
                    b.insert("RightThumb".to_string(), Action::Mouse { button: "right".to_string(), on: "click".to_string() });
                    b
                },
                triggers: {
                    let mut t = HashMap::new();
                    t.insert("LeftTrigger".to_string(), TriggerAction {
                        threshold: 0.4,
                        action: Action::Mouse { button: "left".to_string(), on: "hold".to_string() },
                    });
                    t.insert("RightTrigger".to_string(), TriggerAction {
                        threshold: 0.4,
                        action: Action::Mouse { button: "right".to_string(), on: "hold".to_string() },
                    });
                    t
                },
                sticks: {
                    let mut s = HashMap::new();
                    s.insert("LeftStick".to_string(), StickAction {
                        deadzone: 0.15,
                        mode: "mouse".to_string(),
                        sensitivity: 1.0,
                        drag_button: None,
                        up: None,
                        down: None,
                        left: None,
                        right: None,
                    });
                    s.insert("RightStick".to_string(), StickAction {
                        deadzone: 0.15,
                        mode: "scroll".to_string(),
                        sensitivity: 1.0,
                        drag_button: None,
                        up: None,
                        down: None,
                        left: None,
                        right: None,
                    });
                    s
                },
                combos: vec![],
            },
            deck: vec![
                DeckItem {
                    id: "task_view".to_string(),
                    label: "Task View".to_string(),
                    category: "Windows".to_string(),
                    shortcut: "Win+Tab".to_string(),
                    color: Some("#00e5ff".to_string()),
                    action: Action::ComboKey { key: "Tab".to_string(), modifiers: vec!["Win".to_string()] },
                },
                DeckItem {
                    id: "file_explorer".to_string(),
                    label: "File Explorer".to_string(),
                    category: "Windows".to_string(),
                    shortcut: "Win+E".to_string(),
                    color: None,
                    action: Action::ComboKey { key: "E".to_string(), modifiers: vec!["Win".to_string()] },
                },
                DeckItem {
                    id: "snip".to_string(),
                    label: "Screen Snip".to_string(),
                    category: "Tools".to_string(),
                    shortcut: "Win+Shift+S".to_string(),
                    color: None,
                    action: Action::ComboKey { key: "S".to_string(), modifiers: vec!["Win".to_string(), "Shift".to_string()] },
                },
                DeckItem {
                    id: "show_desktop".to_string(),
                    label: "Desktop".to_string(),
                    category: "Windows".to_string(),
                    shortcut: "Win+D".to_string(),
                    color: None,
                    action: Action::ComboKey { key: "D".to_string(), modifiers: vec!["Win".to_string()] },
                },
                DeckItem {
                    id: "undo".to_string(),
                    label: "Undo".to_string(),
                    category: "Actions".to_string(),
                    shortcut: "Ctrl+Z".to_string(),
                    color: None,
                    action: Action::ComboKey { key: "Z".to_string(), modifiers: vec!["Ctrl".to_string()] },
                },
                DeckItem {
                    id: "redo".to_string(),
                    label: "Redo".to_string(),
                    category: "Actions".to_string(),
                    shortcut: "Ctrl+Y".to_string(),
                    color: None,
                    action: Action::ComboKey { key: "Y".to_string(), modifiers: vec!["Ctrl".to_string()] },
                },
                DeckItem {
                    id: "copy".to_string(),
                    label: "Copy".to_string(),
                    category: "Clipboard".to_string(),
                    shortcut: "Ctrl+C".to_string(),
                    color: None,
                    action: Action::ComboKey { key: "C".to_string(), modifiers: vec!["Ctrl".to_string()] },
                },
                DeckItem {
                    id: "paste".to_string(),
                    label: "Paste".to_string(),
                    category: "Clipboard".to_string(),
                    shortcut: "Ctrl+V".to_string(),
                    color: None,
                    action: Action::ComboKey { key: "V".to_string(), modifiers: vec!["Ctrl".to_string()] },
                },
            ],
        };

        let wme_profile = Profile {
            id: "wme-editor".to_string(),
            name: "WME Editor".to_string(),
            description: "Shortcuts tailored for Waze Map Editor mapping workflows".to_string(),
            version: 1,
            app_rules: vec![
                AppRule {
                    match_type: "window_title".to_string(),
                    pattern: "Waze Map Editor".to_string(),
                    case_sensitive: false,
                },
                AppRule {
                    match_type: "window_title".to_string(),
                    pattern: "Google Maps".to_string(),
                    case_sensitive: false,
                },
            ],
            mappings: Mappings {
                buttons: {
                    let mut b = HashMap::new();
                    b.insert("A".to_string(), Action::Mouse { button: "left".to_string(), on: "click".to_string() }); // Select node / road
                    b.insert("B".to_string(), Action::Key { key: "Escape".to_string(), modifiers: vec![], on: "press".to_string() }); // Deselect
                    b.insert("X".to_string(), Action::Key { key: "S".to_string(), modifiers: vec![], on: "press".to_string() }); // Split segment
                    b.insert("Y".to_string(), Action::Key { key: "D".to_string(), modifiers: vec![], on: "press".to_string() }); // Delete node
                    b.insert("LB".to_string(), Action::ComboKey { key: "Z".to_string(), modifiers: vec!["Ctrl".to_string()] }); // Undo
                    b.insert("RB".to_string(), Action::ComboKey { key: "Z".to_string(), modifiers: vec!["Ctrl".to_string(), "Shift".to_string()] }); // Redo
                    b.insert("Start".to_string(), Action::ComboKey { key: "S".to_string(), modifiers: vec!["Ctrl".to_string()] }); // Save edits
                    b.insert("Back".to_string(), Action::Key { key: "Delete".to_string(), modifiers: vec![], on: "press".to_string() });
                    b.insert("LeftThumb".to_string(), Action::Mouse { button: "left".to_string(), on: "hold".to_string() }); // Click LeftStick to drag node
                    b.insert("RightThumb".to_string(), Action::Mouse { button: "right".to_string(), on: "click".to_string() });
                    b.insert("DPadUp".to_string(), Action::Key { key: "Up".to_string(), modifiers: vec![], on: "press".to_string() });
                    b.insert("DPadDown".to_string(), Action::Key { key: "Down".to_string(), modifiers: vec![], on: "press".to_string() });
                    b.insert("DPadLeft".to_string(), Action::Key { key: "Left".to_string(), modifiers: vec![], on: "press".to_string() });
                    b.insert("DPadRight".to_string(), Action::Key { key: "Right".to_string(), modifiers: vec![], on: "press".to_string() });
                    b
                },
                triggers: {
                    let mut t = HashMap::new();
                    t.insert("LeftTrigger".to_string(), TriggerAction {
                        threshold: 0.35,
                        action: Action::Mouse { button: "left".to_string(), on: "hold".to_string() }, // Squeeze LT to drag node/geometry
                    });
                    t.insert("RightTrigger".to_string(), TriggerAction {
                        threshold: 0.35,
                        action: Action::Mouse { button: "right".to_string(), on: "hold".to_string() }, // Squeeze RT to pan map
                    });
                    t
                },
                sticks: {
                    let mut s = HashMap::new();
                    s.insert("LeftStick".to_string(), StickAction {
                        deadzone: 0.15,
                        mode: "mouse".to_string(),
                        sensitivity: 1.25,
                        drag_button: None,
                        up: None,
                        down: None,
                        left: None,
                        right: None,
                    });
                    s.insert("RightStick".to_string(), StickAction {
                        deadzone: 0.15,
                        mode: "scroll".to_string(),
                        sensitivity: 1.0,
                        drag_button: None,
                        up: None,
                        down: None,
                        left: None,
                        right: None,
                    });
                    s
                },
                combos: vec![
                    ComboAction {
                        buttons: vec!["LB".to_string(), "A".to_string()],
                        action: Action::ComboKey {
                            key: "L".to_string(),
                            modifiers: vec!["Ctrl".to_string(), "Alt".to_string()],
                        },
                        window_ms: 120,
                    }
                ],
            },
            deck: vec![
                DeckItem {
                    id: "split_segment".to_string(),
                    label: "Split Segment".to_string(),
                    category: "Geometry".to_string(),
                    shortcut: "S".to_string(),
                    color: Some("#00e5ff".to_string()),
                    action: Action::Key { key: "S".to_string(), modifiers: vec![], on: "press".to_string() },
                },
                DeckItem {
                    id: "delete_node".to_string(),
                    label: "Delete Node".to_string(),
                    category: "Geometry".to_string(),
                    shortcut: "D / Del".to_string(),
                    color: Some("#ef4444".to_string()),
                    action: Action::Key { key: "D".to_string(), modifiers: vec![], on: "press".to_string() },
                },
                DeckItem {
                    id: "google_maps".to_string(),
                    label: "Google Maps".to_string(),
                    category: "Map Layers".to_string(),
                    shortcut: "Ctrl+Alt+L".to_string(),
                    color: None,
                    action: Action::ComboKey { key: "L".to_string(), modifiers: vec!["Ctrl".to_string(), "Alt".to_string()] },
                },
                DeckItem {
                    id: "osm".to_string(),
                    label: "OpenStreetMap".to_string(),
                    category: "Map Layers".to_string(),
                    shortcut: "Ctrl+Alt+F".to_string(),
                    color: None,
                    action: Action::ComboKey { key: "F".to_string(), modifiers: vec!["Ctrl".to_string(), "Alt".to_string()] },
                },
                DeckItem {
                    id: "undo".to_string(),
                    label: "Undo".to_string(),
                    category: "Actions".to_string(),
                    shortcut: "Ctrl+Z".to_string(),
                    color: None,
                    action: Action::ComboKey { key: "Z".to_string(), modifiers: vec!["Ctrl".to_string()] },
                },
                DeckItem {
                    id: "redo".to_string(),
                    label: "Redo".to_string(),
                    category: "Actions".to_string(),
                    shortcut: "Ctrl+Shift+Z".to_string(),
                    color: None,
                    action: Action::ComboKey { key: "Z".to_string(), modifiers: vec!["Ctrl".to_string(), "Shift".to_string()] },
                },
                DeckItem {
                    id: "save".to_string(),
                    label: "Save Changes".to_string(),
                    category: "Actions".to_string(),
                    shortcut: "Ctrl+S".to_string(),
                    color: Some("#22c55e".to_string()),
                    action: Action::ComboKey { key: "S".to_string(), modifiers: vec!["Ctrl".to_string()] },
                },
                DeckItem {
                    id: "escape".to_string(),
                    label: "Escape / Deselect".to_string(),
                    category: "Actions".to_string(),
                    shortcut: "Esc".to_string(),
                    color: None,
                    action: Action::Key { key: "Escape".to_string(), modifiers: vec![], on: "press".to_string() },
                },
            ],
        };

        let _ = self.save_profile(default_profile);
        let _ = self.save_profile(wme_profile);
    }
}

fn dirs_base() -> PathBuf {
    if let Ok(appdata) = std::env::var("APPDATA") {
        PathBuf::from(appdata).join("ControlDeck").join("profiles")
    } else {
        PathBuf::from("./profiles")
    }
}
