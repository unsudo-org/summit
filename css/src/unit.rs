pub trait PrefixExt {
    const PREFIX: &'static str;
}

macro_rules! prefix_ext_for {
    ($($flag:ident $prefix:literal)*) => {
        $(
            impl PrefixExt for $flag {
                const PREFIX: &'static str = $prefix;
            }
        )*
    };
}

prefix_ext_for!(
    Px "px"
    Cm "cm"
    Mm "mm"
    In "in"
    Pt "pt"
    Pc "pc"
    Percentage "%"
    Rem "rem"
    Em "em"
    Ex "ex"
    Ch "ch"
    Vw "vw"
    Vh "vh"
    Vmin "vmin"
    Vmax "vmax"
    Svw "svw"
    Svh "svh"
    Lvw "lvw"
    Lvh "lvh"
    Dvw "dvw"
    Dvh "dvh"
);


pub struct Px;

pub struct Cm;

pub struct Mm;

pub struct In;

pub struct Pt;

pub struct Pc;

pub struct Percentage;

pub struct Rem;

pub struct Em;

pub struct Ex;

pub struct Ch;

pub struct Vw;

pub struct Vh;

pub struct Vmin;

pub struct Vmax;

pub struct Svw;

pub struct Svh;

pub struct Lvw;

pub struct Lvh;

pub struct Dvw;

pub struct Dvh;


#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Unit<T> {
    mode: std::marker::PhantomData<T>,
    n: f32
}

macro_rules! unit_flag_constructor {
    ($($flag:ident $fn:ident)*) => {
        $(
            impl Unit<$flag> {
                pub fn new(n: f32) -> Self {
                    Self {
                        mode: std::marker::PhantomData,
                        n
                    }
                }
                
                pub fn $fn(n: f32) -> Self {
                    Self {
                        mode: std::marker::PhantomData,
                        n
                    }
                }
            }
        )*
    };
}

unit_flag_constructor!(
    Px from_px
    Cm from_cm
    Mm from_mm
    In from_in
    Pt from_pt
    Pc from_pc
    Percentage from_percentage
    Rem from_rem
    Em from_em
    Ex from_ex
    Ch from_ch
    Vw from_vw
    Vh from_vh
    Vmin from_vmin
    Vmax from_vmax
    Svw from_svw
    Svh from_svh
    Lvw from_lvw
    Lvh from_vvh
    Dvw from_dvw
    Dvh from_dvh
);

impl Unit<Percentage> {
    pub fn one_hundred_percent() -> Self {
        let n: f32 = 100.0;
        Self {
            mode: std::marker::PhantomData,
            n
        }
    }
}

impl Unit<Vw> {
    pub fn viewport_w() -> Self {
        let n: f32 = 100.0;
        Self {
            mode: std::marker::PhantomData,
            n
        }
    }

    pub fn viewport_h() -> Self {
        let n: f32 = 100.0;
        Self {
            mode: std::marker::PhantomData,
            n
        }
    }
}

impl<T> std::fmt::Display for Unit<T> 
where
    T: PrefixExt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.n, T::PREFIX)
    }
}