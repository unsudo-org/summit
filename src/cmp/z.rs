use super::*;

#[component]
pub fn ZStack(children: Option<Element>) -> Element {
    rsx!(
        div {
            position: "relative",
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            min_width: "100%",
            max_width: "100%",
            min_height: "100%",
            max_height: "100%",
            overflow_x: "hidden",
            overflow_y: "hidden",
            { children }
        }
    )
}

#[component]
pub fn ZStackItem(z: usize, children: Option<Element>) -> Element {
    rsx!(
        div {
            position: "absolute",
            min_width: "100%",
            max_width: "100%",
            min_height: "100%",
            max_height: "100%",
            z_index: format!("{}", z),
            { children }
        }
    )
}