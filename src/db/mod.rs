use crate::gui::ConnectionParams;

pub fn save_conn_params(param: &ConnectionParams) {
	println!("{:?}", param);
}

#[cfg(test)]
mod tests {}
