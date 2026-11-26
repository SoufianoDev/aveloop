use aveloop::domain::SessionState;
use aveloop::presentation::workspace::{Region, Workspace};

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

#[test]
fn test_workspace_initial_state() {
    let ws = Workspace::new();
    assert_eq!(ws.buffer().file_path(), "main.cpp");
    assert_eq!(ws.session_state(), SessionState::Idle);
    assert_eq!(ws.trace().points().len(), 7);
    assert_eq!(ws.trace().current().unwrap().step_index, 6);
}
