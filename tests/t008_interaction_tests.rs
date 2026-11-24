use gpui_kit::component::{
    button::Button,
    input::{Input, InputState},
};
use gpui_kit::test::{TestSupportExt, TestWindowExt};
use gpui_kit::{
    div, px, size, AppContext, Bounds, ClipboardEntry, ClipboardItem, Context, Entity,
    InteractiveElement, IntoElement, ParentElement, Point, Render, SharedString,
    StatefulInteractiveElement, Styled, TestAppContext, Window, WindowBounds, WindowOptions,
};

struct TestWorkspace {
    input_state: Entity<InputState>,
    status: SharedString,
    clipboard_text: SharedString,
}

impl Render for TestWorkspace {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("test-workspace")
            .test_support()
            .size_full()
            .flex()
            .flex_col()
            .p_4()
            .gap_3()
            .child(
                div()
                    .id("editor-input")
                    .w(px(300.))
                    .child(Input::new(&self.input_state).id("input-box")),
            )
            .child(
                Button::new("copy-btn")
                    .label("Copy")
                    .on_click(cx.listener(|this, _, _, cx| {
                        let val = this.input_state.read(cx).value();
                        cx.write_to_clipboard(ClipboardItem::new_string(val.to_string()));
                        this.status = "Copied".into();
                        cx.notify();
                    })),
            )
            .child(
                Button::new("paste-btn")
                    .label("Paste")
                    .on_click(cx.listener(|this, _, _, cx| {
                        if let Some(item) = cx.read_from_clipboard() {
                            if let Some(ClipboardEntry::String(s)) = item.entries().first() {
                                this.clipboard_text = s.text().into();
                                this.status = "Pasted".into();
                                cx.notify();
                            }
                        }
                    })),
            )
            .child(
                div()
                    .id("status-label")
                    .aria_label(self.status.clone())
                    .test_support()
                    .child(self.status.clone()),
            )
            .child(
                div()
                    .id("clipboard-result")
                    .aria_label(self.clipboard_text.clone())
                    .test_support()
                    .child(self.clipboard_text.clone()),
            )
    }
}

#[gpui_kit::test]
fn test_t008_text_input_focus_and_keyboard(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);

    let bounds = Bounds {
        origin: Point::default(),
        size: size(px(800.), px(600.)),
    };

    let (window_handle, _) = cx.update(|cx| {
        gpui_kit::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            cx,
            |window, cx| {
                cx.new(|cx| TestWorkspace {
                    input_state: cx.new(|cx| InputState::new(window, cx)),
                    status: "Initial".into(),
                    clipboard_text: "".into(),
                })
            },
        )
        .expect("open test window")
    });

    cx.update_window(window_handle, |_, window, cx| {
        window.render_frame(cx);

        // 1. Focus test
        window.click("input-box", cx);
        let input_elem = window.find("input-box");
        assert_eq!(input_elem.focused(), Some(true));

        // 2. Keyboard typing test
        window.input("int main()", cx);
        assert_eq!(window.find("input-box").value(), Some("int main()"));

        // 3. Keyboard backspace test
        window.press("backspace", cx);
        assert_eq!(window.find("input-box").value(), Some("int main("));

        // 4. Clipboard copy test through UI button
        window.click("copy-btn", cx);
        assert_eq!(window.find("status-label").label(), Some("Copied"));
    })
    .unwrap();

    // 5. System clipboard verification
    cx.update(|cx| {
        let item = cx.read_from_clipboard().expect("clipboard item exists");
        if let Some(ClipboardEntry::String(s)) = item.entries().first() {
            assert_eq!(s.text(), "int main(");
        } else {
            panic!("expected string clipboard entry");
        }
    });

    // 6. Clipboard paste test through UI button
    cx.update_window(window_handle, |_, window, cx| {
        window.click("paste-btn", cx);
        assert_eq!(window.find("status-label").label(), Some("Pasted"));
        assert_eq!(window.find("clipboard-result").label(), Some("int main("));
    })
    .unwrap();
}

#[gpui_kit::test]
fn test_t008_window_resizing_and_layout_reflow(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);

    let initial_bounds = Bounds {
        origin: Point::default(),
        size: size(px(600.), px(400.)),
    };

    let (window_handle, _) = cx.update(|cx| {
        gpui_kit::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(initial_bounds)),
                ..Default::default()
            },
            cx,
            |window, cx| {
                cx.new(|cx| TestWorkspace {
                    input_state: cx.new(|cx| InputState::new(window, cx)),
                    status: "Ready".into(),
                    clipboard_text: "".into(),
                })
            },
        )
        .expect("open test window")
    });

    cx.update_window(window_handle, |_, window, cx| {
        window.render_frame(cx);
        let initial_size = window.bounds().size;
        assert_eq!(initial_size.width, px(600.));
        assert_eq!(initial_size.height, px(400.));
        assert_eq!(window.viewport_size(), initial_size);
        let initial_layout = window.find("test-workspace").bounds().size;

        // Resize window to larger bounds
        window.resize(size(px(1024.), px(768.)));
        window.bounds_changed(cx);
        window.render_frame(cx);

        let new_size = window.bounds().size;
        assert_eq!(new_size.width, px(1024.));
        assert_eq!(new_size.height, px(768.));
        assert_eq!(window.viewport_size(), new_size);

        let layout = window.find("test-workspace").bounds().size;
        assert_eq!(layout.width - initial_layout.width, px(424.));
        assert_eq!(layout.height - initial_layout.height, px(368.));

        // Verify elements are still present and responsive
        assert!(window.find("copy-btn").visible());
        assert!(window.find("paste-btn").visible());
    })
    .unwrap();
}
