//! Flux Inspector Panel
//! 
//! Right-side panel showing Flux-specific type information, bit slices,
//! struct layouts, and binary representations.

use gpui::{div, prelude::*, App, Context, Entity, InteractiveElement, Render, SharedString, Window};
use flux_parser::{flux_language, parse_flux, load_flux_queries, query_flux_ast, flux_language_config};
use language_core::LanguageConfig;

pub struct FluxInspectorPanel {
    focus_handle: gpui::FocusHandle,
}

impl FluxInspectorPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
        }
    }
}

impl Render for FluxInspectorPanel {
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
                    .child("Flux Inspector")
                    .text_xl()
                    .font_weight(gpui::FontWeight::MEDIUM)
            )
            .child(
                div()
                    .flex_1()
                    .p_4()
                    .overflow_auto()
                    .child("Place cursor on a Flux type to see:")
                    .text_sm()
                    .text_color(gpui::white().alpha(0.6))
            )
    }
}