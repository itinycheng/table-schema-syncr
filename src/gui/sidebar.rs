use iced::{
	theme,
	widget::{button, container, row, text, Column, Container},
	Alignment, Length, Renderer,
};

use crate::{
	gui::style::icon::{delete_icon, edit_icon},
	store::conn_conf,
};

use super::{
	style::{border_style, sidebar_style},
	Message,
};

pub fn view<'a>() -> Container<'a, Message, Renderer> {
	let column_list = conn_conf::list_all()
		.unwrap()
		.iter()
		.enumerate()
		.fold(Column::new(), |base, col| {
			base.push(
				container(
					row![
						text(&col.1.name).size(24).width(Length::Fill),
						button(edit_icon())
							.style(theme::Button::Text)
							.on_press(Message::EditConnForm(Some(col.0))),
						button(delete_icon())
							.style(theme::Button::Text)
							.on_press(Message::CloseConnectionForm)
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
