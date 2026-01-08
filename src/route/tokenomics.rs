use super::*;

#[component]
pub fn Tokenomics() -> Element {
    rsx!(
        cmp::Page {
            cmp::PageItem {
                div {
                    display: "flex",
                    flex_direction: "column",
                    justify_content: "start",
                    align_items: "center",
                    min_width: "100%",
                    max_width: "100%",
                    div {
                        display: "flex",
                        flex_direction: "column",
                        justify_content: "start",
                        align_items: "start",
                        max_width: "600px",
                        gap: "16px",
                        cmp::Fieldset {
                            label: rsx!("Tokenomics"),
                            cmp::typography::P { "The maximum UDO supply is capped at 1,000,000 tokens. Token distribution and governance participation are structured to align incentives and long-term protocol health." }
                            div {
                                min_height: "8px"
                            }
                            cmp::table::Table {
                                cmp::table::Row {
                                    cmp::table::Label { "Pool" }
                                    cmp::table::Label { "Balance" }
                                    cmp::table::Label { "Price ($)" }
                                    cmp::table::Label { "Inflow" }
                                    cmp::table::Label { "Outflow" }
                                    cmp::table::Label { "Vesting" }
                                }
                                div {
                                    min_height: "16px"
                                }
                                cmp::table::Row {
                                    cmp::table::Item { "Council" }
                                    cmp::table::Item { "200,000" }
                                    cmp::table::Item { "25" }
                                    cmp::table::Item { "_" }
                                    cmp::table::Item { "5,000,000" }
                                    cmp::table::Item { "20 years" }
                                }
                                cmp::table::Row {
                                    cmp::table::Item { "Core Contributors "}
                                    cmp::table::Item { "75,000" }
                                    cmp::table::Item { "25" }
                                    cmp::table::Item { "_" }
                                    cmp::table::Item { "1,875,000" }
                                    cmp::table::Item { "5 years" }
                                }
                                cmp::table::Row {
                                    cmp::table::Item { "Bounty" }
                                    cmp::table::Item { "50,000" }
                                    cmp::table::Item { "25" }
                                    cmp::table::Item { "_" }
                                    cmp::table::Item { "1,250,000" }
                                    cmp::table::Item { "5 years" }
                                }
                                cmp::table::Row {
                                    cmp::table::Item { "Pre-Seed" }
                                    cmp::table::Item { "20,000" }
                                    cmp::table::Item { "25" }
                                    cmp::table::Item { "50,000" }
                                    cmp::table::Item { "_" }
                                    cmp::table::Item { "_" }
                                }
                                cmp::table::Row {
                                    cmp::table::Item { "Seed" }
                                    cmp::table::Item { "30,000" }
                                    cmp::table::Item { "5" }
                                    cmp::table::Item { "150,000" }
                                    cmp::table::Item { "_" }
                                    cmp::table::Item { "_" }
                                }
                                cmp::table::Row {
                                    cmp::table::Item { "Series A" }
                                    cmp::table::Item { "40,000" }
                                    cmp::table::Item { "10" }
                                    cmp::table::Item { "400,000" }
                                    cmp::table::Item { "_" }
                                    cmp::table::Item { "_" }
                                }
                                cmp::table::Row {
                                    cmp::table::Item { "Series B" }
                                    cmp::table::Item { "50,000" }
                                    cmp::table::Item { "17.5" }
                                    cmp::table::Item { "875,000" }
                                    cmp::table::Item { "_" }
                                    cmp::table::Item { "_" }
                                }
                                cmp::table::Row {
                                    cmp::table::Item { "Series C" }
                                    cmp::table::Item { "60,000" }
                                    cmp::table::Item { "22" }
                                    cmp::table::Item { "1,320,000" }
                                    cmp::table::Item { "_" }
                                    cmp::table::Item { "_" }
                                }
                                cmp::table::Row {
                                    cmp::table::Item { "Public" }
                                    cmp::table::Item { "100,000" }
                                    cmp::table::Item { "25" }
                                    cmp::table::Item { "2,500,000" }
                                    cmp::table::Item { "_" }
                                    cmp::table::Item { "3 months" }
                                }
                                cmp::table::Row {
                                    cmp::table::Item { "Liquidity" }
                                    cmp::table::Item { "50,000" }
                                    cmp::table::Item { "25" }
                                    cmp::table::Item { "_" }
                                    cmp::table::Item { "1,250,000" }
                                    cmp::table::Item { "_" }
                                }
                                cmp::table::Row {
                                    cmp::table::Item { "Reserve" }
                                    cmp::table::Item { "325,000" }
                                    cmp::table::Item { "25" }
                                    cmp::table::Item { "_" }
                                    cmp::table::Item { "8,750,000" }
                                    cmp::table::Item { "_" }
                                }
                            }
                        }
                        cmp::Fieldset {
                            label: rsx!("Vesting"),
                            cmp::typography::P { "Vesting schedules are linear, unlocking per second from the Token Generation Event (TGE). Algorithmic governance adjusts unlock rates based on protocol performance and market conditions. This ensures fairness, protects against market volatility, and aligns long-term stakeholder incentives." }
                        }
                        cmp::Fieldset {
                            label: rsx!("Liquidity Planning"),
                            cmp::typography::P { "Unsudo prioritizes decentralization and initially plans to list on decentralized exchanges (DEXs). A reserve of 50,000 UDO tokens is allocated for liquidity provisioning." }
                        }
                    }
                }
            }
        }
    )
}