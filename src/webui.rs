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
use std::sync::{Mutex, MutexGuard};

use bindgen::*;

// Consts
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub type size_t = ::std::os::raw::c_ulong;
pub type wchar_t = ::std::os::raw::c_int;

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

#[derive(Debug)]
pub struct JavaScript {
    pub timeout: usize,
    pub script: String,
    pub error: bool,
    pub data: String,
}

pub struct WebUIEvent {
    pub window: usize,
    pub event_type: WebUIEventType,
    pub element: String,
    pub event_number: usize,
    pub bind_id: usize,
    //
    pub client_id: Option<usize>,
    pub connection_id: Option<usize>,
    pub cookies: Option<String>,
    e: Option<*mut webui_event_t>,
}

impl WebUIEvent {
    pub fn show_client(self, content: impl AsRef<str> + Into<Vec<u8>>) -> bool {
        match self.e {
            Some(e) => show_client(e, content),
            None => false,
        }
    }

    pub fn close_client(self) {
        if let Some(e) = self.e {
            close_client(e);
        }
    }

    pub fn send_raw(self, function: &str, data: &[u8]) {
        if let Some(e) = self.e {
            send_raw_client(e, function, data);
        }
    }

    pub fn navigate_client(self, url: &str) {
        if let Some(e) = self.e {
            navigate_client(e, url);
        }
    }

    pub fn get_window(&self) -> Window {
        Window::from_id(self.window)
    }
}

pub struct Window {
    pub id: usize,
}

impl Window {
    pub fn new() -> Window {
        let id = new_window();
        Window { id }
    }

    pub fn from_id(id: usize) -> Window {
        Window { id }
    }

    pub fn show(&self, content: impl AsRef<str>) -> bool {
        show(self.id, content.as_ref())
    }

    pub fn show_browser(&self, content: impl AsRef<str>, browser: WebUIBrowser) -> bool {
        show_browser(self.id, content.as_ref(), browser)
    }

    pub fn start_server(&self, content: impl AsRef<str>) -> String {
        start_server(self.id, content.as_ref())
    }

    pub fn is_shown(&self) -> bool {
        is_shown(self.id)
    }

    pub fn get_port(&self) -> usize {
        get_port(self.id)
    }

    pub fn bind(&self, element: impl AsRef<str>, func: fn(WebUIEvent)) -> usize {
        interface_bind(self.id, element.as_ref(), func)
    }

    pub fn run_js(&self, js: impl AsRef<str>) -> JavaScript {
        let mut js = JavaScript {
            timeout: 0,
            script: js.as_ref().to_string(),
            error: false,
            data: "".to_string(),
        };

        run_js(self.id, &mut js);

        js
    }

    pub fn set_icon(&self, icon: impl AsRef<str>, kind: impl AsRef<str>) {
        set_icon(self.id, icon.as_ref(), kind.as_ref());
    }

    pub fn set_root_folder(&self, folder: impl AsRef<str>) {
        set_root_folder(self.id, folder.as_ref());
    }

    pub fn set_file_handler(
        &self,
        handler: unsafe extern "C" fn(*const i8, *mut i32) -> *const std::os::raw::c_void,
    ) {
        set_file_handler(self.id, handler);
    }

    pub fn set_runtime(&self, runtime: WebUIRuntime) {
        set_runtime(self.id, runtime);
    }

    pub fn close(&self) {
        close(self.id);
    }

    pub fn destroy(&self) {
        destroy(self.id);
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        destroy(self.id);
    }
}

// List of Rust user functions (2-dimensional array)
type FunctionType = fn(WebUIEvent);
const WINDOWS: usize = 64;
const ELEMENTS: usize = 64;

#[derive(Copy, Clone, Default)]
enum GlobalArray {
    #[default]
    None,
    Some(FunctionType),
}

static mut GLOBAL_ARRAY: [[GlobalArray; ELEMENTS]; WINDOWS] =
    [[GlobalArray::None; ELEMENTS]; WINDOWS];

static ELEMENTS_MAP: LazyLock<Mutex<HashMap<String, usize>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

// Save a string in the map and return its index
fn save_string(mut map: MutexGuard<HashMap<String, usize>>, s: &str) -> usize {
    // Check if the string already exists in the map
    if let Some(&index) = map.get(s) {
        return index;
    }

    // If the string does not exist, add it to the map and return the new index
    let index = map.len();
    map.insert(s.to_owned(), index);
    index
}

// Search for a string in the map and return its index if found, or -1 if not found
fn find_string(map: &HashMap<String, usize>, s: &str) -> isize {
    if let Some(&index) = map.get(s) {
        index as isize
    } else {
        -1
    }
}

