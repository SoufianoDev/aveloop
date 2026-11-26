use aveloop::presentation::{
    theme,
    workspace::{self, Workspace},
};
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
        workspace::bind_keys(cx);
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
            |window, cx| cx.new(|cx| Workspace::new(window, cx)),
        )
        .expect("failed to open window");
    });
}
