use super::*;

#[component]
pub fn Button(
    children: Option<Element>
) -> Element {
    rsx!(
        button {
            style: format!(
                r#"
                
                "#
            ),
            { children }
        }
    )
}