fn char_to_string(c: *const i8) -> String {
    let cstr = unsafe { CStr::from_ptr(c) };
    let s: String = String::from_utf8_lossy(cstr.to_bytes()).to_string();
    s
}

fn cstr_to_string(c: CString) -> String {
    let s: String = String::from_utf8_lossy(c.to_bytes()).to_string();
    s
}

pub fn run_js(win: usize, js: &mut JavaScript) {
    /// The WebUI Script Interface
    struct WebUIScriptIntf {
        timeout: usize,
        script: *mut i8,
        error: bool,
        data: *const i8,
        length: usize,
    }

    unsafe {
        // Script String to i8/u8
        let script_cpy = js.script.clone();
        let script_c_str = CString::new(script_cpy).unwrap();
        let script_c_char: *mut c_char = script_c_str.as_ptr() as *mut c_char;

        let wuisi = WebUIScriptIntf {
            timeout: js.timeout,
            script: script_c_char,
            data: script_c_char,
            error: false,
            length: 0,
        };

        webui_script(
            win,
            wuisi.script,
            wuisi.timeout,
            script_c_char,
            wuisi.length,
        );

        js.error = wuisi.error;
        js.data = char_to_string(wuisi.data);
    }
}

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

pub fn bind(win: usize, element: &str, func: fn(WebUIEvent)) -> usize {
    let map = ELEMENTS_MAP.lock().unwrap();

    // Element String to i8/u8
    let element_c_str = CString::new(element).unwrap();
    let element_c_char: *const c_char = element_c_str.as_ptr() as *const c_char;

    let element_index = save_string(map, element);

    // Bind
    unsafe {
        let f: Option<unsafe extern "C" fn(*mut webui_event_t)> = Some(bind_events_handler);

        let window_id = webui_interface_get_window_id(win);

        // Add the Rust user function to the list
        GLOBAL_ARRAY[window_id][element_index] = GlobalArray::Some(func as FunctionType);

        webui_bind(win, element_c_char, f)
    }
}

unsafe extern "C" fn bind_events_handler(event: *mut webui_event_t) {
    let map = ELEMENTS_MAP.lock().unwrap();

    let element_index = find_string(&map, &char_to_string((*event).element));
    if element_index < 0 {
        return;
    }

    let evt = WebUIEvent {
        window: (*event).window,
        event_type: WebUIEventType::from_usize((*event).event_type),
        element: char_to_string((*event).element),
        event_number: (*event).event_number,
        bind_id: (*event).bind_id,
        client_id: Some((*event).client_id),
        connection_id: Some((*event).connection_id),
        cookies: Some(char_to_string((*event).cookies)),
        e: Some(event),
    };

    // Call the Rust user function
    unsafe {
        let window_id = webui_interface_get_window_id((*event).window);

        if let GlobalArray::Some(func) = GLOBAL_ARRAY[window_id][element_index as usize] {
            func(evt);
        }
    }
}

