mod style;

use iced::{widget::container, Color, Theme};

pub struct MyStyle;

impl container::StyleSheet for MyStyle {
	type Style = Theme;

	fn appearance(&self, _: &Self::Style) -> container::Appearance {
		container::Appearance {
			border_color: Color::BLACK,
			border_width: 1.0,
			..Default::default()
		}
	}
}
