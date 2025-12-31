use super::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
#[derive(Eq)]
pub enum Property {
    All,
    Opacity,
    Visibility,
    Color,
    BackgroundColor,
    BackgroundPosition,
    BackgroundSize,
    Fill,
    Stroke,
    Left,
    Right,
    Top,
    Bottom,
    Margin,
    Padding,
    Width,
    Height,
    MinWidth,
    MaxWidth,
    MinHeight,
    MaxHeight,
    FlexGrow,
    FlexShrink,
    GridGap,
    Gap,
    Transform,
    BorderWidth,
    BorderRadius,
    OutlineColor,
    OutlineWidth,
    OutlineOffset,
    BoxShadow,
    TextShadow,
    Filter
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: &'static str = match self {
            Self::All => "all",
            Self::Opacity => "opacity",
            Self::Visibility => "visibility",
            Self::Color => "color",
            Self::BackgroundColor =>"background-color",
            Self::BackgroundPosition => "background-position",
            Self::BackgroundSize => "background-size",
            Self::Fill => "fill",
            Self::Stroke => "stroke",
            Self::Left => "left",
            Self::Right => "right",
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Margin => "margin",
            Self::Padding => "padding",
            Self::Width => "width",
            Self::Height => "height",
            Self::MinWidth => "min-width",
            Self::MaxWidth => "max-width",
            Self::MinHeight => "min-height",
            Self::MaxHeight => "max-height",
            Self::FlexGrow => "flex-grow",
            Self::FlexShrink => "flex-shrink",
            Self::GridGap => "grid-gap",
            Self::Gap => "gap",
            Self::Transform => "transform",
            Self::BorderWidth => "border-width",
            Self::BorderRadius => "border-radius",
            Self::OutlineColor => "outline-color",
            Self::OutlineWidth => "outline-width",
            Self::OutlineOffset => "outline-offset",
            Self::BoxShadow => "box-shadow",
            Self::TextShadow => "text-shadow",
            Self::Filter => "filter"
        };
        write!(f, "{}", s)
    }
}