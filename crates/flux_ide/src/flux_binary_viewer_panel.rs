//! Flux Binary Viewer Panel
//! 
//! Panel for viewing binary files (ELF, raw binaries, firmware images)
//! with Flux struct overlays.

use gpui::{div, prelude::*, App, Context, Render, Window};

pub struct FluxBinaryViewerPanel {
    focus_handle: gpui::FocusHandle,
}

impl FluxBinaryViewerPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Render for FluxBinaryViewerPanel {
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
                    .child("Flux Binary Viewer")
                    .text_xl()
                    .font_weight(gpui::FontWeight::MEDIUM),
            )
            .child(
                div()
                    .flex_1()
                    .p_4()
                    .overflow_auto()
                    .child("Open an ELF/binary file to inspect with Flux structs")
                    .text_sm()
                    .text_color(gpui::white().alpha(0.6)),
            )
    }
}