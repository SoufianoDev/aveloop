use gpui_kit::base::StyledExt;
use gpui_kit::component::{
    button::Button, scroll::ScrollableElement, status_bar::StatusBar, tag::Tag, ActiveTheme,
};
use gpui_kit::*;

use crate::domain::{ExecutionPoint, ExecutionTrace, SessionState, SourceBuffer, StopReason};

/// The five regions the Alpha workspace composes. Each one owns a later task; T024 only
/// establishes the layout, the labels, and the shared theme surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Region {
    Editor,
    Inspector,
    Transport,
    Visualization,
    Output,
}

impl Region {
    pub const ALL: [Self; 5] = [
        Self::Editor,
        Self::Inspector,
        Self::Transport,
        Self::Visualization,
        Self::Output,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::Editor => "Editor",
            Self::Inspector => "Inspector",
            Self::Transport => "Transport",
            Self::Visualization => "Visualization",
            Self::Output => "Output",
        }
    }
}

pub struct Workspace {
    buffer: SourceBuffer,
    demo_trace: ExecutionTrace,
    session_state: SessionState,
}

impl Default for Workspace {
    fn default() -> Self {
        Self::new()
    }
}

impl Workspace {
    pub fn new() -> Self {
        let initial_code = r#"#include <iostream>

int main() {
    int sum = 0;
    for (int i = 0; i < 3; ++i) {
        sum += i;
    }
    std::cout << "Sum: " << sum << std::endl;
    return 0;
}
"#;
        // Synthetic fixture for the UI preview, independent of debugger sessions.
        let mut demo_trace = ExecutionTrace::new();
        demo_trace.push(ExecutionPoint {
            step_index: 0,
            file: "main.cpp".into(),
            line: 4,
            column: 9,
            reason: StopReason::Breakpoint(4),
        });
        demo_trace.push(ExecutionPoint {
            step_index: 1,
            file: "main.cpp".into(),
            line: 5,
            column: 19,
            reason: StopReason::Step,
        });
        demo_trace.push(ExecutionPoint {
            step_index: 2,
            file: "main.cpp".into(),
            line: 6,
            column: 9,
            reason: StopReason::Step,
        });
        demo_trace.push(ExecutionPoint {
            step_index: 3,
            file: "main.cpp".into(),
            line: 5,
            column: 29,
            reason: StopReason::Step,
        });
        demo_trace.push(ExecutionPoint {
            step_index: 4,
            file: "main.cpp".into(),
            line: 6,
            column: 9,
            reason: StopReason::Step,
        });
        demo_trace.push(ExecutionPoint {
            step_index: 5,
            file: "main.cpp".into(),
            line: 8,
            column: 5,
            reason: StopReason::Step,
        });
        demo_trace.push(ExecutionPoint {
            step_index: 6,
            file: "main.cpp".into(),
            line: 9,
            column: 5,
            reason: StopReason::Exited(0),
        });

        Self {
            buffer: SourceBuffer::new("main.cpp", initial_code),
            demo_trace,
            session_state: SessionState::Idle,
        }
    }

    pub fn buffer(&self) -> &SourceBuffer {
        &self.buffer
    }

    /// Returns the synthetic trace shown in the integration preview.
    pub fn trace(&self) -> &ExecutionTrace {
        &self.demo_trace
    }

    pub fn session_state(&self) -> SessionState {
        self.session_state
    }

    /// Region order, top to bottom and left to right as composed in [`Render`].
    pub fn regions() -> [Region; 5] {
        Region::ALL
    }

    fn step_forward(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.demo_trace.step_forward() {
            cx.notify();
        }
    }

    fn step_backward(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.demo_trace.step_backward() {
            cx.notify();
        }
    }

    fn region_title(&self, region: Region, cx: &mut Context<Self>) -> Div {
        div()
            .flex()
            .items_center()
            .px(px(12.))
            .h(px(28.))
            .border_b_1()
            .border_color(cx.theme().border)
            .child(
                div()
                    .text_xs()
                    .font_bold()
                    .text_color(cx.theme().muted_foreground)
                    .child(region.label()),
            )
    }

    fn editor_region(&self, cx: &mut Context<Self>) -> Div {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .h_full()
            .overflow_hidden()
            .border_r_1()
            .border_color(cx.theme().border)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .h(px(28.))
                    .justify_center()
                    .px(px(12.))
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .child(
                        div()
                            .text_xs()
                            .font_bold()
                            .text_color(cx.theme().muted_foreground)
                            .child(format!(
                                "{} · {}",
                                Region::Editor.label(),
                                self.buffer.file_path()
                            )),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .p(px(12.))
                    .overflow_y_scrollbar()
                    .children(
                        self.buffer
                            .content()
                            .lines()
                            .enumerate()
                            .map(|(idx, line_text)| {
                                let line_no = (idx + 1) as u32;
                                div()
                                    .flex()
                                    .items_center()
                                    .w_full()
                                    .px(px(4.))
                                    .py(px(2.))
                                    .rounded(px(4.))
                                    .child(
                                        div()
                                            .w(px(32.))
                                            .text_xs()
                                            .text_color(cx.theme().muted_foreground)
                                            .child(format!("{:>2}", line_no)),
                                    )
                                    .child(
                                        div()
                                            .flex_1()
                                            .text_xs()
                                            .font_family(cx.theme().mono_font_family.clone())
                                            .child(line_text.to_string()),
                                    )
                            }),
                    ),
            )
    }

