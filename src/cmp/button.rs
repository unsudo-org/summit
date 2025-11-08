use super::*;

#[component]
pub fn Button(
    class: Option<String>,
    style: Option<String>,
    children: Option<Element>
) -> Element {
    let mut hover: Signal<bool> = use_signal(|| false);

    rsx!(
        button {
            class: class,
            onmouseenter: move |_| *hover.write() = true,
            onmouseleave: move |_| *hover.write() = false,
            style: format!(
                r#"
                    all: unset;
                    display: flex;
                    flex-direction: row;
                    justify-content: center;
                    align-items: center;
                    font-size: 1em;
                    font-family: {};
                    font-weight: normal;
                    color: {};
                    cursor: pointer;
                    border-width: 1px;
                    border-style: solid;
                    border-image: {} 1;
                    border-radius: 2px;
                    padding: 8px;
                    background: {};
                    {}
                "#,
                font::BR_COBANE,
                if *hover.read() {
                    color::RAISIN_BLACK
                } else {
                    color::SILVER
                },
                format!("linear-gradient(to top right, {})", color::SILVER),
                if *hover.read() {
                    color::SILVER
                } else {
                    "transparent"
                },
                style.unwrap_or_default()
            ),
            { children }
        }
    )
}