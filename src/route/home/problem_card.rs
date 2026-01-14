use super::*;



#[component]
pub fn ProblemCardContainer(children: Option<Element>) -> Element {
    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            gap: "8px",
            { children }
        }
    )
}