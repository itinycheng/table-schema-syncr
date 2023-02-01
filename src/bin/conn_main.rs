use clickhouse::Row;
use schema_syncr::{
	conn::{ClickHouseClient, DBAccessor, DBCreator, DsParam, MysqlClient},
	error::IResult,
};
use serde::Deserialize;
use uuid::Uuid;

fn main() {
	test_mysql_conn();
	test_ch_conn();
}

// mysql
fn test_mysql_conn() {
	let ds = DsParam {
		uuid: Uuid::new_v4().to_string(),
		url: "mysql://admin:123456@127.0.0.1:3306/platform".to_string(),
		..Default::default()
	};

	let pool = MysqlClient::get_or_init(ds).unwrap();
	let vec: Vec<String> = pool.query_list("show databases").unwrap();
	println!("mysql: {:?}", vec);
}

// clickhouse
fn test_ch_conn() {
	#[derive(Row, Deserialize, Debug)]
	pub struct RowData {
		pub name: String,
	}

	let ds = DsParam {
		uuid: Uuid::new_v4().to_string(),
		url: "http://127.0.0.1:8123".to_string(),
		user: "admin".to_string(),
		password: "123456".to_string(),
		compression: "none".to_string(),
		..Default::default()
	};

	let client = ClickHouseClient::get_or_init(ds).unwrap();
	let vec: IResult<Option<RowData>> = client.query_one("show databases");
	println!("clickhouse: {:?}", vec);
}
