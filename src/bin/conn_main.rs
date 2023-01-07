use clickhouse::Row;
use schema_syncr::{
	conn::{DBAccessor, DBCreator, DsParam, DBClient},
	error::IResult, database::{DB_CLICK_HOUSE, DB_MYSQL},
};
use serde::Deserialize;

fn main() {
	test_mysql_conn();
	test_ch_conn();
}

// mysql
fn test_mysql_conn() {
	let ds = DsParam {
		uuid: "mysql".to_string(),
		url: "mysql://".to_string(),
		..Default::default()
	};

	let pool = DBClient::<DB_MYSQL>::get_or_init(ds).unwrap();
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
		uuid: "ch".to_string(),
		url: "http://127.0.0.1:8123".to_string(),
		user: "".to_string(),
		password: "".to_string(),
		compression: "none".to_string(),
		..Default::default()
	};

	let client = DBClient::<DB_CLICK_HOUSE>::get_or_init(ds).unwrap();
	let vec: IResult<Option<RowData>> = client.query_one("show databases");
	println!("clickhouse: {:?}", vec);
}
