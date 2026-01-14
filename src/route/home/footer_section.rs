#[component]
fn FooterSection() -> Element {
    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "start",
            min_width: "100%",
            max_width: "100%",
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "start",
                min_width: "100%",
                max_width: "100%",
                padding: "8px",
                
            }
        }
    )
}