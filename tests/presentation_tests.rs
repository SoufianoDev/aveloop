use aveloop::domain::SessionState;
use aveloop::presentation::workspace::{Region, Workspace};
use gpui_kit::{
    px, size, AppContext as _, Bounds, Point, TestAppContext, WindowBounds, WindowOptions,
};

#[test]
fn test_workspace_composes_the_five_regions() {
    let regions = Workspace::regions();
    assert_eq!(
        regions,
        [
            Region::Editor,
            Region::Inspector,
            Region::Transport,
            Region::Visualization,
            Region::Output,
        ]
    );

    let mut labels: Vec<&str> = regions.iter().map(|region| region.label()).collect();
    labels.sort_unstable();
    labels.dedup();
    assert_eq!(labels.len(), regions.len());
}

#[gpui_kit::test]
fn test_workspace_initial_state(cx: &mut TestAppContext) {
    cx.update(gpui_kit::init);

    let bounds = Bounds {
        origin: Point::default(),
        size: size(px(900.), px(600.)),
    };
    let (window_handle, workspace) = cx.update(|cx| {
        gpui_kit::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            cx,
            |window, cx| cx.new(|cx| Workspace::new(window, cx)),
        )
        .expect("open test window")
    });

    cx.update_window(window_handle, |_, _, cx| {
        workspace.read_with(cx, |workspace, cx| {
            assert!(workspace.document_path().is_none());
            assert!(!workspace.is_dirty(cx));
            assert_eq!(workspace.session_state(), SessionState::Idle);
            assert_eq!(workspace.trace().points().len(), 7);
            assert_eq!(workspace.trace().current().unwrap().step_index, 6);
            assert!(workspace
                .editor()
                .read(cx)
                .value()
                .to_string()
                .contains("int main()"));
        });
    })
    .unwrap();
}
