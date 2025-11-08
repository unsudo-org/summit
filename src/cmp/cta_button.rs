use super::*;

#[component]
pub fn CtaButton(
    class: Option<String>,
    style: Option<String>,
    children: Option<Element>
) -> Element {
    let mut hover: Signal<bool> = use_signal(|| false);

    rsx!(
        button {
            onmouseenter: move |_| *hover.write() = true,
            onmouseleave: move |_| *hover.write() = false,
            class: class,
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
                    cursor: url('{}'), auto;
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
                cursor::FINGER,
                if *hover.read() {
                    format!("linear-gradient(to bottom right, {})", color::SILVER)
                } else {
                    format!("linear-gradient(to bottom right, {}, {}, {})", color::MEDIUM_SLATE_BLUE, color::ROSE_POMPADOUR, color::MINDARO)
                },
                if *hover.read() {
                    format!("linear-gradient(to right bottom, {}, {}, {})", color::MEDIUM_SLATE_BLUE, color::ROSE_POMPADOUR, color::MINDARO)
                } else {
                    "transparent".to_owned()
                },
                style.unwrap_or_default()
            ),
            { children }
        }
    )
}