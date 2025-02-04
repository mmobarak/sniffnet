//! SVG style

#![allow(clippy::module_name_repetitions)]

use iced::widget::svg::Appearance;

use crate::{get_colors, StyleType};

#[derive(Clone, Copy, Default)]
pub enum SvgType {
    AdaptColor,
    #[default]
    Standard,
}

impl iced::widget::svg::StyleSheet for StyleType {
    type Style = SvgType;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        Appearance {
            color: match style {
                SvgType::AdaptColor => Some(get_colors(*self).text_body),
                SvgType::Standard => None,
            },
        }
    }
}
