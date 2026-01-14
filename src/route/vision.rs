use super::*;

#[component]
pub fn Vision() -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        component::page::Page {
            component::page::Item {
                component::navbar::Common {}
                div {
                    min_height: "32px"
                }
                component::Fieldset {
                    max_width: "512px",
                    label: rsx!(
                        component::typography::Gradient {
                            bg: format!(
                                "linear-gradient(to bottom right, {}, {})",
                                theme.color.success,
                                theme.color.highlight
                            ),
                            "Vision"
                        }
                    ),
                    component::typography::P {
                        text_align: "justify",
                        span { "We envision Unsudo as a fully self-sovereign, self-governing protocol, a network where people can build and deploy practical tools that feel simple, fast, and intuitive rather than heavy or restrictive." }
                        span { " " }
                        span { "Our goal is to enable anyone, anywhere, to collaborate and transact seamlessly in transparent, verifiable environments that protect freedom and privacy by design." }
                        span { " " }
                        span { "By reducing dependence on centralized intermediaries and lowering operational barriers, Unsudo empowers builders to create meaningful systems and services with less overhead." }
                        span { " " }
                        span { "The result is a more open, efficient, and sustainable digital ecosystem where innovation and governance evolve together." }
                    }
                    div {
                        display: "flex",
                        flex_direction: "row",
                        justify_content: "center",
                        min_width: "100%",
                        padding_top: "16px",
                        padding_bottom: "16px",
                        div {
                            min_width: "100%",
                            min_height: "2px",
                            background: format!("{}", theme.color.foreground),
                            border_radius: "2px"
                        }
                    }
                    component::typography::H4 { "Hello" }
                    component::typography::P {
                        span { "sjjj" }
                    }
                }
            }
        }
    )
}