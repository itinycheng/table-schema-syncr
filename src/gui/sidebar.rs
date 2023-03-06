use iced::{
	theme,
	widget::{button, container, row, text, Column, Container},
	Alignment, Length, Renderer,
};

use crate::gui::style::icon::{delete_icon, edit_icon};

use super::{
	style::{border_style, sidebar_style},
	App, Message,
};

pub fn view<'a>(app: &App) -> Container<'a, Message, Renderer> {
	let column_list = app
		.all_conns
		.iter()
		.enumerate()
		.fold(Column::new(), |base, col| {
			base.push(
				container(
					row![
						text(&col.1.name).size(24).width(Length::Fill),
						button(edit_icon())
							.style(theme::Button::Text)
							.on_press(Message::EditConnection(Some(col.0))),
						button(delete_icon())
							.style(theme::Button::Text)
							.on_press(Message::DeleteConnection(col.0))
					]
					.spacing(5),
				)
				.width(Length::Fill)
				.padding(5)
				.center_y()
				.style(sidebar_style()),
			)
		})
		.spacing(5)
		.width(Length::Fill)
		.height(Length::Fill)
		.align_items(Alignment::Center);

	container(column_list)
		.width(Length::FillPortion(1))
		.height(Length::Fill)
		.padding(5)
		.style(border_style())
}
