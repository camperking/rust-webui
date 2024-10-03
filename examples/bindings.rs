use webui_rs::webui;

const HTML: &str = r#"
<html>
    <script src="/webui.js"></script>
    <button id="btn">Click me!</button>
    <button onclick="webui.call('add', 1, 2).then((res) => { console.log(res); })">Add 1 + 2</button>
</html>
"#;

fn main() {
    let win = webui::Window::new();

    win.show_browser(HTML, webui::WebUIBrowser::Firefox);

    webui::bind(win.id, "btn", |_| {
        println!("Element clicked!");
    });

    webui::bind(win.id, "add", |event| {
        let count = event.get_count();
        if count < 2 {
            return;
        }

        let a = event.get_int_at(0);
        let b = event.get_int_at(1);

        println!("{} + {} = {}", a, b, a + b);

        event.return_int(a + b);
    });

    webui::wait();
}
