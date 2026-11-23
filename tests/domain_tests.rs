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

#[test]
fn test_execution_trace_deserialization_validates_cursor() {
    let point = ExecutionPoint {
        step_index: 0,
        file: "main.cpp".into(),
        line: 10,
        column: 1,
        reason: StopReason::Step,
    };
    let mut trace = ExecutionTrace::new();
    let empty = serde_json::to_string(&trace).unwrap();
    assert_eq!(
        serde_json::from_str::<ExecutionTrace>(&empty).unwrap(),
        trace
    );
    trace.push(point.clone());
    trace.push(ExecutionPoint {
        step_index: 1,
        ..point
    });
    assert!(trace.step_backward());

    let valid = serde_json::to_value(&trace).unwrap();
    assert_eq!(
        serde_json::from_value::<ExecutionTrace>(valid.clone()).unwrap(),
        trace,
    );

    for cursor in [trace.points().len(), usize::MAX] {
        let mut invalid = valid.clone();
        invalid["current_cursor"] = cursor.into();
        assert!(serde_json::from_value::<ExecutionTrace>(invalid).is_err());
    }
    assert!(serde_json::from_str::<ExecutionTrace>(r#"{"points":[],"current_cursor":0}"#).is_err());

    let mut unselected = valid;
    unselected.as_object_mut().unwrap().remove("current_cursor");
    let mut decoded = serde_json::from_value::<ExecutionTrace>(unselected).unwrap();
    assert_eq!(decoded.points(), trace.points());
    assert!(decoded.current().is_none());
    assert!(!decoded.step_backward());
    assert!(!decoded.step_forward());
}
