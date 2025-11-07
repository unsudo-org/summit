use super::*;

#[component]
pub fn CtaButton(
    children: Option<Element>
) -> Element {
    let hover: Signal<bool> = use_signal(|| false);

    rsx!(
        button {
            onmouseenter: {
                let mut hover: Signal<_> = hover.to_owned();
                move |_| {
                    *hover.write() = true;
                }
            },
            onmouseleave: {
                let mut hover: Signal<_> = hover.to_owned();
                move |_| {
                    *hover.write() = false;
                }
            },
            style: format!(
                r#"
                    all: unset;
                    display: flex;
                    flex-direction: column;
                    justify-content: center;
                    align-items: center;
                    font-size: 1em;
                    font-family: {};
                    font-weight: normal;
                    color: {};
                    cursor: pointer;
                    border-width: 1px;
                    border-style: solid;
                    border-image: linear-gradient(
                        to top right, 
                        {}, 
                        {}
                    ) 1;
                    border-radius: 2px;
                    padding: 8px;
                    background: {};
                "#,
                font::BR_COBANE,
                if *hover.read() {
                    color::RAISIN_BLACK
                } else {
                    color::SILVER
                },
                if *hover.read() {
                    color::MEDIUM_SLATE_BLUE
                } else {
                    color::SILVER
                },
                color::SILVER,
                if *hover.read() {
                    format!("linear-gradient(to right bottom, {}, {})", color::MEDIUM_SLATE_BLUE, color::ROSE_POMPADOUR)
                } else {
                    "transparent".to_owned()
                }
            ),
            { children }
        }
    )
}