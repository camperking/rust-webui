/*
  WebUI Library 2.2.0
  http://_webui_core.me
  https://github.com/alifcommunity/webui
  Copyright (c) 2020-2023 Hassan Draga.
  Licensed under GNU General Public License v2.0.
  All rights reserved.
  Canada.
*/

// Flags
// #![allow(unsafe_code)]
// #![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

pub mod bindgen;

// Modules
use std::collections::HashMap;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use std::sync::LazyLock;
use std::sync::Mutex;

use bindgen::*;

// Browsers
pub type WebUIBrowser = webui_browser;

// Runtimes
pub type WebUIRuntime = webui_runtime;

// Events
pub type WebUIEventType = webui_event;

// Implement into<usize>
impl WebUIEventType {
    pub fn from_usize(value: usize) -> WebUIEventType {
        match value {
            0 => WebUIEventType::WEBUI_EVENT_DISCONNECTED,
            1 => WebUIEventType::WEBUI_EVENT_CONNECTED,
            2 => WebUIEventType::WEBUI_EVENT_MOUSE_CLICK,
            3 => WebUIEventType::WEBUI_EVENT_NAVIGATION,
            4 => WebUIEventType::WEBUI_EVENT_CALLBACK,
            _ => WebUIEventType::WEBUI_EVENT_CALLBACK,
        }
    }
}

// Configs
pub type WebUIConfig = webui_config;

pub struct WebUIEventSimple {
    pub win: usize,
    pub event_type: WebUIEventType,
    pub element: String,
    pub event_number: usize,
    pub bind_id: usize,
}

impl WebUIEventSimple {
    pub fn set_response(&self, response: &str) {
        // interface_set_response(self.window, self.event_number, response);
        let response_c_str = CString::new(response).unwrap();
        let response_c_char: *const c_char = response_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_interface_set_response(self.win, self.event_number, response_c_char);
        }
    }

    pub fn get_string_at(&self, index: usize) -> String {
        unsafe {
            let string = webui_interface_get_string_at(self.win, self.event_number, index);
            char_to_string(string)
        }
    }

    pub fn get_int_at(&self, index: usize) -> i64 {
        unsafe { webui_interface_get_int_at(self.win, self.event_number, index) }
    }

    pub fn get_float_at(&self, index: usize) -> f64 {
        unsafe { webui_interface_get_float_at(self.win, self.event_number, index) }
    }

    pub fn get_bool_at(&self, index: usize) -> bool {
        unsafe { webui_interface_get_bool_at(self.win, self.event_number, index) }
    }

    pub fn get_size_at(&self, index: usize) -> usize {
        unsafe { webui_interface_get_size_at(self.win, self.event_number, index) }
    }
}

pub struct WebUIEvent {
    pub win: usize,
    pub event_type: WebUIEventType,
    pub element: String,
    pub event_number: usize,
    pub bind_id: usize,
    pub client_id: usize,
    pub connection_id: usize,
    pub cookies: String,
    event: *mut webui_event_t,
}

impl WebUIEvent {
    pub fn show_client(&self, content: impl AsRef<str> + Into<Vec<u8>>) -> bool {
        unsafe {
            // Content String to i8/u8
            let content_c_str = CString::new(content).unwrap();
            let content_c_char: *const c_char = content_c_str.as_ptr() as *const c_char;

            webui_show_client(self.event, content_c_char)
        }
    }

    pub fn close_client(self) {
        unsafe {
            webui_close_client(self.event);
        }
    }

