use dioxus::html::g::font_weight;
use dioxus::prelude::*;
use std::time;
use std::sync;
use std::rc;
use std::cell;
use gloo_timers::callback as gloo_callback;
use gloo_timers::future as gloo_future;
use kore::color;
use kore::color::CommonExt as _;

use route::*;

mod component;
mod theme;
mod route;

pub type Hex = ::kore::color::Hex<2, u128>;

#[rustfmt::skip]
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Routable)]
enum Route {
    #[route("/")]
    Home {},
    
    #[route("/vision")]
    Vision {},

    #[route("/tokenomics")]
    Tokenomics {}
}

#[component]
fn Main() -> Element {

    use_context_provider(|| {
        theme::Theme {
            color: theme::Color {
                foreground: 0xD2D3D2u32.into(),
                background: 0x1A191Eu32.into(),
                success: 0x72E8C6u32.into(),
                failure: 0xE87294u32.into(),
                warning: 0xCFE872u32.into(),
                highlight: 0x8B72E8u32.into()
            },
            font: theme::Font {
                regular: "br cobane",
                body: "brulia test",
                display: "beyonders",
                monospace: "alien skyline"
            },
            cursor: theme::Cursor {
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
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/strike-fighter" }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/darken-2" }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/beyonders" }
        document::Stylesheet { href: "https://fonts.cdnfonts.com/css/brulia-test" }
        Router::<Route> {}
    )
}

fn main() {
    launch(Main);
}




pub trait Css
where
    Self: std::fmt::Display {
    fn px(self) -> String;
    fn vw(self) -> String;
    fn vh(self) -> String;
    fn pc(self) -> String;
}

impl<T> Css for T 
where
    T: std::fmt::Display {
    fn px(self) -> String {
        format!("{}px", self)
    }

    fn vw(self) -> String {
        format!("{}vw", self)
    }

    fn vh(self) -> String {
        format!("{}vh", self)
    }

    fn pc(self) -> String {
        format!("{}%", self)
    }
}




mod unit {

    macro_rules! n_impl {
        ($($n:ident $n_fn:ident $v:expr)*) => {
            $(
                impl Unit<$n> {
                    pub fn new() -> Self {
                        Self($n)
                    }

                    pub fn $n_fn() -> Self {
                        Self($n)
                    }
                }

                impl Unit<$n> {
                    pub fn to_int(&self) -> u32 {
                        $v
                    }
                }

                impl Default for Unit<$n> {
                    fn default() -> Self {
                        Self($n)
                    }
                }

                impl std::fmt::Display for Unit<$n> {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, "{}", self.to_int())
                    }
                }
            )*
        };
    }

    pub struct N2;
    pub struct N4;
    pub struct N8;
    pub struct N16;
    pub struct N32;
    pub struct N64;
    pub struct N128;
    pub struct N256;
    pub struct N512;
    pub struct N1024;

    #[derive(Debug)]
    #[derive(Clone)]
    #[derive(Copy)]
    #[derive(PartialEq)]
    #[derive(Eq)]
    pub struct Unit<T>(T);

    n_impl!(
        N2 from_n2 2
        N4 from_n4 4
        N8 from_n8 8
        N16 from_n16 16
        N32 from_n32 32
        N64 from_n64 64
        N128 from_n128 128
        N256 from_n256 256
        N512 from_n512 512
        N1024 from_n1024 1024
    );

    impl Unit<u32> {
        pub fn new(n: u32) -> Self {
            Self(n)
        }

        pub fn from_n(n: u32) -> Self {
            Self(n)
        }
    }

    impl Unit<u32> {
        pub fn to_int(&self) -> u32 {
            self.0
        }
    }

    impl std::fmt::Display for Unit<u32> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.to_int())
        }
    }
}