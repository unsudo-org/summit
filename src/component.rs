//! Components

use super::*;

pub mod glyph;
pub mod navbar;
pub mod logo;
pub mod icon;
pub mod page;
pub mod footer;

modwire::expose!(
    pub button
    pub counter
    pub grid
    pub fieldset
    pub fill
    pub hazard_stripe
    pub shape
);

pub mod typography;
pub mod table;
pub mod effect;

#[component]
pub fn To(
    to: &'static str,
    children: Option<Element>
) -> Element {
    rsx!(
        Link {
            to,
            display: "contents",
            { children }
        }
    )
}

#[component]
pub fn FingerDisabled(children: Option<Element>) -> Element {
    let theme: theme::Theme = use_context();
    
    rsx!(
        div {
            cursor: format!("url('{}'), auto", theme.cursor.finger_disabled),
            pointer_events: "auto",
            div {
                pointer_events: "none",
                { children }
            }
        }
    )
}

#[component]
pub fn Ban(children: Option<Element>) -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        div {
            cursor: format!("url('{}'), auto", theme.cursor.ban),
            pointer_events: "auto",
            div {
                pointer_events: "none",
                { children }
            }
        }
    )
}