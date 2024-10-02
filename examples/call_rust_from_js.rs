use webui_rs::webui::{wait, WebUIEvent, Window};

fn main() {
    let win = Window::new();

    // Inline function
    win.bind("my_button", |_: WebUIEvent| {
        println!("Button clicked!");
    });

    win.show(
        r#"
  <html>
    <script src="/webui.js"></script>
    <button id="my_button">Click me for some backend logs!</button>
  </html>
  "#,
    );

    wait();
}
