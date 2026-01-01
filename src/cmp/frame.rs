use super::*;

#[derive(Props, Clone, PartialEq)]
pub struct EightLaneScaffoldProps {
    pub top: Option<Element>,
    pub top_left: Option<Element>,
    pub top_right: Option<Element>,
    pub bottom: Option<Element>,
    pub bottom_left: Option<Element>,
    pub bottom_right: Option<Element>,
    pub left: Option<Element>,
    pub right: Option<Element>,
    pub children: Option<Element>
}

#[component]
pub fn EightLaneScaffold(props: EightLaneScaffoldProps) -> Element {
    rsx! {
        div {
            display: "grid",
            grid_template_columns: "1fr",
            grid_template_rows: "1fr",
            div {
                grid_column_start: "1",
                grid_column_end: "2",
                grid_row_start: "1",
                grid_row_end: "2",
                { props.children }
            }
            div {
                grid_column_start: "1",
                grid_column_end: "2",
                grid_row_start: "1",
                grid_row_end: "2",
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "start",
                    align_items: "center",
                    min_width: "100%",
                    max_width: "100%",
                    min_height: "100%",
                    max_height: "100%",
                    { props.top }
                }
            }
            div {
                grid_column_start: "1",
                grid_column_end: "2",
                grid_row_start: "1",
                grid_row_end: "2",
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "start",
                    align_items: "start",
                    min_width: "100%",
                    max_width: "100%",
                    min_height: "100%",
                    max_height: "100%",
                    { props.top_left }
                }
            }
            div {
                grid_column_start: "1",
                grid_column_end: "2",
                grid_row_start: "1",
                grid_row_end: "2",
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "start",
                    align_items: "end",
                    min_width: "100%",
                    max_width: "100%",
                    min_height: "100%",
                    max_height: "100%",
                    { props.top_right }
                }
            }
            div {
                grid_column_start: "1",
                grid_column_end: "2",
                grid_row_start: "1",
                grid_row_end: "2",
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "end",
                    align_items: "center",
                    min_width: "100%",
                    max_width: "100%",
                    min_height: "100%",
                    max_height: "100%",
                    { props.bottom }
                }
            }            
            div {
                grid_column_start: "1",
                grid_column_end: "2",
                grid_row_start: "1",
                grid_row_end: "2",
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "end",
                    align_items: "start",
                    min_width: "100%",
                    max_width: "100%",
                    min_height: "100%",
                    max_height: "100%",
                    { props.bottom_left }
                }
            }
            div {
                grid_column_start: "1",
                grid_column_end: "2",
                grid_row_start: "1",
                grid_row_end: "2",
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "end",
                    align_items: "end",
                    min_width: "100%",
                    max_width: "100%",
                    min_height: "100%",
                    max_height: "100%",
                    { props.bottom_right }
                }
            }
            div {
                grid_column_start: "1",
                grid_column_end: "2",
                grid_row_start: "1",
                grid_row_end: "2",
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "center",
                    align_items: "start",
                    min_width: "100%",
                    max_width: "100%",
                    min_height: "100%",
                    max_height: "100%",
                    { props.left }
                }
            }
            div {
                grid_column_start: "1",
                grid_column_end: "2",
                grid_row_start: "1",
                grid_row_end: "2",
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "center",
                    align_items: "end",
                    min_width: "100%",
                    max_width: "100%",
                    min_height: "100%",
                    max_height: "100%",
                    { props.right }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
pub struct AngledProps {
    pub border_width: usize,
    pub border_color: Hex,
    pub border_style: &'static str,
    pub border_length: usize,
    pub border_radius: usize,
    pub padding: usize,
    pub children: Option<Element>
}

#[component]
pub fn Angled(props: AngledProps) -> Element {
    rsx! {
        EightLaneScaffold {
            top_left: rsx! {
                div {
                    border_left_width: format!("{}", props.border_width),
                    border_left_style: format!("{}", props.border_style),
                    border_left_color: format!("{}", props.border_color),
                    border_top_width: format!("{}", props.border_width),
                    border_top_style: format!("{}", props.border_style),
                    border_top_color: format!("{}", props.border_color),
                    border_top_left_radius: format!("{}", props.border_radius),
                    min_width: format!("{}", props.border_length),
                    max_width: format!("{}", props.border_length),
                    min_height: format!("{}", props.border_length),
                    max_height: format!("{}", props.border_length)
                }
            },
            top_right: rsx! {
                div {
                    border_right_width: format!("{}", props.border_width),
                    border_right_style: format!("{}", props.border_style),
                    border_right_color: format!("{}", props.border_color),
                    border_top_width: format!("{}", props.border_width),
                    border_top_style: format!("{}", props.border_style),
                    border_top_color: format!("{}", props.border_color),
                    border_top_right_radius: format!("{}", props.border_radius),
                    min_width: format!("{}", props.border_length),
                    max_width: format!("{}", props.border_length),
                    min_height: format!("{}", props.border_length),
                    max_height: format!("{}", props.border_length)
                }
            },
            bottom_left: rsx! {
                div {
                    border_left_width: format!("{}", props.border_width),
                    border_left_style: format!("{}", props.border_style),
                    border_left_color: format!("{}", props.border_color),
                    border_bottom_width: format!("{}", props.border_width),
                    border_bottom_style: format!("{}", props.border_style),
                    border_bottom_color: format!("{}", props.border_color),
                    border_bottom_left_radius: format!("{}", props.border_radius),
                    min_width: format!("{}", props.border_length),
                    max_width: format!("{}", props.border_length),
                    min_height: format!("{}", props.border_length),
                    max_height: format!("{}", props.border_length)
                }
            },
            bottom_right: rsx! {
                div {
                    border_right_width: format!("{}", props.border_width),
                    border_right_style: format!("{}", props.border_style),
                    border_right_color: format!("{}", props.border_color),
                    border_bottom_width: format!("{}", props.border_width),
                    border_bottom_style: format!("{}", props.border_style),
                    border_bottom_color: format!("{}", props.border_color),
                    border_bottom_right_radius: format!("{}", props.border_radius),
                    min_width: format!("{}", props.border_length),
                    max_width: format!("{}", props.border_length),
                    min_height: format!("{}", props.border_length),
                    max_height: format!("{}", props.border_length)
                }
            },
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "center",
                align_items: "center",
                padding: props.padding,
                { props.children }
            }
        }
    }
}