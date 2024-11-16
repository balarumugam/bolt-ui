use web_sys::Element;
use wasm_bindgen::JsCast;
use crate::rsx;
use crate::rsx_internal;

pub enum TooltipPosition {
    Top,
    Bottom,
    Left,
    Right,
}

impl TooltipPosition {
    fn to_class(&self) -> &'static str {
        match self {
            TooltipPosition::Top => "tooltip-top",
            TooltipPosition::Bottom => "tooltip-bottom",
            TooltipPosition::Left => "tooltip-left",
            TooltipPosition::Right => "tooltip-right",
        }
    }
}

pub fn tooltip(content: &str, position: TooltipPosition, children: Element) -> Element {
    let position_class = position.to_class();
    
    rsx!(div {
        class = "tooltip-container",
        children,
        span {
            class = format!("tooltip {}", position_class),
            @content
        }
    })
}