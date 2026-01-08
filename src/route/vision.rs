use super::*;

#[component]
pub fn Vision() -> Element {
    let theme: theme::Theme = use_context();
    
    rsx!(
        cmp::Page {
            cmp::PageItem {
                cmp::navbar::Build {}
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "start",
                    align_items: "start",
                    cmp::typography::H1 {
                        span {
                            "Our vision for "
                        }
                        span {
                            cmp::typography::Gradient {
                                bg: format!(
                                    "linear-gradient(to bottom right, {}, {})",
                                    theme.color.success,
                                    theme.color.failure
                                ),
                                "Polkadot"
                            }
                        }
                    }
                }
            }
        }
    )
}