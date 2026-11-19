use crate::controller::ControllerState;
use crate::injection::{
    execute_action, release_action, send_mouse_click, send_mouse_down, send_mouse_move,
    send_mouse_up, send_mouse_wheel, send_text_unicode, string_to_vk, tap_key,
};
use crate::profile::schema::Action;
use crate::profile::ProfileStore;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, UdpSocket};
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

const MOBILE_APP_HTML: &str = include_str!("dist-mobile/index.html");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileClientInfo {
    pub ip: String,
    pub connected: bool,
    pub last_seen: u64,
}

pub struct MobileServer {
    pub port: u16,
    pub local_ip: String,
    pub connected_count: Arc<Mutex<usize>>,
}

impl MobileServer {
    pub fn new(port: u16) -> Self {
        let local_ip = detect_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());
        Self {
            port,
            local_ip,
            connected_count: Arc::new(Mutex::new(0)),
        }
    }

    pub fn get_connect_url(&self) -> String {
        format!("http://{}:{}", self.local_ip, self.port)
    }

    pub fn start(
        &self,
        store: Arc<Mutex<ProfileStore>>,
        sim_state: Arc<Mutex<ControllerState>>,
    ) {
        let port = self.port;
        let connected_count = self.connected_count.clone();

        std::thread::spawn(move || {
            let rt = match tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
            {
                Ok(rt) => rt,
                Err(e) => {
                    eprintln!("Failed to create Tokio runtime for MobileServer: {}", e);
                    return;
                }
            };

            rt.block_on(async move {
                let app = Router::new()
                    .route("/", get(serve_index))
                    .route(
                        "/ws",
                        get(move |ws: WebSocketUpgrade| {
                            let st = store.clone();
                            let sim = sim_state.clone();
                            let cnt = connected_count.clone();
                            async move { ws.on_upgrade(move |socket| handle_socket(socket, st, sim, cnt)) }
                        }),
                    )
                    .layer(CorsLayer::permissive());

                let addr = SocketAddr::from(([0, 0, 0, 0], port));
                println!("Mobile Controller server listening on http://{}", addr);

                if let Ok(listener) = tokio::net::TcpListener::bind(addr).await {
                    let _ = axum::serve(listener, app).await;
                } else {
                    eprintln!("Failed to bind MobileServer to port {}", port);
                }
            });
        });
    }
}

