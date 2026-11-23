use aveloop::presentation::{theme, workspace::Workspace};
use gpui_kit::*;

fn main() {
    const {
        assert!(
            gpui_kit::GPUI_FAST,
            "gpui-fast retained-mode backend required"
        )
    };

    gpui_kit::application().run(|cx| {
        gpui_kit::init(cx);
        theme::apply_night_owl(cx);

        let _ = gpui_kit::open_window(
            WindowOptions {
                titlebar: Some(TitlebarOptions {
                    title: Some("AveLoop".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            cx,
            |_, cx| cx.new(|_| Workspace::new()),
        )
        .expect("failed to open window");
    });
}