    pub fn send_raw(&self, function: &str, data: &[u8]) {
        let size = data.len();
        let raw = data.as_ptr() as *mut std::os::raw::c_void;
        let function_c_str = CString::new(function).unwrap();
        let function_c_char: *const c_char = function_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_send_raw_client(self.event, function_c_char, raw, size);
        }
    }

    pub fn navigate_client(&self, url: &str) {
        let url_c_str = CString::new(url).unwrap();
        let url_c_char: *const c_char = url_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_navigate_client(self.event, url_c_char);
        }
    }

    pub fn run(&self, script: &str) {
        let script_c_str = CString::new(script).unwrap();
        let script_c_char: *const c_char = script_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_run_client(self.event, script_c_char);
        }
    }

    pub fn script(&self, script: &str, timeout: usize, buffer_length: usize) -> Result<String, ()> {
        let script_c_str = CString::new(script).unwrap();
        let script_c_char: *const c_char = script_c_str.as_ptr() as *const c_char;

        let buffer_c_str = CString::new(vec![0; buffer_length]).unwrap();
        let buffer_c_char: *mut c_char = buffer_c_str.as_ptr() as *mut c_char;

        unsafe {
            match webui_script_client(
                self.event,
                script_c_char,
                timeout,
                buffer_c_char,
                buffer_length,
            ) {
                true => Ok(char_to_string(buffer_c_char)),
                false => Err(()),
            }
        }
    }

    pub fn get_count(&self) -> usize {
        unsafe { webui_get_count(self.event) }
    }

    pub fn get_int_at(&self, index: usize) -> i64 {
        unsafe { webui_get_int_at(self.event, index) }
    }

    pub fn get_int(&self) -> i64 {
        unsafe { webui_get_int(self.event) }
    }

    pub fn get_float_at(&self, index: usize) -> f64 {
        unsafe { webui_get_float_at(self.event, index) }
    }

    pub fn get_float(&self) -> f64 {
        unsafe { webui_get_float(self.event) }
    }

    pub fn get_string_at(&self, index: usize) -> String {
        unsafe {
            let string = webui_get_string_at(self.event, index);
            char_to_string(string)
        }
    }

    pub fn get_string(&self) -> String {
        unsafe {
            let string = webui_get_string(self.event);
            char_to_string(string)
        }
    }

    pub fn get_bool_at(&self, index: usize) -> bool {
        unsafe { webui_get_bool_at(self.event, index) }
    }

    pub fn get_bool(&self) -> bool {
        unsafe { webui_get_bool(self.event) }
    }

    pub fn get_size_at(&self, index: usize) -> usize {
        unsafe { webui_get_size_at(self.event, index) }
    }

    pub fn get_size(&self) -> usize {
        unsafe { webui_get_size(self.event) }
    }

    pub fn return_int(&self, value: i64) {
        unsafe {
            webui_return_int(self.event, value);
        }
    }

    pub fn return_float(&self, value: f64) {
        unsafe {
            webui_return_float(self.event, value);
        }
    }

    pub fn return_string(&self, value: &str) {
        let value_c_str = CString::new(value).unwrap();
        let value_c_char: *const c_char = value_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_return_string(self.event, value_c_char);
        }
    }

    pub fn return_bool(&self, value: bool) {
        unsafe {
            webui_return_bool(self.event, value);
        }
    }
}

pub struct WebUIWindow {
    pub id: usize,
}

impl WebUIWindow {
    pub fn new() -> WebUIWindow {
        let id = new_window();
        WebUIWindow { id }
    }

    pub fn from_id(id: usize) -> WebUIWindow {
        WebUIWindow { id }
    }

    pub fn bind(&self, element: &str, func: fn(WebUIEvent)) -> usize {
        // Element String to i8/u8
        let element_c_str = CString::new(element).unwrap();
        let element_c_char: *const c_char = element_c_str.as_ptr() as *const c_char;

        // Bind
        unsafe {
            let f: Option<unsafe extern "C" fn(*mut webui_event_t)> = Some(bind_events_handler);

            let window_id = webui_interface_get_window_id(self.id);

            // Add the Rust user function to the list
            BIND_STORE.add_function(window_id, element, func);

            webui_bind(self.id, element_c_char, f)
        }
    }

    pub fn get_best_browser(&self) -> WebUIBrowser {
        unsafe {
            match webui_get_best_browser(self.id) {
                0 => WebUIBrowser::NoBrowser,
                1 => WebUIBrowser::AnyBrowser,
                2 => WebUIBrowser::Chrome,
                3 => WebUIBrowser::Firefox,
                4 => WebUIBrowser::Edge,
                5 => WebUIBrowser::Safari,
                6 => WebUIBrowser::Chromium,
                7 => WebUIBrowser::Opera,
                8 => WebUIBrowser::Brave,
                9 => WebUIBrowser::Vivaldi,
                10 => WebUIBrowser::Epic,
                11 => WebUIBrowser::Yandex,
                12 => WebUIBrowser::ChromiumBased,
                13 => WebUIBrowser::Webview,
                _ => WebUIBrowser::NoBrowser,
            }
        }
    }

    pub fn show(&self, content: &str) -> bool {
        unsafe {
            // Content String to i8/u8
            let content_c_str = CString::new(content).unwrap();
            let content_c_char: *const c_char = content_c_str.as_ptr() as *const c_char;

            webui_show(self.id, content_c_char)
        }
    }

    pub fn show_browser(&self, content: &str, browser: WebUIBrowser) -> bool {
        let content_c_str = CString::new(content).unwrap();
        let content_c_char: *const c_char = content_c_str.as_ptr() as *const c_char;

        unsafe { webui_show_browser(self.id, content_c_char, browser as usize) }
    }

