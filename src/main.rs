use ::dioxus::prelude::*;

mod cmp;

mod color {
    pub static RAISIN_BLACK: &str = "#1A191E";
    pub static MEDIUM_SLATE_BLUE: &str = "#8B72E8";
    pub static ROSE_POMPADOUR: &str = "#E87294";
    pub static AQUAMARINE: &str = "#72E8C6";
    pub static MINDARO: &str = "#CFE872";
    pub static TIMBERWOLF: &str = "#D2D3D2";
    pub static SILVER: &str = "#C1C1C1";
}

mod font {
    pub static ALIEN_SKYLINE: &str = "alien skyline";
    pub static STRAY: &str = "stray";
    pub static BR_COBANE: &str = "br cobane";
}

#[component]
fn Home() -> Element {
    rsx!(
        ::diogen::layout::Page {
            scroll_snap: diogen::layout::PageScrollSnap::Proximity,
            style: format!(
                r#"
                    background: {};
                "#,
                color::RAISIN_BLACK
            ),
            ::diogen::layout::PageItem {
                top: rsx!(),
                "Hello World"
            }
        }
    )
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Routable)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {}
}

#[component]
fn Main() -> Element {
    rsx!(
        document::Stylesheet { href: asset!("asset/css/keyframe/stripe/horizontal-stripe-motion.css") }
        document::Stylesheet { href: asset!("asset/css/keyframe/stripe/vertical-stripe-motion.css") }
        document::Stylesheet { href: asset!("asset/css/keyframe/faulty-neon.css") }
        document::Stylesheet { href: asset!("asset/css/keyframe/float.css") }
        document::Stylesheet { href: asset!("asset/css/keyframe/reveal.css") }
        document::Stylesheet { href: asset!("asset/css/scrollbar/webkit-scrollbar-thumb-hover.css") }
        document::Stylesheet { href: asset!("asset/css/scrollbar/webkit-scrollbar-thumb.css") }
        document::Stylesheet { href: asset!("asset/css/scrollbar/webkit-scrollbar-track.css") }
        document::Stylesheet { href: asset!("asset/css/scrollbar/webkit-scrollbar.css") }
        document::Stylesheet { href: asset!("asset/css/color.css") }
        document::Stylesheet { href: asset!("asset/css/reset.css") }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/alien-skyline" }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/stray" }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/br-cobane" }
        Router::<Route> {}
    )
}

fn main() {
    launch(Main);
}