async fn serve_index() -> impl IntoResponse {
    Html(MOBILE_APP_HTML)
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum WsClientMessage {
    #[serde(rename = "ping")]
    Ping,

    #[serde(rename = "btn_down")]
    BtnDown { btn: String },

    #[serde(rename = "btn_up")]
    BtnUp { btn: String },

    #[serde(rename = "axis")]
    Axis { axis: String, val: f32 },

    #[serde(rename = "stick_move")]
    StickMove { stick: String, x: f32, y: f32 },

    #[serde(rename = "trigger_move", alias = "trigger")]
    Trigger {
        trigger: String,
        #[serde(rename = "value", alias = "val")]
        val: f32,
    },

    #[serde(rename = "mouse_move")]
    MouseMove { dx: i32, dy: i32 },

    #[serde(rename = "mouse_click")]
    MouseClick { btn: String },

    #[serde(rename = "mouse_down")]
    MouseDown { btn: String },

    #[serde(rename = "mouse_up")]
    MouseUp { btn: String },

    #[serde(rename = "mouse_scroll")]
    MouseScroll { dy: i32 },

    #[serde(rename = "key_tap")]
    KeyTap { key: String },

    #[serde(rename = "key_combo")]
    KeyCombo { key: String, mods: Vec<String> },

    #[serde(rename = "text_input")]
    TextInput { text: String },

    #[serde(rename = "set_sensitivity")]
    SetSensitivity { val: f32 },

    #[serde(rename = "get_profile")]
    GetProfile,

    #[serde(rename = "switch_profile")]
    SwitchProfile { profile_id: String },

    #[serde(rename = "deck_action")]
    DeckAction { action: Action },
}

#[derive(Serialize)]
struct ProfileSyncMessage {
    r#type: &'static str,
    active_profile_id: String,
    active_profile_name: String,
    profiles: Vec<ProfileSummary>,
    deck: Vec<crate::profile::schema::DeckItem>,
}

#[derive(Serialize)]
struct ProfileSummary {
    id: String,
    name: String,
}

fn build_profile_sync(store: &ProfileStore) -> Option<String> {
    let active = store.get_active_profile()?;
    let summaries: Vec<ProfileSummary> = store
        .get_profiles()
        .iter()
        .map(|p| ProfileSummary {
            id: p.id.clone(),
            name: p.name.clone(),
        })
        .collect();

    let mut deck = active.deck.clone();
    if deck.is_empty() {
        for (btn, act) in &active.mappings.buttons {
            deck.push(crate::profile::schema::DeckItem {
                id: format!("btn_{}", btn),
                label: format!("Button {}", btn),
                category: "Controller Buttons".to_string(),
                shortcut: btn.clone(),
                color: None,
                action: act.clone(),
            });
        }
        for combo in &active.mappings.combos {
            deck.push(crate::profile::schema::DeckItem {
                id: format!("combo_{}", combo.buttons.join("_")),
                label: combo.buttons.join(" + "),
                category: "Combos".to_string(),
                shortcut: combo.buttons.join("+"),
                color: Some("#00e5ff".to_string()),
                action: combo.action.clone(),
            });
        }
    }

    let sync_msg = ProfileSyncMessage {
        r#type: "profile_sync",
        active_profile_id: active.id,
        active_profile_name: active.name,
        profiles: summaries,
        deck,
    };

    serde_json::to_string(&sync_msg).ok()
}

async fn handle_socket(
    mut socket: WebSocket,
    store: Arc<Mutex<ProfileStore>>,
    sim_state: Arc<Mutex<ControllerState>>,
    connected_count: Arc<Mutex<usize>>,
) {
    let mut last_synced_profile_id = String::new();

    {
        let mut cnt = connected_count.lock().unwrap();
        *cnt += 1;
    }

    // Send initial profile & dynamic deck sync upon connection
    let initial_sync = {
        let st = store.lock().unwrap();
        if let Some(sync_json) = build_profile_sync(&st) {
            if let Some(curr) = st.get_active_profile() {
                last_synced_profile_id = curr.id;
            }
            Some(sync_json)
        } else {
            None
        }
    };
    if let Some(json) = initial_sync {
        let _ = socket.send(Message::Text(json)).await;
    }

    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                if let Ok(cmd) = serde_json::from_str::<WsClientMessage>(&text) {
                    match cmd {
                        WsClientMessage::Ping => {
                            let _ = socket.send(Message::Text(r#"{"type":"pong"}"#.to_string())).await;

                            // Check if active profile changed on PC (e.g. via window focus auto-switch)
                            let (need_sync, sync_json) = {
                                let st = store.lock().unwrap();
                                if let Some(curr) = st.get_active_profile() {
                                    if curr.id != last_synced_profile_id {
                                        last_synced_profile_id = curr.id.clone();
                                        (true, build_profile_sync(&st))
                                    } else {
                                        (false, None)
                                    }
                                } else {
                                    (false, None)
                                }
                            };
                            if need_sync {
                                if let Some(json) = sync_json {
                                    let _ = socket.send(Message::Text(json)).await;
                                }
                            }
                        }

                        WsClientMessage::GetProfile => {
                            let sync_json = {
                                let st = store.lock().unwrap();
                                if let Some(json) = build_profile_sync(&st) {
                                    if let Some(curr) = st.get_active_profile() {
                                        last_synced_profile_id = curr.id;
                                    }
                                    Some(json)
                                } else {
                                    None
                                }
                            };
                            if let Some(json) = sync_json {
                                let _ = socket.send(Message::Text(json)).await;
                            }
                        }

                        WsClientMessage::SwitchProfile { profile_id } => {
                            let sync_json = {
                                let mut st = store.lock().unwrap();
                                st.set_active_profile(&profile_id);
                                last_synced_profile_id = profile_id;
                                build_profile_sync(&st)
                            };
                            if let Some(json) = sync_json {
                                let _ = socket.send(Message::Text(json)).await;
                            }
                        }

                        WsClientMessage::DeckAction { action } => {
                            execute_action(&action);
                        }

                        WsClientMessage::BtnDown { btn } => {
                            let mut sim = sim_state.lock().unwrap();
                            sim.connected = true;
                            sim.buttons.insert(btn.clone(), true);
                            drop(sim);

                            // Execute mapped key if assigned in active profile
                            let profile = {
                                let st = store.lock().unwrap();
                                st.get_active_profile()
                            };
                            if let Some(p) = profile {
                                if let Some(act) = p.mappings.buttons.get(&btn) {
                                    execute_action(act);
                                }
                            }
                        }

                        WsClientMessage::BtnUp { btn } => {
                            let mut sim = sim_state.lock().unwrap();
                            sim.buttons.insert(btn.clone(), false);
                            drop(sim);

                            let profile = {
                                let st = store.lock().unwrap();
                                st.get_active_profile()
                            };
                            if let Some(p) = profile {
                                if let Some(act) = p.mappings.buttons.get(&btn) {
                                    release_action(act);
                                }
                            }
                        }

                        WsClientMessage::Axis { axis, val } => {
                            let mut sim = sim_state.lock().unwrap();
                            sim.connected = true;
                            match axis.as_str() {
                                "lx" => sim.left_stick_x = val,
                                "ly" => sim.left_stick_y = val,
                                "rx" => sim.right_stick_x = val,
                                "ry" => sim.right_stick_y = val,
                                _ => {}
                            }
                        }

                        WsClientMessage::StickMove { stick, x, y } => {
                            let mut sim = sim_state.lock().unwrap();
                            apply_stick_move(&mut sim, &stick, x, y);
                        }

                        WsClientMessage::Trigger { trigger, val } => {
                            let mut sim = sim_state.lock().unwrap();
                            apply_trigger_move(&mut sim, &trigger, val);
                        }

                        WsClientMessage::MouseMove { dx, dy } => {
                            send_mouse_move(dx, dy);
                        }

                        WsClientMessage::MouseClick { btn } => {
                            send_mouse_click(&btn);
                        }

                        WsClientMessage::MouseDown { btn } => {
                            send_mouse_down(&btn);
                        }

                        WsClientMessage::MouseUp { btn } => {
                            send_mouse_up(&btn);
                        }

                        WsClientMessage::MouseScroll { dy } => {
                            send_mouse_wheel(dy);
                        }

                        WsClientMessage::KeyTap { key } => {
                            if let Some(vk) = string_to_vk(&key) {
                                tap_key(vk);
                            }
                        }

                        WsClientMessage::KeyCombo { key, mods } => {
                            execute_action(&Action::ComboKey {
                                key,
                                modifiers: mods,
                            });
                        }

                        WsClientMessage::TextInput { text } => {
                            send_text_unicode(&text);
                        }

                        WsClientMessage::SetSensitivity { val } => {
                            let mut st = store.lock().unwrap();
                            if let Some(mut profile) = st.get_active_profile() {
                                if let Some(ls) = profile.mappings.sticks.get_mut("LeftStick") {
                                    ls.sensitivity = val;
                                }
                                let _ = st.save_profile(profile);
                            }
                        }
                    }
                }
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    {
        let mut cnt = connected_count.lock().unwrap();
        if *cnt > 0 {
            *cnt -= 1;
        }
    }
}

fn apply_stick_move(sim: &mut ControllerState, stick: &str, x: f32, y: f32) {
    match stick {
        "left" => {
            sim.left_stick_x = x;
            sim.left_stick_y = y;
        }
        "right" => {
            sim.right_stick_x = x;
            sim.right_stick_y = y;
        }
        _ => return,
    }
    sim.connected = true;
}

fn apply_trigger_move(sim: &mut ControllerState, trigger: &str, value: f32) {
    let target = match trigger {
        "lt" => &mut sim.left_trigger,
        "rt" => &mut sim.right_trigger,
        _ => return,
    };
    *target = value;
    sim.connected = true;
}

pub fn detect_local_ip() -> Option<String> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    socket.connect("8.8.8.8:80").ok()?;
    Some(socket.local_addr().ok()?.ip().to_string())
}

#[cfg(test)]
mod tests {
    use super::{apply_stick_move, apply_trigger_move, WsClientMessage};
    use crate::controller::ControllerState;

    #[test]
    fn parses_mobile_trigger_move_frames() {
        let mut state = ControllerState::default();
        for frame in [
            r#"{"type":"trigger_move","trigger":"lt","value":0.65}"#,
            r#"{"type":"trigger_move","trigger":"rt","value":0.4}"#,
            r#"{"type":"trigger","trigger":"lt","val":0.2}"#,
        ] {
            let message = serde_json::from_str::<WsClientMessage>(frame)
                .expect("server must accept PocketPad trigger movement");
            let WsClientMessage::Trigger { trigger, val } = message else {
                panic!("expected a trigger movement message");
            };
            apply_trigger_move(&mut state, &trigger, val);
        }

        assert!(state.connected);
        assert_eq!(state.left_trigger, 0.2);
        assert_eq!(state.right_trigger, 0.4);
        apply_trigger_move(&mut state, "lt", 0.0);
        apply_trigger_move(&mut state, "rt", 0.0);
        assert_eq!(state.left_trigger, 0.0);
        assert_eq!(state.right_trigger, 0.0);
    }

    #[test]
    fn applies_mobile_stick_move_messages_to_the_matching_stick() {
        let mut state = ControllerState::default();
        for (stick, x, y) in [("left", 0.5, -0.25), ("right", -0.75, 0.4)] {
            let frame = format!(
                r#"{{"type":"stick_move","stick":"{}","x":{},"y":{}}}"#,
                stick, x, y
            );
            let message = serde_json::from_str::<WsClientMessage>(&frame)
                .expect("server must accept PocketPad stick movement");
            let WsClientMessage::StickMove { stick, x, y } = message else {
                panic!("expected a stick movement message");
            };
            apply_stick_move(&mut state, &stick, x, y);
        }

        assert!(state.connected);
        assert_eq!(state.left_stick_x, 0.5);
        assert_eq!(state.left_stick_y, -0.25);
        assert_eq!(state.right_stick_x, -0.75);
        assert_eq!(state.right_stick_y, 0.4);

        apply_stick_move(&mut state, "left", 0.0, 0.0);
        assert_eq!(state.left_stick_x, 0.0);
        assert_eq!(state.left_stick_y, 0.0);
    }
}
