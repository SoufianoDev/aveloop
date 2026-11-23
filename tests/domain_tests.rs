use aveloop::domain::{ExecutionPoint, ExecutionTrace, SessionState, SourceBuffer, StopReason};

#[test]
fn test_source_buffer_line_count() {
    let buf = SourceBuffer::new("test.cpp", "line 1\nline 2\nline 3");
    assert_eq!(buf.file_path(), "test.cpp");
    assert_eq!(buf.line_count(), 3);
}

#[test]
fn test_execution_trace_navigation() {
    let mut trace = ExecutionTrace::new();
    assert!(trace.current().is_none());
    assert!(!trace.step_forward());
    assert!(!trace.step_backward());

    trace.push(ExecutionPoint {
        step_index: 0,
        file: "main.cpp".into(),
        line: 10,
        column: 1,
        reason: StopReason::Breakpoint(10),
    });
    trace.push(ExecutionPoint {
        step_index: 1,
        file: "main.cpp".into(),
        line: 11,
        column: 5,
        reason: StopReason::Step,
    });

    assert_eq!(trace.points().len(), 2);
    assert_eq!(trace.current().unwrap().step_index, 1);

    assert!(trace.step_backward());
    assert_eq!(trace.current().unwrap().step_index, 0);
    assert!(!trace.step_backward());

    assert!(trace.step_forward());
    assert_eq!(trace.current().unwrap().step_index, 1);
    assert!(!trace.step_forward());
}

#[test]
fn test_session_state_transitions() {
    let mut state = SessionState::Idle;
    assert_eq!(state, SessionState::Idle);

    state = SessionState::Compiling;
    assert_eq!(state, SessionState::Compiling);

    state = SessionState::Debugging;
    assert_eq!(state, SessionState::Debugging);

    state = SessionState::Paused;
    assert_eq!(state, SessionState::Paused);

    state = SessionState::Terminated;
    assert_eq!(state, SessionState::Terminated);
}