    pub fn start_server(&self, content: &str) -> String {
        let content_c_str = CString::new(content).unwrap();
        let content_c_char: *const c_char = content_c_str.as_ptr() as *const c_char;

        unsafe {
            let server = webui_start_server(self.id, content_c_char);
            char_to_string(server)
        }
    }

    pub fn show_wv(&self, content: &str) -> bool {
        let content_c_str = CString::new(content).unwrap();
        let content_c_char: *const c_char = content_c_str.as_ptr() as *const c_char;

        unsafe { webui_show_wv(self.id, content_c_char) }
    }

    pub fn set_kiosk(&self, status: bool) {
        unsafe {
            webui_set_kiosk(self.id, status);
        }
    }

    pub fn set_high_contrast(&self, status: bool) {
        unsafe {
            webui_set_high_contrast(self.id, status);
        }
    }

    pub fn close(self) {
        unsafe {
            webui_close(self.id);
        }
    }

    pub fn destroy(&self) {
        unsafe {
            webui_destroy(self.id);
        }
    }

    pub fn set_root_folder(&self, folder: &str) {
        let folder_c_str = CString::new(folder).unwrap();
        let folder_c_char: *const c_char = folder_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_set_root_folder(self.id, folder_c_char);
        }
    }

    pub fn set_file_handler(
        &self,
        handler: unsafe extern "C" fn(*const i8, *mut i32) -> *const std::os::raw::c_void,
    ) {
        unsafe {
            webui_set_file_handler(self.id, Some(handler));
        }
    }

    pub fn is_shown(&self) -> bool {
        unsafe { webui_is_shown(self.id) }
    }

    pub fn set_icon(&self, icon: &str, kind: &str) {
        let icon_c_str = CString::new(icon).unwrap();
        let kind_c_str = CString::new(kind).unwrap();
        let icon_c_char: *const c_char = icon_c_str.as_ptr() as *const c_char;
        let kind_c_char: *const c_char = kind_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_set_icon(self.id, icon_c_char, kind_c_char);
        }
    }

    pub fn send_raw(&self, function: &str, data: &[u8]) {
        let size = data.len();
        let raw = data.as_ptr() as *mut std::os::raw::c_void;
        let function_c_str = CString::new(function).unwrap();
        let function_c_char: *const c_char = function_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_send_raw(self.id, function_c_char, raw, size);
        }
    }

    pub fn set_hide(&self, status: bool) {
        unsafe {
            webui_set_hide(self.id, status);
        }
    }

    pub fn set_size(&self, width: u32, height: u32) {
        unsafe {
            webui_set_size(self.id, width, height);
        }
    }

    pub fn set_position(&self, x: u32, y: u32) {
        unsafe {
            webui_set_position(self.id, x, y);
        }
    }

    pub fn set_profile(&self, name: &str, path: &str) {
        let name_c_str = CString::new(name).unwrap();
        let path_c_str = CString::new(path).unwrap();
        let name_c_char: *const c_char = name_c_str.as_ptr() as *const c_char;
        let path_c_char: *const c_char = path_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_set_profile(self.id, name_c_char, path_c_char);
        }
    }

    pub fn set_proxy(&self, proxy: &str) {
        let proxy_c_str = CString::new(proxy).unwrap();
        let proxy_c_char: *const c_char = proxy_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_set_proxy(self.id, proxy_c_char);
        }
    }

    pub fn get_url(&self) -> String {
        unsafe {
            let url = webui_get_url(self.id);
            char_to_string(url)
        }
    }

    pub fn set_public(&self, status: bool) {
        unsafe {
            webui_set_public(self.id, status);
        }
    }

    pub fn navigate(&self, url: &str) {
        let url_c_str = CString::new(url).unwrap();
        let url_c_char: *const c_char = url_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_navigate(self.id, url_c_char);
        }
    }

    pub fn delete_profile(&self) {
        unsafe {
            webui_delete_profile(self.id);
        }
    }

    pub fn get_parent_process_id(&self) -> usize {
        unsafe { webui_get_parent_process_id(self.id) }
    }

    pub fn get_child_process_id(&self) -> usize {
        unsafe { webui_get_child_process_id(self.id) }
    }

    pub fn get_port(&self) -> usize {
        unsafe { webui_get_port(self.id) }
    }

    pub fn set_port(&self, port: usize) -> bool {
        unsafe { webui_set_port(self.id, port) }
    }

    pub fn set_event_blocking(&self, status: bool) {
        unsafe {
            webui_set_event_blocking(self.id, status);
        }
    }

    pub fn run(&self, script: &str) {
        let script_c_str = CString::new(script).unwrap();
        let script_c_char: *const c_char = script_c_str.as_ptr() as *const c_char;

        unsafe {
            webui_run(self.id, script_c_char);
        }
    }

    pub fn script(&self, script: &str, timeout: usize, buffer_length: usize) -> Result<String, ()> {
        let script_c_str = CString::new(script).unwrap();
        let script_c_char: *const c_char = script_c_str.as_ptr() as *const c_char;

        let buffer_c_str = CString::new(vec![0; buffer_length]).unwrap();
        let buffer_c_char: *mut c_char = buffer_c_str.as_ptr() as *mut c_char;

        unsafe {
            match webui_script(
                self.id,
                script_c_char,
                timeout,
                buffer_c_char,
                buffer_length,
            ) {
                true => Ok(char_to_string(buffer_c_char)),
                false => Err(()),
            }
        }
    }

    pub fn set_runtime(&self, runtime: WebUIRuntime) {
        unsafe {
            webui_set_runtime(self.id, runtime as usize);
        }
    }
}

