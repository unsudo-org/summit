use super::*;

#[component]
pub fn Grid(
    col_count: usize,
    row_count: usize,
    w: Option<String>,
    h: Option<String>,
    min_w: Option<String>,
    max_w: Option<String>,
    min_h: Option<String>,
    max_h: Option<String>,
    gap: Option<String>,
    children: Option<Element>
) -> Element {
    rsx!(
        div {
            display: "grid",
            grid_template_columns: format!("repeat({}, 1fr)", col_count),
            grid_template_rows: format!("repeat({}, 1fr)", row_count),
            width: w,
            height: h,
            min_width: min_w,
            max_width: max_w,
            min_height: min_h,
            max_height: max_h,
            gap: gap,
            { children }
        }
    )
}

#[component]
pub fn GridItem(
    from_x: usize,
    from_y: usize,
    to_x: usize,
    to_y: usize,
    children: Option<Element>
) -> Element {
    rsx!(
        div {
            grid_column_start: format!("{}", from_x),
            grid_column_end: format!("{}", to_x),
            grid_row_start: format!("{}", from_x),
            grid_row_end: format!("{}", to_y),
            { children }
        }
    )
}