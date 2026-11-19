use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use windows::core::BOOL;
use windows::Win32::Foundation::{CloseHandle, HWND, LPARAM};
use windows::Win32::System::ProcessStatus::K32GetProcessImageFileNameW;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId,
    IsWindowVisible,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActiveWindowInfo {
    pub title: String,
    pub process_name: String,
    pub hwnd: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenWindowInfo {
    pub hwnd: usize,
    pub title: String,
    pub process_name: String,
    pub is_browser: bool,
    pub clean_title: String,
}

pub fn get_active_window() -> ActiveWindowInfo {
    unsafe {
        let hwnd: HWND = GetForegroundWindow();
        if hwnd.0 == 0 as _ {
            return ActiveWindowInfo::default();
        }

        let mut title_buf = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut title_buf);
        let title = if len > 0 {
            String::from_utf16_lossy(&title_buf[..len as usize])
        } else {
            String::new()
        };

        let mut process_id = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut process_id));

        let mut process_name = String::new();
        if process_id != 0 {
            if let Ok(process_handle) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id) {
                let mut path_buf = [0u16; 1024];
                let path_len = K32GetProcessImageFileNameW(process_handle, &mut path_buf);
                if path_len > 0 {
                    let full_path = String::from_utf16_lossy(&path_buf[..path_len as usize]);
                    if let Some(filename) = full_path.split('\\').last() {
                        process_name = filename.to_string();
                    }
                }
                let _ = CloseHandle(process_handle);
            }
        }

        ActiveWindowInfo {
            title,
            process_name,
            hwnd: hwnd.0 as usize,
        }
    }
}

static ENUMERATED_WINDOWS: Mutex<Vec<OpenWindowInfo>> = Mutex::new(Vec::new());

unsafe extern "system" fn enum_windows_callback(hwnd: HWND, _lparam: LPARAM) -> BOOL {
    unsafe {
        if !IsWindowVisible(hwnd).as_bool() {
            return BOOL(1);
        }

        let text_len = GetWindowTextLengthW(hwnd);
        if text_len <= 0 {
            return BOOL(1);
        }

        let mut buf = vec![0u16; (text_len + 1) as usize];
        let actual_len = GetWindowTextW(hwnd, &mut buf);
        if actual_len <= 0 {
            return BOOL(1);
        }

        let title = String::from_utf16_lossy(&buf[..actual_len as usize]).trim().to_string();
        if title.is_empty()
            || title == "Program Manager"
            || title == "Settings"
            || title == "Windows Input Experience"
        {
            return BOOL(1);
        }

        let mut process_id = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut process_id));

        let mut process_name = String::new();
        if process_id != 0 {
            if let Ok(process_handle) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, process_id) {
                let mut path_buf = [0u16; 1024];
                let path_len = K32GetProcessImageFileNameW(process_handle, &mut path_buf);
                if path_len > 0 {
                    let full_path = String::from_utf16_lossy(&path_buf[..path_len as usize]);
                    if let Some(filename) = full_path.split('\\').last() {
                        process_name = filename.to_string();
                    }
                }
                let _ = CloseHandle(process_handle);
            }
        }

        let (is_browser, clean_title) = extract_clean_title(&title, &process_name);

        if let Ok(mut list) = ENUMERATED_WINDOWS.lock() {
            list.push(OpenWindowInfo {
                hwnd: hwnd.0 as usize,
                title,
                process_name,
                is_browser,
                clean_title,
            });
        }

        BOOL(1)
    }
}

pub fn get_open_windows() -> Vec<OpenWindowInfo> {
    if let Ok(mut list) = ENUMERATED_WINDOWS.lock() {
        list.clear();
    }

    unsafe {
        let _ = EnumWindows(Some(enum_windows_callback), LPARAM(0));
    }

    let mut result = if let Ok(list) = ENUMERATED_WINDOWS.lock() {
        list.clone()
    } else {
        Vec::new()
    };

    // Sort browsers first, then alphabetically
    result.sort_by(|a, b| {
        b.is_browser.cmp(&a.is_browser).then_with(|| a.clean_title.cmp(&b.clean_title))
    });

    result
}

fn extract_clean_title(title: &str, proc: &str) -> (bool, String) {
    let p_lower = proc.to_lowercase();
    let is_browser = p_lower.contains("chrome")
        || p_lower.contains("edge")
        || p_lower.contains("firefox")
        || p_lower.contains("brave")
        || p_lower.contains("opera")
        || p_lower.contains("vivaldi")
        || p_lower.contains("arc");

    let suffixes = [
        " - Google Chrome",
        " - Microsoft​ Edge",
        " - Microsoft Edge",
        " — Mozilla Firefox",
        " - Mozilla Firefox",
        " - Brave",
        " - Opera",
        " - Vivaldi",
        " - Arc",
    ];

    let mut clean = title.to_string();
    for suffix in suffixes {
        if clean.ends_with(suffix) {
            clean = clean[..clean.len() - suffix.len()].trim().to_string();
            break;
        }
    }

    (is_browser, clean)
}