impl Drop for WebUIWindow {
    fn drop(&mut self) {
        self.destroy();
    }
}

const WINDOWS: usize = 64;
const ELEMENTS: usize = 64;

struct BindStore<T> {
    func_store: Mutex<[[Option<T>; ELEMENTS]; WINDOWS]>,
    elements_map: Mutex<HashMap<String, usize>>,
}

impl<T: Copy> BindStore<T> {
    fn new() -> BindStore<T> {
        BindStore {
            func_store: Mutex::new([[None; ELEMENTS]; WINDOWS]),
            elements_map: Mutex::new(HashMap::new()),
        }
    }

    fn save_string(&self, s: &str) -> usize {
        let mut map = self.elements_map.lock().unwrap();
        // Check if the string already exists in the map
        if let Some(&index) = map.get(s) {
            return index;
        }

        // If the string does not exist, add it to the map and return the new index
        let index = map.len();
        map.insert(s.to_owned(), index);
        index
    }

    fn find_string(&self, s: &str) -> isize {
        let map = self.elements_map.lock().unwrap();
        if let Some(&index) = map.get(s) {
            index as isize
        } else {
            -1
        }
    }

    fn add_function(&self, window: usize, element: &str, func: T) {
        let element_index = self.save_string(element);
        self.func_store.lock().unwrap()[window][element_index] = Some(func);
    }

    fn get_function(&self, window: usize, element: &str) -> Option<T> {
        let element_index = self.find_string(element);
        if element_index < 0 {
            return None;
        }
        self.func_store.lock().unwrap()[window][element_index as usize]
    }
}

static mut BIND_STORE_SIMPLE: LazyLock<BindStore<fn(WebUIEventSimple)>> =
    LazyLock::new(|| BindStore::new());

static mut BIND_STORE: LazyLock<BindStore<fn(WebUIEvent)>> = LazyLock::new(|| BindStore::new());

fn char_to_string(c: *const i8) -> String {
    let cstr = unsafe { CStr::from_ptr(c) };
    let s: String = String::from_utf8_lossy(cstr.to_bytes()).to_string();
    s
}

// fn cstr_to_string(c: CString) -> String {
//     let s: String = String::from_utf8_lossy(c.to_bytes()).to_string();
//     s
// }

// Function Implementations

pub fn new_window() -> usize {
    unsafe {
        // GLOBAL_ARRAY = [[GlobalArray::None; COLS]; ROWS];
        webui_new_window()
    }
}

pub fn new_window_id(id: usize) -> usize {
    unsafe {
        // GLOBAL_ARRAY = [[GlobalArray::None; COLS]; ROWS];
        webui_new_window_id(id)
    }
}

pub fn get_new_window_id() -> usize {
    unsafe { webui_get_new_window_id() }
}

unsafe extern "C" fn bind_events_handler(event: *mut webui_event_t) {
    let evt = WebUIEvent {
        win: (*event).window,
        event_type: WebUIEventType::from_usize((*event).event_type),
        element: char_to_string((*event).element),
        event_number: (*event).event_number,
        bind_id: (*event).bind_id,
        client_id: (*event).client_id,
        connection_id: (*event).connection_id,
        cookies: char_to_string((*event).cookies),
        event,
    };

    // Call the Rust user function
    unsafe {
        let window_id = webui_interface_get_window_id((*event).window);

        if let Some(func) = BIND_STORE.get_function(window_id, &evt.element) {
            func(evt);
        }
    }
}

