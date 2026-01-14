use super::*;

#[component]
pub fn Governance() -> Element {
    rsx!(
        component::page::Page {
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "start",
                min_height: "100vw",
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "start",
                    align_items: "start",
                    min_width: "100%",
                    max_width: "100%",
                    component::typography::P { "Unsudo introduces next-generation DAO models that leverage algorithms for unbiased decision-making, contrasting traditional organizational structures. Governance is designed to be largely autonomous, mathematically deterministic, and resilient, minimizing human error while aligning incentives across the community and protocol participants." }
                }
            }   
        }
    )
}