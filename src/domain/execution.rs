use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StopReason {
    Breakpoint(u32),
    Step,
    Exception(String),
    Exited(i32),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionPoint {
    pub step_index: usize,
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub reason: StopReason,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "ExecutionTraceData")]
pub struct ExecutionTrace {
    points: Vec<ExecutionPoint>,
    current_cursor: Option<usize>,
}

#[derive(Deserialize)]
#[serde(rename = "ExecutionTrace")]
struct ExecutionTraceData {
    points: Vec<ExecutionPoint>,
    current_cursor: Option<usize>,
}

impl TryFrom<ExecutionTraceData> for ExecutionTrace {
    type Error = &'static str;

    fn try_from(data: ExecutionTraceData) -> Result<Self, Self::Error> {
        if data
            .current_cursor
            .is_some_and(|cursor| cursor >= data.points.len())
        {
            return Err("execution trace cursor is outside points");
        }

        Ok(Self {
            points: data.points,
            current_cursor: data.current_cursor,
        })
    }
}

impl ExecutionTrace {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, point: ExecutionPoint) {
        self.points.push(point);
        self.current_cursor = Some(self.points.len() - 1);
    }

    pub fn points(&self) -> &[ExecutionPoint] {
        &self.points
    }

    pub fn current(&self) -> Option<&ExecutionPoint> {
        self.current_cursor.and_then(|idx| self.points.get(idx))
    }

    pub fn step_backward(&mut self) -> bool {
        if let Some(cursor) = self.current_cursor {
            if cursor > 0 {
                self.current_cursor = Some(cursor - 1);
                return true;
            }
        }
        false
    }

    pub fn step_forward(&mut self) -> bool {
        if let Some(cursor) = self.current_cursor {
            if cursor + 1 < self.points.len() {
                self.current_cursor = Some(cursor + 1);
                return true;
            }
        }
        false
    }
}
