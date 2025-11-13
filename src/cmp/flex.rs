use super::*;

#[component]
pub fn FlexCenter(children: Element) -> Element {
    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "center",
            align_items: "center",
            { children }
        }
    )
}