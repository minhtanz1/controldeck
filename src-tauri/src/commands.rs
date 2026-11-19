use crate::controller::{ControllerState, XInputPoller};
use crate::detector::{get_active_window as detect_window, get_open_windows as detect_open_windows, ActiveWindowInfo, OpenWindowInfo};
use crate::injection::{execute_action, release_action};
use crate::mapping::MappingEngine;
use crate::profile::{Action, Profile, ProfileStore};
use crate::server::MobileServer;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use tauri::State;

#[derive(Debug, Clone, Serialize)]
pub struct MobileServerInfo {
    pub port: u16,
    pub local_ip: String,
    pub url: String,
    pub connected_clients: usize,
}

pub struct AppState {
    pub store: Arc<Mutex<ProfileStore>>,
    pub poller: Mutex<XInputPoller>,
    pub sim_state: Arc<Mutex<ControllerState>>,
    pub engine: MappingEngine,
    pub server: MobileServer,
}

impl AppState {
    pub fn new() -> Self {
        let store = Arc::new(Mutex::new(ProfileStore::new()));
        let sim_state = Arc::new(Mutex::new(ControllerState::default()));
        let engine = MappingEngine::new(store.clone(), sim_state.clone());
        engine.start_background_loop();

        let server = MobileServer::new(4820);
        server.start(store.clone(), sim_state.clone());

        Self {
            store,
            poller: Mutex::new(XInputPoller::new(0)),
            sim_state,
            engine,
            server,
        }
    }
}

#[tauri::command]
pub fn get_profiles(state: State<AppState>) -> Vec<Profile> {
    let store = state.store.lock().unwrap();
    store.get_profiles()
}

#[tauri::command]
pub fn get_active_profile(state: State<AppState>) -> Option<Profile> {
    let store = state.store.lock().unwrap();
    store.get_active_profile()
}

#[tauri::command]
pub fn set_active_profile(id: String, state: State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap();
    if store.set_active_profile(&id) {
        Ok(())
    } else {
        Err(format!("Profile '{}' not found", id))
    }
}

#[tauri::command]
pub fn save_profile(profile: Profile, state: State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap();
    store.save_profile(profile)
}

#[tauri::command]
pub fn delete_profile(id: String, state: State<AppState>) -> Result<(), String> {
    let mut store = state.store.lock().unwrap();
    store.delete_profile(&id)
}

#[tauri::command]
pub fn get_controller_state(state: State<AppState>) -> ControllerState {
    let mut poller = state.poller.lock().unwrap();
    let real_state = poller.poll();
    if real_state.connected {
        return real_state;
    }

    let sim = state.sim_state.lock().unwrap();
    sim.clone()
}

#[tauri::command]
pub fn get_active_window() -> ActiveWindowInfo {
    detect_window()
}

#[tauri::command]
pub fn get_open_windows() -> Vec<OpenWindowInfo> {
    detect_open_windows()
}

#[tauri::command]
pub fn simulate_controller_button(button: String, pressed: bool, state: State<AppState>) -> ControllerState {
    let mut sim = state.sim_state.lock().unwrap();
    sim.connected = true;
    sim.buttons.insert(button.clone(), pressed);
    let st = sim.clone();
    drop(sim);

    // Also trigger key injection if mapped on active profile!
    let active_profile = {
        let store = state.store.lock().unwrap();
        store.get_active_profile()
    };

    if let Some(profile) = active_profile {
        if let Some(act) = profile.mappings.buttons.get(&button) {
            if pressed {
                execute_action(act);
            } else {
                release_action(act);
            }
        }
    }

    st
}

#[tauri::command]
pub fn test_action(action: Action) {
    execute_action(&action);
}

#[tauri::command]
pub fn set_mapping_enabled(enabled: bool, state: State<AppState>) {
    state.engine.set_enabled(enabled);
}

#[tauri::command]
pub fn is_mapping_enabled(state: State<AppState>) -> bool {
    state.engine.is_enabled()
}

#[tauri::command]
pub fn export_profile(id: String, state: State<AppState>) -> Result<String, String> {
    let store = state.store.lock().unwrap();
    let profiles = store.get_profiles();
    let p = profiles.iter().find(|x| x.id == id).ok_or("Profile not found")?;
    serde_json::to_string_pretty(p).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn import_profile(json_content: String, state: State<AppState>) -> Result<Profile, String> {
    let profile: Profile = serde_json::from_str(&json_content).map_err(|e| format!("Invalid profile format: {}", e))?;
    let mut store = state.store.lock().unwrap();
    store.save_profile(profile.clone())?;
    Ok(profile)
}

#[tauri::command]
pub fn get_mobile_server_info(state: State<AppState>) -> MobileServerInfo {
    let count = *state.server.connected_count.lock().unwrap();
    MobileServerInfo {
        port: state.server.port,
        local_ip: state.server.local_ip.clone(),
        url: state.server.get_connect_url(),
        connected_clients: count,
    }
}
