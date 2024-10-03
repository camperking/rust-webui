use webui_rs::{webui::wait, window::WebUIWindow};

fn main() {
    let win = WebUIWindow::new();

    win.show("examples/html/index.html");

    wait();
}
