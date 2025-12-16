use super::*;

#[component]
pub fn Spacer(
    min_w: Option<String>,
    max_w: Option<String>,
    min_h: Option<String>,
    max_h: Option<String>,
    fill: Option<Hex>
) -> Element {
    rsx!(
        div {
            display: "flex",
            flex_direction: "row",
            justify_content: "center",
            align_items: "center",
            min_width: min_w,
            max_width: max_w,
            min_height: min_h,
            max_height: max_h,
            background: if let Some(fill) = fill {
                fill.to_string()
            }
        }
    )
}