    fn inspector_region(&self, cx: &mut Context<Self>) -> Div {
        let current = self.demo_trace.current();
        let step_idx = current.map(|point| point.step_index).unwrap_or(0);
        let current_line = current.map(|point| point.line).unwrap_or(0);
        let reason = current
            .map(|point| point.reason.clone())
            .unwrap_or(StopReason::Step);

        div()
            .flex()
            .flex_col()
            .w(px(320.))
            .h_full()
            .bg(cx.theme().secondary)
            .child(self.region_title(Region::Inspector, cx))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .p(px(12.))
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .text_xs()
                            .child("Step:")
                            .child(format!(
                                "{} / {}",
                                step_idx + 1,
                                self.demo_trace.points().len()
                            )),
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .text_xs()
                            .child("Line:")
                            .child(format!("{}", current_line)),
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .text_xs()
                            .child("Reason:")
                            .child(format!("{:?}", reason)),
                    ),
            )
    }

    fn transport_region(&self, cx: &mut Context<Self>, step_idx: usize, total_steps: usize) -> Div {
        div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(44.))
            .px(px(16.))
            .border_t_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().tab_bar)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(12.))
                    .child(
                        div()
                            .text_xs()
                            .font_bold()
                            .text_color(cx.theme().muted_foreground)
                            .child(Region::Transport.label()),
                    )
                    .child(div().text_xs().child(format!(
                        "Stop {} / {}",
                        step_idx + 1,
                        total_steps
                    ))),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.))
                    .child(
                        Button::new("btn-step-back")
                            .outline()
                            .label("Step Back")
                            .on_click(cx.listener(Self::step_backward)),
                    )
                    .child(
                        Button::new("btn-step-fwd")
                            .label("Step Forward")
                            .on_click(cx.listener(Self::step_forward)),
                    ),
            )
    }

    fn visualization_region(&self, cx: &mut Context<Self>) -> Div {
        let selected = self.demo_trace.current().cloned();

        div()
            .flex()
            .flex_col()
            .flex_1()
            .h_full()
            .border_r_1()
            .border_color(cx.theme().border)
            .child(self.region_title(Region::Visualization, cx))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .flex_1()
                    .p(px(12.))
                    .child(
                        div().text_xs().text_color(cx.theme().muted_foreground).child(
                            "Recorded stops of the demo trace; the shared visualization engine is a later task.",
                        ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(4.))
                            .children(self.demo_trace.points().iter().map(|point| {
                                let current = selected
                                    .as_ref()
                                    .is_some_and(|stop| stop.step_index == point.step_index);
                                div()
                                    .w(px(18.))
                                    .h(px(6.))
                                    .rounded(px(3.))
                                    .bg(if current {
                                        cx.theme().foreground
                                    } else {
                                        cx.theme().border
                                    })
                            })),
                    ),
            )
    }

    fn output_region(&self, cx: &mut Context<Self>) -> Div {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .h_full()
            .child(self.region_title(Region::Output, cx))
            .child(
                div().p(px(12.)).text_xs().text_color(cx.theme().muted_foreground).child(
                    "No program output yet. Standard output and standard error appear here after a run.",
                ),
            )
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let step_idx = self
            .demo_trace
            .current()
            .map(|point| point.step_index)
            .unwrap_or(0);
        let total_steps = self.demo_trace.points().len();

        let state_label = match self.session_state {
            SessionState::Idle => "IDLE",
            SessionState::Compiling => "COMPILING",
            SessionState::Debugging => "DEBUGGING",
            SessionState::Paused => "PAUSED",
            SessionState::Terminated => "TERMINATED",
        };

        let bg_color = cx.theme().background;
        let border_color = cx.theme().border;

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(bg_color)
            .text_color(cx.theme().foreground)
            .child(
                div()
                    .flex()
                    .h(px(48.))
                    .w_full()
                    .items_center()
                    .px(px(16.))
                    .border_b_1()
                    .border_color(border_color)
                    .bg(cx.theme().tab_bar)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(12.))
                            .child(div().text_sm().font_bold().child("AveLoop"))
                            .child(Tag::info().child("DEMO PREVIEW"))
                            .child(Tag::info().child(state_label)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .child(self.editor_region(cx))
                    .child(self.inspector_region(cx)),
            )
            .child(self.transport_region(cx, step_idx, total_steps))
            .child(
                div()
                    .flex()
                    .w_full()
                    .h(px(180.))
                    .overflow_hidden()
                    .child(self.visualization_region(cx))
                    .child(self.output_region(cx)),
            )
            .child(
                StatusBar::new()
                    .left(format!("Demo file: {}", self.buffer.file_path()))
                    .right("GPUI Fast | Night Owl | M3"),
            )
    }
}
