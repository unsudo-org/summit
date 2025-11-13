use super::*;

#[repr(u8)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum PageScrollSnap {
    Mandatory,
    Proximity
}

#[component]
pub fn Page(
    scroll_snap: Option<PageScrollSnap>, 
    surface: Option<Element>,
    color: Option<conf::Hex>,
    children: Option<Element>
) -> Element {
    rsx!(
        div {
            position: "relative",
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "center",
            min_width: "100vw",
            max_width: "100vw",
            min_height: "100vh",
            max_height: "100vh",
            overflow_x: "hidden",
            overflow_y: "hidden",
            cursor: format!("url('{}'), auto", cursor::DEFAULT),
            background: color,
            if let Some(surface) = surface {
                div {
                    position: "absolute",
                    z_index: "6767676767676767",
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
                    pointer_events: "none",
                    { surface }
                }
            }
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "center",
                min_width: "100%",
                max_width: "100%",
                overflow_x: "hidden",
                overflow_y: "auto",
                scroll_behavior: "smooth",
                scroll_snap_type: match scroll_snap {
                    Some(PageScrollSnap::Mandatory) => "y mandatory",
                    Some(PageScrollSnap::Proximity) => "y proximity",
                    None => "none"
                },
                { children }
            }
        }
    )
}

#[component]
pub fn PageItem(children: Option<Element>) -> Element {
    rsx!(
        div {
            style: "scroll-snap-align: start;",
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            min_width: "100vw",
            max_width: "100vw",
            min_height: "100vh",
            max_height: "100vh",
            { children }
        }
    )
}