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
                "Hello World"
            }
            cmp::PageItem {
                "Hello World"
            }
        }
    )
}