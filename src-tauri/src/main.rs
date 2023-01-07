use std::fmt::Debug;

use tauri::{utils::config::parse::parse_value, Icon};
#[allow(unused_imports)]
use wry::*;
use wry::application::{platform::windows::WindowExtWindows};
fn main() -> wry::Result<()> {
    use wry::{
        application::{
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            window::WindowBuilder,
        },
        webview::WebViewBuilder,
    };

    enum UserEvent {
        NewWindow(String),
    }

    let html = r#"
      <body>
        <div>
          <script>
            window.resizeTo(1280,720)
            window.location.replace("https://app.skiff.com")
            var anchor = document.querySelector('a')
            anchor.rel = "noopener"
            anchor.target = "_top"
            console.log(anchor)
            anchor.addEventListener('click', (event) =>
                event.target = "_self"
            );
          </script>
        </div>
      </body>
    "#;

    let event_loop: EventLoop<UserEvent> = EventLoop::with_user_event();
    let proxy = event_loop.create_proxy();
    let window = WindowBuilder::new()
        .with_title("Skiff App")
        .build(&event_loop)?;
    let _webview = WebViewBuilder::new(window)?
        .with_html(html)?
        .with_new_window_req_handler(move |uri: String| {
            let submitted = proxy.send_event(UserEvent::NewWindow(uri.clone())).is_ok();
            submitted && uri.contains("wikipedia")
        })
        .build()?;
        
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::UserEvent(UserEvent::NewWindow(uri)) => {
                let icon = std::option::Option::None;
               _webview.window().set_window_icon(icon);
                _webview.load_url(&uri);
            }
            _ => (),
        }
    });
}
