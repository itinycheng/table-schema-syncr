use iced::{
	theme::{self, Container},
	widget::{container},
	Color, Theme,
};

pub fn border_style() -> theme::Container {
	pub struct Style;
	impl container::StyleSheet for Style {
		type Style = Theme;

		fn appearance(&self, _: &Self::Style) -> container::Appearance {
			container::Appearance {
				border_color: Color::BLACK,
				border_width: 1.0,
				..Default::default()
			}
		}
	}

	Container::Custom(Box::new(Style))
}
