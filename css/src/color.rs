use super::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
pub struct Hex {
    code: u32
}

impl Default for Hex {
    fn default() -> Self {
        0x000000u32.into()
    }
}

impl<T> From<T> for Hex 
where
    T: Into<u32> {
    fn from(value: T) -> Self {
        let code: u32 = value.into();
        Self {
            code
        }
    }
}

impl ops::Deref for Hex {
    type Target = u32;
    
    fn deref(&self) -> &Self::Target {
        &self.code
    }
}

impl ops::DerefMut for Hex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.code
    }
}

impl fmt::Display for Hex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{:06X}", self.code)
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8
}

impl Rgb {
    pub fn r(self) -> u8 {
        self.r
    }

    pub fn g(self) -> u8 {
        self.b
    }

    pub fn b(self) -> u8 {
        self.g
    }
}

impl Default for Rgb {
    fn default() -> Self {
        let r: u8 = 0;
        let g: u8 = 0;
        let b: u8 = 0;
        Self {
            r,
            g,
            b
        }
    }
}

impl<A, B, C> From<(A, B, C)> for Rgb
where
    A: Into<u8>,
    B: Into<u8>,
    C: Into<u8> {
    fn from(value: (A, B, C)) -> Self {
        let r: u8 = value.0.into();
        let g: u8 = value.1.into();
        let b: u8 = value.2.into();
        Self {
            r,
            g,
            b
        }
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r: u8 = self.r;
        let g: u8 = self.g;
        let b: u8 = self.b;
        write!(f, "rgb({}, {}, {})", r, g, b)
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub struct Rgba {
    r: u8,
    g: u8,
    b: u8,
    a: f32
}

impl Rgba {
    pub fn r(self) -> u8 {
        self.r
    }

    pub fn g(self) -> u8 {
        self.g
    }

    pub fn b(self) -> u8 {
        self.b
    }

    pub fn a(self) -> f32 {
        self.a
    }
}

impl Default for Rgba {
    fn default() -> Self {
        let r: u8 = 0;
        let g: u8 = 0;
        let b: u8 = 0;
        let a: f32 = 0.0;
        Self {
            r,
            g,
            b,
            a
        }
    }
}

impl<A, B, C, D> From<(A, B, C, D)> for Rgba
where
    A: Into<u8>,
    B: Into<u8>,
    C: Into<u8>,
    D: Into<f32> {
    fn from(value: (A, B, C, D)) -> Self {
        let r: u8 = value.0.into();
        let g: u8 = value.1.into();
        let b: u8 = value.2.into();
        let a: f32 = value.3.into();
        Self {
            r,
            g,
            b,
            a
        }
    }
}

impl fmt::Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r: u8 = self.r;
        let g: u8 = self.g;
        let b: u8 = self.b;
        let a: f32 = self.a;
        write!(f, "rgba({}, {}, {}, {})", r, g, b, a)
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
pub enum Color {
    Hex(Hex),
    Rgb(Rgb),
    Rgba(Rgba)
}

impl Default for Color {
    fn default() -> Self {
        Self::Hex(Hex::default())
    }
}

impl From<Hex> for Color {
    fn from(value: Hex) -> Self {
        Self::Hex(value)
    }
}

impl From<Rgb> for Color {
    fn from(value: Rgb) -> Self {
        Self::Rgb(value)
    }
}

impl From<Rgba> for Color {
    fn from(value: Rgba) -> Self {
        Self::Rgba(value)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hex(hex) => write!(f, "{}", hex),
            Self::Rgb(rgb) => write!(f, "{}", rgb),
            Self::Rgba(rgba) => write!(f, "{}", rgba)
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
pub struct ColorStop {
    color: Color,
    position: Option<f32>
}

impl From<Color> for ColorStop {
    fn from(value: Color) -> Self {
        let color: Color = value;
        Self {
            color,
            position: None
        }
    }
}

impl From<(Color, f32)> for ColorStop {
    fn from(value: (Color, f32)) -> Self {
        let color: Color = value.0;
        let position: f32 = value.1;
        let position: Option<f32> = Some(position);
        Self {
            color,
            position
        }
    }
}

impl fmt::Display for ColorStop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(position) = self.position {
            write!(f, "{} {}%", self.color, position * 100.0)
        } else {
            write!(f, "{}", self.color)
        }
    }
}