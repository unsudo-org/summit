use ::dioxus::prelude::*;

use route::*;

mod cmp;
mod color;
mod font;
mod route;

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