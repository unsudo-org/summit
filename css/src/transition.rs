use super::*;

macro_rules! with_property {
    ($($property_fn:ident => $property:ident)*) => {
        $(
            pub fn $property_fn(mut self) -> Self {
                self.properties.push(Property::$property);
                self
            }
        )*
    };
}

pub struct Unset;

pub struct Param<A, B> {
    properties: Vec<Property>,
    duration: A,
    easing: B
}

impl<A, B> Param<A, B> {
    with_property!(
        all => All
        opacity => Opacity
        visibility => Visibility
        color => Color
        background_color => BackgroundColor
        background_position => BackgroundPosition
        background_size => BackgroundSize
        fill => Fill
        stroke => Stroke
        left => Left
        right => Right
        top => Top
        bottom => Bottom
        margin => Margin
        padding => Padding
        width => Width
        height => Height
        min_width => MinWidth
        max_width => MaxWidth
        min_height => MinHeight
        max_height => MaxHeight
        flex_grow => FlexGrow
        flex_shrink => FlexShrink
        grid_gap => GridGap
        gap => Gap
        transform => Transform
    );
}

impl Default for Param<Unset, Unset> {
    fn default() -> Self {
        Self {
            properties: vec![],
            duration: Unset,
            easing: Unset
        }
    }
}

impl<T> Param<Unset, T> {
    pub fn with_duration(self, duration: time::Duration) -> Param<time::Duration, T> {
        let Self {
            properties,
            easing,
            ..
        } = self;
        Param {
            properties,
            duration,
            easing
        }
    }
}

impl<T> Param<T, Unset> {
    pub fn with_easing(self, easing: Easing) -> Param<T, Easing> {
        let Self {
            properties,
            duration,
            ..
        } = self;
        Param {
            properties,
            duration,
            easing
        }
    }
}

impl fmt::Display for Param<time::Duration, Easing> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let ms: u128 = self.duration.as_millis();
        for (key, property) in self.properties.iter().enumerate() {
            if key > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{} {}ms {}", property, ms, self.easing)?;
        }
        Ok(())
    }
}


// MARK: Transition

pub struct Transition<A, B> {
    params: Vec<Param<A, B>>
}

impl<A, B> Transition<A, B> {
    pub fn add_param(mut self, param: impl Into<Param<A, B>>) -> Self {
        self.params.push(param.into());
        self
    }
}

impl<A, B> Default for Transition<A, B> {
    fn default() -> Self {
        Self {
            params: vec![]
        }
    }
}

impl<A, B> From<Vec<Param<A, B>>> for Transition<A, B> {
    fn from(value: Vec<Param<A, B>>) -> Self {
        let params: Vec<Param<A, B>> = value;
        Self {
            params
        }
    }
}

impl fmt::Display for Transition<time::Duration, Easing> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, param) in self.params.iter().enumerate() {
            if key > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", param)?;
        }
        Ok(())
    }
}


fn t() {
    Transition::default().add_param(
        Param::default()
            .background_color()
            .width()
            .grid_gap()
            .fill()
            .all()
            .with_duration(time::Duration::from_millis(20000))
            .with_easing(Easing::Linear)
    ).to_string();
}