use super::*;

#[component]
pub fn Icon(
    url: Asset,
    w: String,
    class: Option<String>,
    style: Option<String>
) -> Element {
    rsx!(
        div {
            style: format!(
                r#"
                    min-width: {};
                    aspect-ratio: 1 / 1;
                    background-image: url({});
                    background-position: center;
                    background-size: contain;
                    background-repeat: no-repeat;
                    color: {};
                    {}
                "#,
                w,
                url,
                conf::SILVER,
                style.unwrap_or_default()
            )
        }
    )
}