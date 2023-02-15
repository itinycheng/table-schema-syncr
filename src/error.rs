pub type IResult<T> = core::result::Result<T, crate::error::IError>;

#[derive(Debug, thiserror::Error)]
pub enum IError {
	#[error("Access ClickHouse error: {0}")]
	CHError(#[from] clickhouse::error::Error),

	#[error("IO error: {0}")]
	IOError(#[from] std::io::Error),

	#[error("Access mysql error: {0}")]
	MysqlError(#[from] mysql::error::Error),

	#[error("Mysql url error: {0}")]
	MysqlUrlError(#[from] mysql::error::UrlError),

	#[error("Sqlite error: {0}")]
	SqliteError(#[from] rusqlite::Error),

	#[error("Iced error: {0}")]
	IcedError(#[from] iced::Error),

	#[error("Prompt error: {0}")]
	PromptError(String),

	#[error(transparent)]
	AnyError(#[from] anyhow::Error),
}
