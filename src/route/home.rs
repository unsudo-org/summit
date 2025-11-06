use super::*;

#[component]
pub fn Home() -> Element {
    rsx!(
        cmp::Page {
            scroll_snap: cmp::PageScrollSnap::Proximity,
            style: format!(
                r#"
                    background: {};
                "#,
                color::RAISIN_BLACK
            ),
            cmp::PageItem {
                div {
                    style: format!(
                        r#"
                            display: flex;
                            flex-direction: column;
                            justify-content: center;
                            align-items: center;
                            font-size: 4em;
                            font-family: br cobane;
                            font-weight: normal;
                            color: white;
                            padding: 32px;
                            line-height: 1.75em;
                        "#
                    ),
                    "Layer 1.5 Build on top of Polkadot JAM"
                }
            }
            cmp::PageItem {
                "Hello World"
            }
        }
    )
}