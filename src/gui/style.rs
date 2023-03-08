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

pub fn sidebar_item_style(active: bool) -> theme::Button {
	if active {
		theme::Button::Primary
	} else {
		theme::Button::Secondary
	}
}

pub mod icon {
	use iced::{widget::Text, Font};

	const ICONS: Font =
		Font::External { name: "Icons", bytes: include_bytes!("../../fonts/icons.ttf") };

	pub fn edit_icon() -> Text<'static> {
		icon('\u{F303}')
	}

	pub fn delete_icon() -> Text<'static> {
		icon('\u{F1F8}')
	}

	fn icon(unicode: char) -> Text<'static> {
		iced::widget::text(unicode.to_string())
			.font(ICONS)
			.width(20)
			.horizontal_alignment(iced::alignment::Horizontal::Center)
			.size(20)
	}
}