pub fn is_high_contrast() -> bool {
    unsafe { webui_is_high_contrast() }
}

pub fn browser_exist(browser: WebUIBrowser) {
    unsafe {
        webui_browser_exist(browser as usize);
    }
}

pub fn wait() {
    unsafe {
        webui_wait();
    }
}

pub fn exit() {
    unsafe {
        webui_exit();
    }
}

pub fn set_default_root_folder(folder: &str) {
    let folder_c_str = CString::new(folder).unwrap();
    let folder_c_char: *const c_char = folder_c_str.as_ptr() as *const c_char;

    unsafe {
        webui_set_default_root_folder(folder_c_char);
    }
}

pub fn set_timeout(seconds: usize) {
    unsafe {
        webui_set_timeout(seconds);
    }
}

pub fn encode(data: &str) -> String {
    let data_c_str = CString::new(data).unwrap();
    let data_c_char: *const c_char = data_c_str.as_ptr() as *const c_char;

    unsafe {
        let encoded = webui_encode(data_c_char);
        char_to_string(encoded)
    }
}

pub fn decode(data: &str) -> String {
    let data_c_str = CString::new(data).unwrap();
    let data_c_char: *const c_char = data_c_str.as_ptr() as *const c_char;

    unsafe {
        let decoded = webui_decode(data_c_char);
        char_to_string(decoded)
    }
}

pub fn free(data: *mut std::os::raw::c_void) {
    unsafe {
        webui_free(data);
    }
}

pub fn malloc(size: usize) -> *mut std::os::raw::c_void {
    unsafe { webui_malloc(size) }
}

pub fn open_url(url: &str) {
    let url_c_str = CString::new(url).unwrap();
    let url_c_char: *const c_char = url_c_str.as_ptr() as *const c_char;

    unsafe {
        webui_open_url(url_c_char);
    }
}

pub fn clean() {
    unsafe {
        webui_clean();
    }
}

pub fn delete_all_profiles() {
    unsafe {
        webui_delete_all_profiles();
    }
}

pub fn get_free_port() -> usize {
    unsafe { webui_get_free_port() }
}

pub fn set_config(option: WebUIConfig, enabled: bool) {
    unsafe {
        webui_set_config(option as webui_config, enabled);
    }
}

pub fn get_mime_type(file: &str) -> String {
    let file_c_str = CString::new(file).unwrap();
    let file_c_char: *const c_char = file_c_str.as_ptr() as *const c_char;

    unsafe {
        let mime = webui_get_mime_type(file_c_char);
        char_to_string(mime)
    }
}

pub fn set_tls_certificate(cert_pem: &str, key_pem: &str) -> bool {
    let cert_pem_c_str = CString::new(cert_pem).unwrap();
    let key_pem_c_str = CString::new(key_pem).unwrap();
    let cert_pem_c_char: *const c_char = cert_pem_c_str.as_ptr() as *const c_char;
    let key_pem_c_char: *const c_char = key_pem_c_str.as_ptr() as *const c_char;

    unsafe { webui_set_tls_certificate(cert_pem_c_char, key_pem_c_char) }
}

unsafe extern "C" fn events_handler(
    window: usize,
    event_type: usize,
    element: *mut ::std::os::raw::c_char,
    event_number: usize,
    bind_id: usize,
) {
    // Call the Rust user function
    unsafe {
        let window_id = webui_interface_get_window_id(window);

        if let Some(func) = BIND_STORE_SIMPLE.get_function(window_id, &char_to_string(element)) {
            let evt = WebUIEventSimple {
                win: window,
                event_type: WebUIEventType::from_usize(event_type),
                element: char_to_string(element),
                event_number,
                bind_id,
            };

            func(evt);
        }
    }
}

pub fn interface_bind(win: usize, element: &str, func: fn(WebUIEventSimple)) -> usize {
    // Element String to i8/u8
    let element_c_str = CString::new(element).unwrap();
    let element_c_char: *const c_char = element_c_str.as_ptr() as *const c_char;

    // Bind
    unsafe {
        let f: Option<
            unsafe extern "C" fn(usize, usize, *mut ::std::os::raw::c_char, usize, usize),
        > = Some(events_handler);

        let window_id = webui_interface_get_window_id(win);

        // Add the Rust user function to the list
        BIND_STORE_SIMPLE.add_function(window_id, element, func);

        webui_interface_bind(win, element_c_char, f)
    }
}

pub fn interface_is_app_running() -> bool {
    unsafe { webui_interface_is_app_running() }
}

pub fn interface_get_window_id(win: usize) -> usize {
    unsafe { webui_interface_get_window_id(win) }
}
