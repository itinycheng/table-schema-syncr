use clickhouse::Row;
use schema_syncr::{
	conn::{DBClient, DBParam, DBQuery},
	database::{DbType, DB_CLICK_HOUSE, DB_MYSQL},
};
use serde::Deserialize;
use uuid::Uuid;

fn main() {
	test_mysql_conn();
	test_ch_conn();
}

// mysql
fn test_mysql_conn() {
	let ds = DBParam {
		uuid: Uuid::new_v4().to_string(),
		db_type: DbType::MySQL,
		url: "mysql://admin:123456@127.0.0.1:3306/platform".to_string(),
		..Default::default()
	};

	let client = DBClient::get_or_init(ds).unwrap();
	let vec =
		<DBClient as DBQuery<DB_MYSQL, std::string::String>>::query_list(&client, "show databases");
	println!("mysql: {:?}", vec);
}

// clickhouse
fn test_ch_conn() {
	#[derive(Row, Deserialize, Debug)]
	pub struct RowData {
		pub name: String,
	}

	let ds = DBParam {
		uuid: Uuid::new_v4().to_string(),
		db_type: DbType::ClickHouse,
		url: "http://127.0.0.1:8123".to_string(),
		user: "admin".to_string(),
		password: "123456".to_string(),
		compression: "none".to_string(),
		..Default::default()
	};

	let client = DBClient::get_or_init(ds).unwrap();
	let vec = <DBClient as DBQuery<DB_CLICK_HOUSE, std::string::String>>::query_one(
		&client,
		"show databases",
	);
	println!("clickhouse: {:?}", vec);
}
