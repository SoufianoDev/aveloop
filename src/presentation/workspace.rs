use gpui_kit::base::StyledExt;
use gpui_kit::component::{
    button::Button, scroll::ScrollableElement, status_bar::StatusBar, tag::Tag, ActiveTheme,
};
use gpui_kit::*;

use crate::domain::{ExecutionPoint, ExecutionTrace, SessionState, SourceBuffer, StopReason};

pub struct Workspace {
    buffer: SourceBuffer,
    trace: ExecutionTrace,
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
        let mut trace = ExecutionTrace::new();
        trace.push(ExecutionPoint {
            step_index: 0,
            file: "main.cpp".into(),
            line: 4,
            column: 9,
            reason: StopReason::Breakpoint(4),
        });
        trace.push(ExecutionPoint {
            step_index: 1,
            file: "main.cpp".into(),
            line: 5,
            column: 19,
            reason: StopReason::Step,
        });
        trace.push(ExecutionPoint {
            step_index: 2,
            file: "main.cpp".into(),
            line: 6,
            column: 9,
            reason: StopReason::Step,
        });
        trace.push(ExecutionPoint {
            step_index: 3,
            file: "main.cpp".into(),
            line: 5,
            column: 29,
            reason: StopReason::Step,
        });
        trace.push(ExecutionPoint {
            step_index: 4,
            file: "main.cpp".into(),
            line: 6,
            column: 9,
            reason: StopReason::Step,
        });
        trace.push(ExecutionPoint {
            step_index: 5,
            file: "main.cpp".into(),
            line: 8,
            column: 5,
            reason: StopReason::Step,
        });
        trace.push(ExecutionPoint {
            step_index: 6,
            file: "main.cpp".into(),
            line: 9,
            column: 5,
            reason: StopReason::Exited(0),
        });

        Self {
            buffer: SourceBuffer::new("main.cpp", initial_code),
            trace,
            session_state: SessionState::Paused,
        }
    }

    pub fn buffer(&self) -> &SourceBuffer {
        &self.buffer
    }

    pub fn trace(&self) -> &ExecutionTrace {
        &self.trace
    }

    pub fn session_state(&self) -> SessionState {
        self.session_state
    }

    fn step_forward(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.trace.step_forward() {
            cx.notify();
        }
    }

    fn step_backward(&mut self, _: &ClickEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.trace.step_backward() {
            cx.notify();
        }
    }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let current_pt = self.trace.current().cloned();
        let current_line = current_pt.as_ref().map(|p| p.line).unwrap_or(0);
        let step_idx = current_pt.as_ref().map(|p| p.step_index).unwrap_or(0);
        let total_steps = self.trace.points().len();

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
                    .justify_between()
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
                            .child(Tag::info().child(state_label)),
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
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_1()
                    .w_full()
                    .overflow_hidden()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .h_full()
                            .p(px(16.))
                            .overflow_y_scrollbar()
                            .border_r_1()
                            .border_color(border_color)
                            .child(
                                div()
                                    .text_xs()
                                    .font_bold()
                                    .text_color(cx.theme().muted_foreground)
                                    .pb(px(8.))
                                    .child(self.buffer.file_path().to_string()),
                            )
                            .children(self.buffer.content().lines().enumerate().map(
                                |(idx, line_text)| {
                                    let line_no = (idx + 1) as u32;
                                    let is_active = line_no == current_line;

                                    let row_bg = if is_active {
                                        cx.theme().accent
                                    } else {
                                        transparent_black()
                                    };

                                    div()
                                        .flex()
                                        .items_center()
                                        .w_full()
                                        .px(px(4.))
                                        .py(px(2.))
                                        .bg(row_bg)
                                        .rounded(px(4.))
                                        .child(
                                            div()
                                                .w(px(32.))
                                                .text_xs()
                                                .text_color(if is_active {
                                                    cx.theme().ring
                                                } else {
                                                    cx.theme().muted_foreground
                                                })
                                                .child(format!("{:>2}", line_no)),
                                        )
                                        .child(
                                            div()
                                                .flex_1()
                                                .text_xs()
                                                .font_family(cx.theme().mono_font_family.clone())
                                                .child(line_text.to_string()),
                                        )
                                },
                            )),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .w(px(320.))
                            .h_full()
                            .p(px(16.))
                            .bg(cx.theme().secondary)
                            .child(
                                div()
                                    .text_sm()
                                    .font_bold()
                                    .pb(px(12.))
                                    .child("Execution Inspector"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.))
                                    .child(
                                        div()
                                            .flex()
                                            .justify_between()
                                            .text_xs()
                                            .child("Step:")
                                            .child(format!("{} / {}", step_idx + 1, total_steps)),
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
                                            .child(format!(
                                                "{:?}",
                                                current_pt
                                                    .map(|p| p.reason)
                                                    .unwrap_or(StopReason::Step)
                                            )),
                                    ),
                            ),
                    ),
            )
            .child(
                StatusBar::new()
                    .left(format!(
                        "File: {} | Ln {}",
                        self.buffer.file_path(),
                        current_line
                    ))
                    .right("GPUI Fast | Night Owl | M3"),
            )
    }
}
