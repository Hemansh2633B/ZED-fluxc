//! Flux Build Panel
//! 
//! Bottom panel showing build dashboard, project analyzer, and compilation metrics.

use gpui::{div, prelude::*, App, Context, Render, Window};

pub struct FluxBuildPanel {
    focus_handle: gpui::FocusHandle,
}

impl FluxBuildPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Render for FluxBuildPanel {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(gpui::white().alpha(0.02))
            .child(
                div()
                    .p_4()
                    .border_b_1()
                    .border_color(gpui::white().alpha(0.1))
                    .child("Flux Build Dashboard")
                    .text_xl()
                    .font_weight(gpui::FontWeight::MEDIUM),
            )
            .child(
                div()
                    .flex_1()
                    .p_4()
                    .overflow_auto()
                    .child("Build metrics, compilation times, and project analysis")
                    .text_sm()
                    .text_color(gpui::white().alpha(0.6)),
            )
    }
}