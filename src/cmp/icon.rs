use super::*;

#[component]
pub fn Icon(
    w: String,
    url: Asset,
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
                color::SILVER,
                style.to_owned().unwrap_or_default()
            )
        }
    )
}