pub fn get_best_browser(window: usize) -> WebUIBrowser {
    unsafe {
        match webui_get_best_browser(window) {
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

pub fn show(win: usize, content: impl AsRef<str> + Into<Vec<u8>>) -> bool {
    unsafe {
        // Content String to i8/u8
        let content_c_str = CString::new(content).unwrap();
        let content_c_char: *const c_char = content_c_str.as_ptr() as *const c_char;

        webui_show(win, content_c_char)
    }
}

pub fn show_client(event: *mut webui_event_t, content: impl AsRef<str> + Into<Vec<u8>>) -> bool {
    unsafe {
        // Content String to i8/u8
        let content_c_str = CString::new(content).unwrap();
        let content_c_char: *const c_char = content_c_str.as_ptr() as *const c_char;

        webui_show_client(event, content_c_char)
    }
}

pub fn show_browser(
    win: usize,
    content: impl AsRef<str> + Into<Vec<u8>>,
    browser: WebUIBrowser,
) -> bool {
    let content_c_str = CString::new(content).unwrap();
    let content_c_char: *const c_char = content_c_str.as_ptr() as *const c_char;

    unsafe { webui_show_browser(win, content_c_char, browser as usize) }
}

pub fn start_server(win: usize, content: impl AsRef<str> + Into<Vec<u8>>) -> String {
    let content_c_str = CString::new(content).unwrap();
    let content_c_char: *const c_char = content_c_str.as_ptr() as *const c_char;

    unsafe {
        let server = webui_start_server(win, content_c_char);
        char_to_string(server)
    }
}

pub fn show_wv(win: usize, content: impl AsRef<str> + Into<Vec<u8>>) -> bool {
    let content_c_str = CString::new(content).unwrap();
    let content_c_char: *const c_char = content_c_str.as_ptr() as *const c_char;

    unsafe { webui_show_wv(win, content_c_char) }
}

pub fn set_kiosk(win: usize, status: bool) {
    unsafe {
        webui_set_kiosk(win, status);
    }
}

pub fn set_high_contrast(win: usize, status: bool) {
    unsafe {
        webui_set_high_contrast(win, status);
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

pub fn close(win: usize) {
    unsafe {
        webui_close(win);
    }
}

pub fn close_client(event: *mut webui_event_t) {
    unsafe {
        webui_close_client(event);
    }
}

pub fn destroy(win: usize) {
    unsafe {
        webui_destroy(win);
    }
}

pub fn exit() {
    unsafe {
        webui_exit();
    }
}

pub fn set_root_folder(win: usize, folder: &str) {
    let folder_c_str = CString::new(folder).unwrap();
    let folder_c_char: *const c_char = folder_c_str.as_ptr() as *const c_char;

    unsafe {
        webui_set_root_folder(win, folder_c_char);
    }
}

pub fn set_default_root_folder(folder: &str) {
    let folder_c_str = CString::new(folder).unwrap();
    let folder_c_char: *const c_char = folder_c_str.as_ptr() as *const c_char;

    unsafe {
        webui_set_default_root_folder(folder_c_char);
    }
}

pub fn set_file_handler(
    win: usize,
    handler: unsafe extern "C" fn(*const i8, *mut i32) -> *const std::os::raw::c_void,
) {
    unsafe {
        webui_set_file_handler(win, Some(handler));
    }
}

pub fn is_shown(win: usize) -> bool {
    unsafe { webui_is_shown(win) }
}

pub fn set_timeout(seconds: usize) {
    unsafe {
        webui_set_timeout(seconds);
    }
}

pub fn set_icon(win: usize, icon: &str, kind: &str) {
    let icon_c_str = CString::new(icon).unwrap();
    let kind_c_str = CString::new(kind).unwrap();
    let icon_c_char: *const c_char = icon_c_str.as_ptr() as *const c_char;
    let kind_c_char: *const c_char = kind_c_str.as_ptr() as *const c_char;

    unsafe {
        webui_set_icon(win, icon_c_char, kind_c_char);
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

pub fn send_raw(win: usize, function: &str, raw: *mut std::os::raw::c_void, size: usize) {
    let function_c_str = CString::new(function).unwrap();
    let function_c_char: *const c_char = function_c_str.as_ptr() as *const c_char;

    unsafe {
        webui_send_raw(win, function_c_char, raw, size);
    }
}

pub fn send_raw_client(
    event: *mut webui_event_t,
    function: &str,
    // raw: *mut std::os::raw::c_void,
    // size: usize,
    data: &[u8],
) {
    let function_c_str = CString::new(function).unwrap();
    let function_c_char: *const c_char = function_c_str.as_ptr() as *const c_char;

    let raw = data.as_ptr() as *mut std::os::raw::c_void;
    let size = data.len();

    unsafe {
        webui_send_raw_client(event, function_c_char, raw, size);
    }
}

pub fn set_hide(win: usize, status: bool) {
    unsafe {
        webui_set_hide(win, status);
    }
}

pub fn set_size(win: usize, width: u32, height: u32) {
    unsafe {
        webui_set_size(win, width, height);
    }
}

pub fn set_position(win: usize, x: u32, y: u32) {
    unsafe {
        webui_set_position(win, x, y);
    }
}

pub fn set_profile(win: usize, name: &str, path: &str) {
    let name_c_str = CString::new(name).unwrap();
    let path_c_str = CString::new(path).unwrap();
    let name_c_char: *const c_char = name_c_str.as_ptr() as *const c_char;
    let path_c_char: *const c_char = path_c_str.as_ptr() as *const c_char;

    unsafe {
        webui_set_profile(win, name_c_char, path_c_char);
    }
}

pub fn set_proxy(win: usize, proxy: &str) {
    let proxy_c_str = CString::new(proxy).unwrap();
    let proxy_c_char: *const c_char = proxy_c_str.as_ptr() as *const c_char;

    unsafe {
        webui_set_proxy(win, proxy_c_char);
    }
}

pub fn get_url(win: usize) -> String {
    unsafe {
        let url = webui_get_url(win);
        char_to_string(url)
    }
}

pub fn open_url(url: &str) {
    let url_c_str = CString::new(url).unwrap();
    let url_c_char: *const c_char = url_c_str.as_ptr() as *const c_char;

    unsafe {
        webui_open_url(url_c_char);
    }
}

pub fn set_public(win: usize, status: bool) {
    unsafe {
        webui_set_public(win, status);
    }
}

pub fn navigate(win: usize, url: &str) {
    let url_c_str = CString::new(url).unwrap();
    let url_c_char: *const c_char = url_c_str.as_ptr() as *const c_char;

    unsafe {
        webui_navigate(win, url_c_char);
    }
}

pub fn navigate_client(event: *mut webui_event_t, url: &str) {
    let url_c_str = CString::new(url).unwrap();
    let url_c_char: *const c_char = url_c_str.as_ptr() as *const c_char;

    unsafe {
        webui_navigate_client(event, url_c_char);
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

pub fn delete_profile(win: usize) {
    unsafe {
        webui_delete_profile(win);
    }
}

pub fn get_parent_process_id(win: usize) -> usize {
    unsafe { webui_get_parent_process_id(win) }
}

pub fn get_child_process_id(win: usize) -> usize {
    unsafe { webui_get_child_process_id(win) }
}

pub fn get_port(win: usize) -> usize {
    unsafe { webui_get_port(win) }
}

pub fn set_port(win: usize, port: usize) -> bool {
    unsafe { webui_set_port(win, port) }
}

pub fn get_free_port() -> usize {
    unsafe { webui_get_free_port() }
}

pub fn set_config(option: WebUIConfig, enabled: bool) {
    unsafe {
        webui_set_config(option as webui_config, enabled);
    }
}

pub fn set_event_blocking(win: usize, status: bool) {
    unsafe {
        webui_set_event_blocking(win, status);
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

pub fn run(win: usize, script: &str) {
    let script_c_str = CString::new(script).unwrap();
    let script_c_char: *const c_char = script_c_str.as_ptr() as *const c_char;

    unsafe {
        webui_run(win, script_c_char);
    }
}

pub fn run_client(event: *mut webui_event_t, script: &str) {
    let script_c_str = CString::new(script).unwrap();
    let script_c_char: *const c_char = script_c_str.as_ptr() as *const c_char;

    unsafe {
        webui_run_client(event, script_c_char);
    }
}

pub fn script(
    win: usize,
    script: &str,
    timeout: usize,
    buffer_length: usize,
) -> Result<String, ()> {
    let script_c_str = CString::new(script).unwrap();
    let script_c_char: *const c_char = script_c_str.as_ptr() as *const c_char;

    let buffer_c_str = CString::new(vec![0; buffer_length]).unwrap();
    let buffer_c_char: *mut c_char = buffer_c_str.as_ptr() as *mut c_char;

    unsafe {
        match webui_script(win, script_c_char, timeout, buffer_c_char, buffer_length) {
            true => Ok(char_to_string(buffer_c_char)),
            false => Err(()),
        }
    }
}

//fn script_client
pub fn script_client(
    event: *mut webui_event_t,
    script: &str,
    timeout: usize,
    buffer_length: usize,
) -> Result<String, ()> {
    let script_c_str = CString::new(script).unwrap();
    let script_c_char: *const c_char = script_c_str.as_ptr() as *const c_char;

    let buffer_c_str = CString::new(vec![0; buffer_length]).unwrap();
    let buffer_c_char: *mut c_char = buffer_c_str.as_ptr() as *mut c_char;

    unsafe {
        match webui_script_client(event, script_c_char, timeout, buffer_c_char, buffer_length) {
            true => Ok(char_to_string(buffer_c_char)),
            false => Err(()),
        }
    }
}

pub fn set_runtime(win: usize, runtime: WebUIRuntime) {
    unsafe {
        webui_set_runtime(win, runtime as usize);
    }
}

pub fn get_count(event: *mut webui_event_t) -> usize {
    unsafe { webui_get_count(event) }
}

pub fn get_int_at(event: *mut webui_event_t, index: usize) -> i64 {
    unsafe { webui_get_int_at(event, index) }
}

pub fn get_int(event: *mut webui_event_t) -> i64 {
    unsafe { webui_get_int(event) }
}

pub fn get_float_at(event: *mut webui_event_t, index: usize) -> f64 {
    unsafe { webui_get_float_at(event, index) }
}

pub fn get_float(event: *mut webui_event_t) -> f64 {
    unsafe { webui_get_float(event) }
}

pub fn get_string_at(event: *mut webui_event_t, index: usize) -> String {
    unsafe {
        let string = webui_get_string_at(event, index);
        char_to_string(string)
    }
}

pub fn get_string(event: *mut webui_event_t) -> String {
    unsafe {
        let string = webui_get_string(event);
        char_to_string(string)
    }
}

pub fn get_bool_at(event: *mut webui_event_t, index: usize) -> bool {
    unsafe { webui_get_bool_at(event, index) }
}

pub fn get_bool(event: *mut webui_event_t) -> bool {
    unsafe { webui_get_bool(event) }
}

pub fn get_size_at(event: *mut webui_event_t, index: usize) -> usize {
    unsafe { webui_get_size_at(event, index) }
}

pub fn get_size(event: *mut webui_event_t) -> usize {
    unsafe { webui_get_size(event) }
}

pub fn return_int(event: *mut webui_event_t, value: i64) {
    unsafe {
        webui_return_int(event, value);
    }
}

pub fn return_float(event: *mut webui_event_t, value: f64) {
    unsafe {
        webui_return_float(event, value);
    }
}

pub fn return_string(event: *mut webui_event_t, value: &str) {
    let value_c_str = CString::new(value).unwrap();
    let value_c_char: *const c_char = value_c_str.as_ptr() as *const c_char;

    unsafe {
        webui_return_string(event, value_c_char);
    }
}

pub fn return_bool(event: *mut webui_event_t, value: bool) {
    unsafe {
        webui_return_bool(event, value);
    }
}

unsafe extern "C" fn events_handler(
    window: usize,
    event_type: usize,
    element: *mut ::std::os::raw::c_char,
    event_number: usize,
    bind_id: usize,
) {
    let map = ELEMENTS_MAP.lock().unwrap();

    let element_index = find_string(&map, &char_to_string(element));
    if element_index < 0 {
        return;
    }

    let evt = WebUIEvent {
        window,
        event_type: WebUIEventType::from_usize(event_type),
        element: char_to_string(element),
        event_number,
        bind_id,
        client_id: None,
        connection_id: None,
        cookies: None,
        e: None,
    };

    // Call the Rust user function
    unsafe {
        let window_id = webui_interface_get_window_id(window);

        if let GlobalArray::Some(func) = GLOBAL_ARRAY[window_id][element_index as usize] {
            func(evt);
        }
    }
}

pub fn interface_bind(win: usize, element: &str, func: fn(WebUIEvent)) -> usize {
    let map = ELEMENTS_MAP.lock().unwrap();

    // Element String to i8/u8
    let element_c_str = CString::new(element).unwrap();
    let element_c_char: *const c_char = element_c_str.as_ptr() as *const c_char;

    let element_index = save_string(map, element);

    // Bind
    unsafe {
        let f: Option<
            unsafe extern "C" fn(usize, usize, *mut ::std::os::raw::c_char, usize, usize),
        > = Some(events_handler);

        let window_id = webui_interface_get_window_id(win);

        // Add the Rust user function to the list
        GLOBAL_ARRAY[window_id][element_index] = GlobalArray::Some(func as FunctionType);

        webui_interface_bind(win, element_c_char, f)
    }
}

pub fn interface_set_response(win: usize, event_number: usize, response: &str) {
    let response_c_str = CString::new(response).unwrap();
    let response_c_char: *const c_char = response_c_str.as_ptr() as *const c_char;

    unsafe {
        webui_interface_set_response(win, event_number, response_c_char);
    }
}

pub fn interface_is_app_running() -> bool {
    unsafe { webui_interface_is_app_running() }
}

pub fn interface_get_window_id(win: usize) -> usize {
    unsafe { webui_interface_get_window_id(win) }
}

pub fn interface_get_string_at(win: usize, event_number: usize, index: usize) -> String {
    unsafe {
        let string = webui_interface_get_string_at(win, event_number, index);
        char_to_string(string)
    }
}

pub fn interface_get_int_at(win: usize, event_number: usize, index: usize) -> i64 {
    unsafe { webui_interface_get_int_at(win, event_number, index) }
}

pub fn interface_get_float_at(win: usize, event_number: usize, index: usize) -> f64 {
    unsafe { webui_interface_get_float_at(win, event_number, index) }
}

pub fn interface_get_bool_at(win: usize, event_number: usize, index: usize) -> bool {
    unsafe { webui_interface_get_bool_at(win, event_number, index) }
}

pub fn interface_get_size_at(win: usize, event_number: usize, index: usize) -> usize {
    unsafe { webui_interface_get_size_at(win, event_number, index) }
}
