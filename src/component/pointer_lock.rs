use super::*;

#[component]
pub fn PointerLock(locked: bool, children: Option<Element>) -> Element {
    rsx!(
        div {
            display: "contents",
            pointer_events: if locked {
                "none"
            } else {
                "auto"
            },
            { children }
        }
    )
}