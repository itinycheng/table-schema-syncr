use crate::util;

pub(crate) fn app_init() {
	

	let db_file = util::app_db_file();
	util::create_not_exist_file(&db_file).unwrap();

	let log_file = util::app_log_file();
	util::create_not_exist_file(&log_file).unwrap();
}
