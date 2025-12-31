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
    #[props(default = None)]
    scroll_snap: Option<PageScrollSnap>,

    #[props(default = None)]
    surface: Option<Element>,

    #[props(default = None)]
    children: Option<Element>,

    #[props(extends = GlobalAttributes)]
    attr: Vec<Attribute>
) -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        div {
            display: "grid",
            grid_template_columns: "1fr",
            grid_template_rows: "1fr",
            min_width: "100vw",
            max_width: "100vw",
            min_height: "100vh",
            max_height: "100vh",
            overflow_x: "hidden",
            overflow_y: "hidden",
            background: format!("{}", theme.color.background),
            cursor: format!("url('{}'), auto", theme.cursor.default),
            ..attr,
            if let Some(surface) = surface {
                div {
                    grid_column_start: "1",
                    grid_column_end: "2",
                    grid_row_start: "1",
                    grid_row_end: "2",
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
            },
            div {
                grid_column_start: "1",
                grid_column_end: "2",
                grid_row_start: "1",
                grid_row_end: "2",
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "start",
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
pub fn PageItem(
    #[props(default = None)]
    children: Option<Element>
) -> Element {
    rsx!(
        div {
            style: "scroll_snap_align: start",
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            min_width: "100vw",
            max_width: "100vw",
            min_height: "100vh",
            max_height: "100vh",
            div {
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
        }
    )
}