use super::*;

#[component]
pub fn AutoGrid(
    #[props(default = 1)]
    col_count: usize,

    #[props(default = 1)]
    row_count: usize,

    #[props(default = None)]
    children: Option<Element>,

    #[props(extends = GlobalAttributes)]
    attr: Vec<Attribute>
) -> Element {
    rsx!(
        div {
            display: "grid",
            grid_template_columns: format!("repeat({}, 1fr)", col_count),
            grid_template_rows: format!("repeat({}, 1fr)", row_count),
            ..attr,
            { children }
        }
    )
}

#[component]
pub fn GridItem(
    #[props(default = 0)]
    from_x: usize,

    #[props(default = 0)]
    from_y: usize,

    #[props(default = 1)]
    to_x: usize,

    #[props(default = 1)]
    to_y: usize,

    #[props(default = None)]
    children: Option<Element>,

    #[props(extends = GlobalAttributes)]
    attr: Vec<Attribute>
) -> Element {
    let from_x: usize = from_x + 1;
    let from_y: usize = from_y + 1;
    let to_x: usize = to_x + 1;
    let to_y: usize = to_y + 1;
    rsx!(
        div {
            grid_column_start: format!("{}", from_x),
            grid_column_end: format!("{}", to_x),
            grid_row_start: format!("{}", from_y),
            grid_row_end: format!("{}", to_y),
            ..attr,
            { children }
        }
    )
}