use super::*;

#[component]
pub fn Table(
    children: Option<Element>,

    #[props(extends = GlobalAttributes)]
    more: Vec<Attribute>
) -> Element {
    rsx!(
        table {
            ..more,
            { children }
        }
    )
}

#[component]
pub fn Row(
    children: Option<Element>
) -> Element {
    rsx!(
        tr {
            { children }
        }
    )
}

#[component]
pub fn Label(
    children: Option<Element>
) -> Element {
    let theme: theme::Theme = use_context();

    rsx!(
        th {
            background: format!("{}", theme.color.foreground),
            border_radius: "2px",
            padding: "1px",
            typography::H6 {
                color: format!("{}", theme.color.background),
                { children }
            }
        }
    )
}

#[component]
pub fn Item(
    children: Option<Element>
) -> Element {
    rsx!(
        td {
            typography::P {
                { children }
            }
        }
    )
}