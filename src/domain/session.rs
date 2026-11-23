#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Idle,
    Compiling,
    Debugging,
    Paused,
    Terminated,
}
