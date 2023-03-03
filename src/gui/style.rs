use iced::{
	theme::{self, Container},
	widget::container,
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

pub fn sidebar_style() -> theme::Container {
	pub struct Style;
	impl container::StyleSheet for Style {
		type Style = Theme;

		fn appearance(&self, _: &Self::Style) -> container::Appearance {
			container::Appearance {
				border_color: Color::TRANSPARENT,
				border_width: 1.0,
				background: Some(iced::Background::Color(Color::from_rgb(0.0, 0.9, 0.0))),
				..Default::default()
			}
		}
	}

	Container::Custom(Box::new(Style))
}
