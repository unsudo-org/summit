use super::*;

#[component]
pub fn Tokenomics() -> Element {
    let theme: theme::Theme = use_context();
    rsx!(
        component::page::Page {
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "start",
                min_width: 100.pc(),
                min_height: 100.vh(),
                component::navbar::Common {}
                div {
                    min_height: 32.px()
                }
                Section {}
            }
        }
    )
}

#[component]
fn Section() -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        div {
            display: "flex",
            flex_direction: "column",
            justify_content: "start",
            align_items: "center",
            min_width: 100.pc(),
            max_width: 100.pc(),
            div {
                display: "flex",
                flex_direction: "row",
                flex_wrap: "wrap",
                justify_content: "start",
                align_items: "start",
                gap: 16.px(),
                Block {
                    component::Fieldset {
                        label: rsx!(
                            component::typography::Gradient {
                                bg: format!(
                                    "linear-gradient(to bottom right, {}, {})",
                                    theme.color.success,
                                    theme.color.highlight
                                ),
                                "Governance"
                            }
                        ),
                        component::typography::P { "Unsudo introduces next-generation DAO models that leverage algorithms for unbiased decision-making, contrasting traditional organizational structures. Governance is designed to be largely autonomous, mathematically deterministic, and resilient, minimizing human error while aligning incentives across the community and protocol participants." }
                    }
                    component::Fieldset {
                        label: rsx!("Dual Chamber System"),
                        component::typography::P { "Much like a government, Unsudo operates with a dual-chamber system. The community, holding UDO tokens, can vote on proposals that directly alter protocol code or authorize fund transfers. The council serves as a representative body with oversight functions. It cannot pass proposals independently; instead, it ensures community-backed decisions and holds temporary veto power over malicious actions. Over time, as the DAO matures, the council’s role diminishes, with governance becoming entirely community-driven." }
                    }
                }
                Block {
                    component::Fieldset {
                        label: rsx!(
                            component::typography::Gradient {
                                bg: format!(
                                    "linear-gradient(to bottom right, {}, {})",
                                    theme.color.success,
                                    theme.color.highlight
                                ),
                                "Tokenomics"
                            }
                        ),
                        component::typography::P { "The maximum UDO supply is capped at 1,000,000 tokens. Token distribution and governance participation are structured to align incentives and long-term protocol health." }
                        div {
                            min_height: "8px"
                        }
                        component::table::Table {
                            component::table::Row {
                                component::table::Label { "Pool" }
                                component::table::Label { "Balance" }
                                component::table::Label { "Price ($)" }
                                component::table::Label { "Inflow" }
                                component::table::Label { "Outflow" }
                                component::table::Label { "Vesting" }
                            }
                            div {
                                min_height: "16px"
                            }
                            component::table::Row {
                                component::table::Item { "Council" }
                                component::table::Item { "200,000" }
                                component::table::Item { "25" }
                                component::table::Item { "_" }
                                component::table::Item { "5,000,000" }
                                component::table::Item { "20 years" }
                            }
                            component::table::Row {
                                component::table::Item { "Core Contributors "}
                                component::table::Item { "75,000" }
                                component::table::Item { "25" }
                                component::table::Item { "_" }
                                component::table::Item { "1,875,000" }
                                component::table::Item { "5 years" }
                            }
                            component::table::Row {
                                component::table::Item { "Bounty" }
                                component::table::Item { "50,000" }
                                component::table::Item { "25" }
                                component::table::Item { "_" }
                                component::table::Item { "1,250,000" }
                                component::table::Item { "5 years" }
                            }
                            component::table::Row {
                                component::table::Item { "Pre-Seed" }
                                component::table::Item { "20,000" }
                                component::table::Item { "25" }
                                component::table::Item { "50,000" }
                                component::table::Item { "_" }
                                component::table::Item { "_" }
                            }
                            component::table::Row {
                                component::table::Item { "Seed" }
                                component::table::Item { "30,000" }
                                component::table::Item { "5" }
                                component::table::Item { "150,000" }
                                component::table::Item { "_" }
                                component::table::Item { "_" }
                            }
                            component::table::Row {
                                component::table::Item { "Series A" }
                                component::table::Item { "40,000" }
                                component::table::Item { "10" }
                                component::table::Item { "400,000" }
                                component::table::Item { "_" }
                                component::table::Item { "_" }
                            }
                            component::table::Row {
                                component::table::Item { "Series B" }
                                component::table::Item { "50,000" }
                                component::table::Item { "17.5" }
                                component::table::Item { "875,000" }
                                component::table::Item { "_" }
                                component::table::Item { "_" }
                            }
                            component::table::Row {
                                component::table::Item { "Series C" }
                                component::table::Item { "60,000" }
                                component::table::Item { "22" }
                                component::table::Item { "1,320,000" }
                                component::table::Item { "_" }
                                component::table::Item { "_" }
                            }
                            component::table::Row {
                                component::table::Item { "Public" }
                                component::table::Item { "100,000" }
                                component::table::Item { "25" }
                                component::table::Item { "2,500,000" }
                                component::table::Item { "_" }
                                component::table::Item { "3 months" }
                            }
                            component::table::Row {
                                component::table::Item { "Liquidity" }
                                component::table::Item { "50,000" }
                                component::table::Item { "25" }
                                component::table::Item { "_" }
                                component::table::Item { "1,250,000" }
                                component::table::Item { "_" }
                            }
                            component::table::Row {
                                component::table::Item { "Reserve" }
                                component::table::Item { "325,000" }
                                component::table::Item { "25" }
                                component::table::Item { "_" }
                                component::table::Item { "8,750,000" }
                                component::table::Item { "_" }
                            }
                        }
                    }
                    component::Fieldset {
                        label: rsx!("Vesting"),
                        component::typography::P { "Vesting schedules are linear, unlocking per second from the Token Generation Event (TGE). Algorithmic governance adjusts unlock rates based on protocol performance and market conditions. This ensures fairness, protects against market volatility, and aligns long-term stakeholder incentives." }
                    }
                    component::Fieldset {
                        label: rsx!("Liquidity Planning"),
                        component::typography::P { "Unsudo prioritizes decentralization and initially plans to list on decentralized exchanges (DEXs). A reserve of 50,000 UDO tokens is allocated for liquidity provisioning." }
                    }
                }
                Block {
                    component::Fieldset {
                        label: rsx!(
                            component::typography::Gradient {
                                bg: format!(
                                    "linear-gradient(to bottom right, {}, {})",
                                    theme.color.success,
                                    theme.color.highlight
                                ),
                                "Role"
                            }
                        ),
                        component::typography::P {  }
                    }
                }
            }
        }
    )
}

#[component]
fn Block(
    children: Option<Element>
) -> Element {
    rsx!(
        component::glyph::Angled {
            div {
                display: "flex",
                flex_direction: "column",
                justify_content: "start",
                align_items: "start",
                gap: "8px",
                max_width: "600px",
                { children }
            }
        }
    )
}