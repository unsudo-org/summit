use dioxus::html::g::color;
use ::dioxus::prelude::*;
use ::std::sync;

use route::*;

mod cmp;
mod conf;
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
    use_context_provider(|| {
        conf::Conf {
            color: conf::ColorPalette {
                raisin_black: 0x1A191Eu32.into(),
                medium_slate_blue: 0x8B72E8u32.into(),
                rose_pompadour: 0xE87294u32.into(),
                aquamarine: 0x72E8C6u32.into(),
                mindaro: 0xCFE872u32.into(),
                timberwolf: 0xD2D3D2u32.into(),
                silver: 0xC1C1C1u32.into()
            },
            font: conf::FontPalette {
                br_cobane: "br cobane",
                stray: "stray",
                alien_skyline: "alien skyline"
            },
            cursor: conf::CursorAssetSheet {
                ban: asset!("/asset/icon/cursor/ban.svg"),
                click: asset!("/asset/icon/cursor/click.svg"),
                default: asset!("/asset/icon/cursor/default.svg"),
                disabled: asset!("/asset/icon/cursor/disabled.svg"),
                finger: asset!("/asset/icon/cursor/finger.svg"),
                finger_disabled: asset!("/asset/icon/cursor/finger_disabled.svg"),
                grip: asset!("/asset/icon/cursor/grip.svg"),
                grip_horizontal: asset!("/asset/icon/cursor/grip_horizontal.svg"),
                grip_vertical: asset!("/asset/icon/cursor/grip_vertical.svg"),
                hand: asset!("/asset/icon/cursor/hand.svg"),
                loading: asset!("/asset/icon/cursor/loading.svg"),
                move_diagonal_from_bottom_left: asset!("/asset/icon/cursor/move_diagonal_from_bottom_left.svg"),
                move_diagonal_from_top_left: asset!("/asset/icon/cursor/move_diagonal_from_top_left.svg"),
                move_horizontal: asset!("/asset/icon/cursor/move_horizontal.svg"),
                move_vertical: asset!("/asset/icon/cursor/move_vertical.svg"),
                square: asset!("/asset/icon/cursor/square.svg"),
                square_dashed: asset!("/asset/icon/cursor/square_dashed.svg"),
                text_input: asset!("/asset/icon/cursor/text_input.svg"),
                zoom_in: asset!("/asset/icon/cursor/zoom_in.svg"),
                zoom_out: asset!("/asset/icon/cursor/zoom_out.svg")
            }
        }
    });

    rsx!(
        document::Title { "Unsudo" }
        document::Link {
            rel: "icon",
            href: asset!("/asset/press-kit/logo.svg")
        }
        document::Stylesheet { href: asset!("/asset/css/keyframe/stripe/horizontal-stripe-motion.css") }
        document::Stylesheet { href: asset!("/asset/css/keyframe/stripe/vertical-stripe-motion.css") }
        document::Stylesheet { href: asset!("/asset/css/keyframe/faulty-neon.css") }
        document::Stylesheet { href: asset!("/asset/css/keyframe/float.css") }
        document::Stylesheet { href: asset!("/asset/css/keyframe/reveal.css") }
        document::Stylesheet { href: asset!("/asset/css/scrollbar/webkit-scrollbar-thumb-hover.css") }
        document::Stylesheet { href: asset!("/asset/css/scrollbar/webkit-scrollbar-thumb.css") }
        document::Stylesheet { href: asset!("/asset/css/scrollbar/webkit-scrollbar-track.css") }
        document::Stylesheet { href: asset!("/asset/css/scrollbar/webkit-scrollbar.css") }
        document::Stylesheet { href: asset!("/asset/css/color.css") }
        document::Stylesheet { href: asset!("/asset/css/cursor.css") }
        document::Stylesheet { href: asset!("/asset/css/reset.css") }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/alien-skyline" }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/stray" }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/br-cobane" }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/alien-android" }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/futurama" }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/sf-alien-encounters" }
        Router::<Route> {}
    )
}

fn main() {
    launch(Main);
}