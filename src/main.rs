pub mod column;
pub mod conf;
pub mod conn;
pub mod database;
pub mod datastore;
pub mod error;
pub mod table;
pub mod util;

#[tokio::main]
async fn main() {
	conf::app_init();
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {}
}
