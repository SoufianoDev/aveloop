use aveloop::presentation::theme;
use gpui_kit::base::StyledExt;
use gpui_kit::App;
use gpui_kit::component::{
    button::Button,
    input::{Input, InputEvent, InputState},
    ActiveTheme,
};
use gpui_kit::*;

struct T008VerificationApp {
    input_state: Entity<InputState>,
    status: SharedString,
    clipboard_content: SharedString,
    wayland_display: Option<String>,
    x11_display: Option<String>,
    window_size: Size<Pixels>,
    _subscriptions: Vec<Subscription>,
}

impl T008VerificationApp {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let wayland_display = std::env::var("WAYLAND_DISPLAY").ok();
        let x11_display = std::env::var("DISPLAY").ok();

        let input_state = cx.new(|cx| {
            InputState::new(window, cx).placeholder("Type C++ code or paste here...")
        });

        let _subscriptions = vec![cx.subscribe_in(&input_state, window, {
            let input_state = input_state.clone();
            move |this, _, ev: &InputEvent, _window, cx| match ev {
                InputEvent::Change => {
                    let val = input_state.read(cx).value();
                    this.status = format!("Input changed (len={})", val.len()).into();
                    cx.notify();
                }
                InputEvent::PressEnter { .. } => {
                    this.status = "Enter pressed".into();
                    cx.notify();
                }
                _ => {}
            }
        })];

        Self {
            input_state,
            status: "Ready".into(),
            clipboard_content: "".into(),
            wayland_display,
            x11_display,
            window_size: window.bounds().size,
            _subscriptions,
        }
    }
}

impl Render for T008VerificationApp {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.window_size = window.bounds().size;

        let display_server_label = if let Some(ref w) = self.wayland_display {
            format!("Wayland ({})", w)
        } else if let Some(ref x) = self.x11_display {
            format!("X11 ({})", x)
        } else {
            "Headless / Unknown".into()
        };

        div()
            .size_full()
            .flex()
            .flex_col()
            .p_4()
            .gap_4()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .pb_2()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .child(
                        div()
                            .text_base()
                            .font_bold()
                            .child("AveLoop — T008 Interaction & Hardware Verification"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .px_2()
                            .py_1()
                            .rounded(px(4.))
                            .bg(cx.theme().secondary)
                            .child(display_server_label),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_xs()
                            .font_bold()
                            .text_color(cx.theme().muted_foreground)
                            .child("1. Text Rendering & Keyboard Input:"),
                    )
                    .child(
                        Input::new(&self.input_state)
                            .id("input-verification-field")
                            .w(px(480.)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Button::new("copy-btn")
                            .label("Copy Input to Clipboard")
                            .on_click(cx.listener(|this, _, _, cx| {
                                let val = this.input_state.read(cx).value();
                                cx.write_to_clipboard(ClipboardItem::new_string(val.to_string()));
                                this.status = "Copied to system clipboard".into();
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new("paste-btn")
                            .label("Paste from Clipboard")
                            .on_click(cx.listener(|this, _, _, cx| {
                                if let Some(item) = cx.read_from_clipboard() {
                                    if let Some(ClipboardEntry::String(s)) = item.entries().first()
                                    {
                                        this.clipboard_content = s.text().into();
                                        this.status = "Pasted from system clipboard".into();
                                        cx.notify();
                                    }
                                }
                            })),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .p_3()
                    .bg(cx.theme().secondary)
                    .rounded(px(6.))
                    .child(
                        div()
                            .text_xs()
                            .child(format!("Window Dimensions: {} x {}", self.window_size.width, self.window_size.height)),
                    )
                    .child(
                        div()
                            .text_xs()
                            .child(format!("Status: {}", self.status)),
                    )
                    .child(
                        div()
                            .text_xs()
                            .child(format!("Clipboard Value: {}", self.clipboard_content)),
                    ),
            )
    }
}

fn read_clipboard_text(cx: &mut App) -> Option<String> {
    cx.read_from_clipboard().and_then(|item| {
        match item.entries().first() {
            Some(ClipboardEntry::String(s)) => Some(s.text().to_string()),
            _ => None,
        }
    })
}

fn main() {
    println!("[T008] Starting GPUI interaction verification...");

    let wayland = std::env::var("WAYLAND_DISPLAY").ok();
    let x11 = std::env::var("DISPLAY").ok();
    println!("[T008] WAYLAND_DISPLAY: {:?}", wayland);
    println!("[T008] DISPLAY: {:?}", x11);

    const { assert!(gpui_kit::GPUI_FAST, "gpui-fast retained mode required") };

    let is_automated = std::env::var("AVELOOP_PROBE_EXIT").is_ok();
    let read_only = std::env::var("AVELOOP_PROBE_READ_ONLY").is_ok();

    gpui_kit::application().run(move |cx| {
        gpui_kit::init(cx);
        theme::apply_night_owl(cx);

        // The clipboard must be read before the event loop starts: reading from a
        // deferred callback panics on X11 ("RefCell already borrowed").
        let probe_text = "AveLoop T008 Clipboard Verification";
        if !read_only {
            cx.write_to_clipboard(ClipboardItem::new_string(probe_text.to_string()));
        }
        match read_clipboard_text(cx).as_deref() {
            Some(text) if text == probe_text => {
                println!("[T008] Programmatic clipboard write & read: PASSED")
            }
            Some(text) if read_only => {
                println!("[T008] Clipboard read from external owner: {:?}", text)
            }
            other if read_only => {
                println!("[T008] Clipboard read from external owner: FAILED ({:?})", other)
            }
            other => println!("[T008] Programmatic clipboard write & read: FAILED ({:?})", other),
        }

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(Bounds {
                origin: Point::default(),
                size: size(px(720.), px(480.)),
            })),
            titlebar: Some(TitlebarOptions {
                title: Some("AveLoop T008 Verification".into()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let _ = gpui_kit::open_window(window_options, cx, |window, cx| {
            cx.new(|cx| T008VerificationApp::new(window, cx))
        })
        .expect("Failed to open T008 verification window");

        println!("[T008] Verification window opened successfully.");

        // Defer so the platform event loop is running: a quit issued inside the
        // launch callback is swallowed by the calloop signal, and Wayland delivers
        // selection offers only after the loop starts.
        if is_automated {
            cx.defer(move |cx| {
                println!("[T008] Automated probe mode: closing application.");
                cx.quit();
            });
        }
    });
}
