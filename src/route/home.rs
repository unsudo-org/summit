use super::*;

#[component]
pub fn Home() -> Element {
    rsx!(
        cmp::Page {
            scroll_snap: cmp::PageScrollSnap::Proximity,
            style: format!(
                r#"
                    background: {};
                "#,
                color::RAISIN_BLACK
            ),
            cmp::PageItem {
                style: format!(
                    r#"
                        justify-content: space-between;
                        padding-left: 16px;
                        padding-right: 16px;
                        padding-top: 16px;
                    "#
                ),
                div {
                    style: format!(
                        r#"
                            display: flex;
                            flex-direction: column;
                            justify-content: center;
                            align-items: center;
                            min-width: 100%;
                            max-width: 100%;
                        "#
                    ),
                    cmp::NavbarBuild {}
                }
                div {
                    style: format!(
                        r#"
                            display: flex;
                            flex-direction: column;
                            justify-content: center;
                            align-items: center;
                            min-width: 100%;
                            max-width: 100%;
                            flex: 1;
                        "#
                    ),
                    div {
                        style: format!(
                            r#"
                                display: flex;
                                flex-direction: column;
                                justify-content: center;
                                align-items: center;
                                font-size: 4em;
                                font-family: br cobane;
                                font-weight: normal;
                                color: white;
                                padding: 32px;
                                line-height: 1.75em;
                            "#
                        ),
                        "Layer 1.5 Build on top of Polkadot JAM"
                    }
                    cmp::Shape {
                        w: 400.0,
                        h: 400.0,
                        model: cmp::ShapeModel::random(),
                        color: color::ROSE_POMPADOUR
                    }
                    cmp::Shape {
                        w: 250.0,
                        h: 250.0,
                        model: cmp::ShapeModel::random(),
                        color: color::ROSE_POMPADOUR
                    }
                    cmp::Shape {
                        w: 250.0,
                        h: 250.0,
                        model: cmp::ShapeModel::random(),
                        color: color::ROSE_POMPADOUR
                    }
                }
            }
            cmp::PageItem {
                "Hello World"
            }
        }
    )
}