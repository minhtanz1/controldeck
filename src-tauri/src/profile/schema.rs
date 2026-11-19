use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: u32,
    pub app_rules: Vec<AppRule>,
    pub mappings: Mappings,
    #[serde(default)]
    pub deck: Vec<DeckItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeckItem {
    pub id: String,
    pub label: String,
    #[serde(default = "default_category")]
    pub category: String,
    #[serde(default)]
    pub shortcut: String,
    #[serde(default)]
    pub color: Option<String>,
    pub action: Action,
}

fn default_category() -> String {
    "Actions".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppRule {
    pub match_type: String, // "window_title" | "process_name"
    pub pattern: String,
    #[serde(default)]
    pub case_sensitive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Mappings {
    pub buttons: HashMap<String, Action>,
    #[serde(default)]
    pub triggers: HashMap<String, TriggerAction>,
    #[serde(default)]
    pub sticks: HashMap<String, StickAction>,
    #[serde(default)]
    pub combos: Vec<ComboAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Action {
    #[serde(rename = "key")]
    Key {
        key: String,
        #[serde(default)]
        modifiers: Vec<String>,
        #[serde(default = "default_on")]
        on: String, // "press" | "hold" | "release"
    },
    #[serde(rename = "combo_key")]
    ComboKey {
        key: String,
        modifiers: Vec<String>,
    },
    #[serde(rename = "toggle")]
    Toggle {
        key: String,
        #[serde(default)]
        modifiers: Vec<String>,
        #[serde(default)]
        state: bool,
    },
    #[serde(rename = "mouse")]
    Mouse {
        button: String, // "left" | "right" | "middle"
        #[serde(default = "default_on")]
        on: String, // "click" | "hold" | "release" | "toggle"
    },
    #[serde(rename = "macro")]
    Macro {
        sequence: Vec<MacroStep>,
        #[serde(default)]
        repeat: bool,
    },
}

fn default_on() -> String {
    "press".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroStep {
    pub key: String,
    #[serde(default)]
    pub modifiers: Vec<String>,
    #[serde(default = "default_delay")]
    pub delay_ms: u64,
}

fn default_delay() -> u64 {
    50
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerAction {
    #[serde(default = "default_threshold")]
    pub threshold: f32,
    pub action: Action,
}

fn default_threshold() -> f32 {
    0.5
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickAction {
    #[serde(default = "default_deadzone")]
    pub deadzone: f32,
    #[serde(default = "default_stick_mode")]
    pub mode: String, // "mouse" | "scroll" | "dpad"
    #[serde(default = "default_sensitivity")]
    pub sensitivity: f32,
    #[serde(default)]
    pub drag_button: Option<String>, // "none" | "left" | "right"
    #[serde(default)]
    pub up: Option<Action>,
    #[serde(default)]
    pub down: Option<Action>,
    #[serde(default)]
    pub left: Option<Action>,
    #[serde(default)]
    pub right: Option<Action>,
}

fn default_deadzone() -> f32 {
    0.15
}

fn default_stick_mode() -> String {
    "mouse".to_string()
}

fn default_sensitivity() -> f32 {
    1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComboAction {
    pub buttons: Vec<String>,
    pub action: Action,
    #[serde(default = "default_combo_window")]
    pub window_ms: u64,
}

fn default_combo_window() -> u64 {
    100
}
