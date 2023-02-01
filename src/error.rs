pub type IResult<T> = core::result::Result<T, crate::error::IError>;

#[derive(Debug, thiserror::Error)]
pub enum IError {
	#[error("Access ClickHouse error: {0}")]
	CHError(#[from] clickhouse::error::Error),

	#[error("IO error: {0}")]
	IOError(#[from] std::io::Error),

	#[error("Access mysql error: {0:?}")]
	MysqlError(#[from] mysql::error::Error),

	#[error("Mysql url error: {0:?}")]
	MysqlUrlError(#[from] mysql::error::UrlError),

	#[error(transparent)]
	AnyError(#[from] anyhow::Error),
}
