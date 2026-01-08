use super::*;

#[component]
pub fn Glyph(
    top: Option<Element>,
    top_left: Option<Element>,
    top_right: Option<Element>,
    bottom: Option<Element>,
    bottom_left: Option<Element>,
    bottom_right: Option<Element>,
    left: Option<Element>,
    right: Option<Element>,
    children: Option<Element>,

    #[props(extends = GlobalAttributes)]
    more: Vec<Attribute>
) -> Element {
    rsx!(
        div {
            display: "grid",
            grid_template_columns: "1fr",
            grid_template_rows: "1fr",
            ..more,
            if let Some(children) = children {
                div {
                    grid_column_start: "1",
                    grid_column_end: "2",
                    grid_row_start: "1",
                    grid_row_end: "2",
                    { children }
                }
            }
            if let Some(top) = top {
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
                        { top }
                    }
                }
            }
            if let Some(top_left) = top_left {
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
                        { top_left }
                    }
                }
            }
            if let Some(top_right) = top_right {
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
                        { top_right }
                    }
                }
            }
            if let Some(bottom) = bottom {
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
                        { bottom }
                    }
                }       
            }
            if let Some(bottom_left) = bottom_left {
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
                        { bottom_left }
                    }
                }
            }
            if let Some(bottom_right) = bottom_right {
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
                        { bottom_right }
                    }
                }
            }
            if let Some(left) = left {
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
                        { left }
                    }
                }
            }
            if let Some(right) = right {
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
                        { right }
                    }
                }
            }
        }
    )
}

#[component]
pub fn Content(
    children: Option<Element>,

    #[props(extends = GlobalAttributes)]
    more: Vec<Attribute>
) -> Element {
    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "center",
            align_items: "center",
            ..more,
            { children }
        }
    )
}

#[component]
pub fn TopLeftAngle(
    border_width: String,
    border_style: String,
    border_color: String,
    border_radius: String,
    border_length: String,

    #[props(extends = GlobalAttributes)]
    more: Vec<Attribute>
) -> Element {
    rsx!(
        div {
            border_left_width: format!("{}", border_width),
            border_left_style: format!("{}", border_style),
            border_left_color: format!("{}", border_color),
            border_top_width: format!("{}", border_width),
            border_top_style: format!("{}", border_style),
            border_top_color: format!("{}", border_color),
            border_top_left_radius: format!("{}", border_radius),
            min_width: format!("{}", border_length),
            max_width: format!("{}", border_length),
            min_height: format!("{}", border_length),
            max_height: format!("{}", border_length),
            ..more
        }
    )
}

#[component]
pub fn TopRightAngle(
    border_width: String,
    border_style: String,
    border_color: String,
    border_radius: String,
    border_length: String,

    #[props(extends = GlobalAttributes)]
    more: Vec<Attribute>
) -> Element {
    rsx!(
        div {
            border_right_width: format!("{}", border_width),
            border_right_style: format!("{}", border_style),
            border_right_color: format!("{}", border_color),
            border_top_width: format!("{}", border_width),
            border_top_style: format!("{}", border_style),
            border_top_color: format!("{}", border_color),
            border_top_right_radius: format!("{}", border_radius),
            min_width: format!("{}", border_length),
            max_width: format!("{}", border_length),
            min_height: format!("{}", border_length),
            max_height: format!("{}", border_length),
            ..more
        }
    )
}

#[component]
pub fn BottomLeftAngle(
    border_width: String,
    border_style: String,
    border_color: String,
    border_radius: String,
    border_length: String,

    #[props(extends = GlobalAttributes)]
    more: Vec<Attribute>
) -> Element {
    rsx!(
        div {
            border_left_width: format!("{}", border_width),
            border_left_style: format!("{}", border_style),
            border_left_color: format!("{}", border_color),
            border_bottom_width: format!("{}", border_width),
            border_bottom_style: format!("{}", border_style),
            border_bottom_color: format!("{}", border_color),
            border_bottom_left_radius: format!("{}", border_radius),
            min_width: format!("{}", border_length),
            max_width: format!("{}", border_length),
            min_height: format!("{}", border_length),
            max_height: format!("{}", border_length),
            ..more
        }
    )
}

#[component]
pub fn BottomRightAngle(
    border_width: String,
    border_style: String,
    border_color: String,
    border_radius: String,
    border_length: String,

    #[props(extends = GlobalAttributes)]
    more: Vec<Attribute>
) -> Element {
    rsx!(
        div {
            border_right_width: format!("{}", border_width),
            border_right_style: format!("{}", border_style),
            border_right_color: format!("{}", border_color),
            border_bottom_width: format!("{}", border_width),
            border_bottom_style: format!("{}", border_style),
            border_bottom_color: format!("{}", border_color),
            border_bottom_right_radius: format!("{}", border_radius),
            min_width: format!("{}", border_length),
            max_width: format!("{}", border_length),
            min_height: format!("{}", border_length),
            max_height: format!("{}", border_length),
            ..more
        }
    )
}

#[component]
pub fn TopSegment(
    border_width: String,
    border_style: String,
    border_color: String,
    border_length: String,

    #[props(extends = GlobalAttributes)]
    more: Vec<Attribute>
) -> Element {
    rsx!(
        div {
            border_top_width: format!("{}", border_width),
            border_top_style: format!("{}", border_style),
            border_top_color: format!("{}", border_color),
            min_width: format!("{}", border_length),
            max_width: format!("{}", border_length),
            ..more
        }
    )
}

#[component]
pub fn BottomSegment(
    border_width: String,
    border_style: String,
    border_color: String,
    border_length: String,

    #[props(extends = GlobalAttributes)]
    more: Vec<Attribute>
) -> Element {
    rsx!(
        div {
            border_bottom_width: format!("{}", border_width),
            border_bottom_style: format!("{}", border_style),
            border_bottom_color: format!("{}", border_color),
            min_width: format!("{}", border_length),
            max_width: format!("{}", border_length),
            ..more
        }
    )
}

#[component]
pub fn LeftSegment(
    border_width: String,
    border_style: String,
    border_color: String,
    border_length: String,

    #[props(extends = GlobalAttributes)]
    more: Vec<Attribute>
) -> Element {
    rsx!(
        div {
            border_left_width: format!("{}", border_width),
            border_left_style: format!("{}", border_style),
            border_left_color: format!("{}", border_color),
            min_height: format!("{}", border_length),
            max_height: format!("{}", border_length)
        }
    )
}

#[component]
pub fn RightSegment(
    border_width: String,
    border_style: String,
    border_color: String,
    border_length: String,

    #[props(extends = GlobalAttributes)]
    more: Vec<Attribute>
) -> Element {
    rsx!(
        div {
            border_right_width: format!("{}", border_width),
            border_right_style: format!("{}", border_style),
            border_right_color: format!("{}", border_color),
            min_height: format!("{}", border_length),
            max_height: format!("{}", border_length)
        }
    )
}