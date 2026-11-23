use aveloop::domain::SessionState;
use aveloop::presentation::workspace::Workspace;

#[test]
fn test_workspace_initial_state() {
    let ws = Workspace::new();
    assert_eq!(ws.buffer().file_path(), "main.cpp");
    assert_eq!(ws.session_state(), SessionState::Idle);
    assert_eq!(ws.trace().points().len(), 7);
    assert_eq!(ws.trace().current().unwrap().step_index, 6);
}
