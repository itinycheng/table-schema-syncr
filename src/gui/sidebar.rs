use iced::{
	alignment::Vertical,
	widget::{container, text, Column, Container},
	Alignment, Length, Renderer,
};

use crate::store::conn_conf;

use super::{
	style::{border_style, sidebar_style},
	Message,
};

pub fn view<'a>() -> Container<'a, Message, Renderer> {
	let column_list = conn_conf::list_all()
		.unwrap()
		.iter()
		.fold(Column::new(), |base, col| {
			base.push(
				container(text(&col.name))
					.width(Length::Fill)
					.height(30)
					.padding(5)
					.align_y(Vertical::Center)
					.style(sidebar_style()),
			)
		})
		.width(Length::Fill)
		.height(Length::Fill)
		.align_items(Alignment::Center);

	container(column_list)
		.width(Length::FillPortion(1))
		.height(Length::Fill)
		.padding(5)
		.style(border_style())
}
