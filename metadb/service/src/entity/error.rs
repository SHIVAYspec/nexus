#[derive(Clone, Debug)]
pub enum EntityParseError {
    Empty,
    Msg(String),
}

#[derive(Clone, Debug)]
pub enum DBError {
    Empty,
    Msg(String),
    Connection,
    Constraints,
}

pub type DBResult<T> = Result<T, DBError>;

#[derive(Clone, Debug)]
pub enum MetaDBError {
    Parse(EntityParseError),
    DB(DBError),
}
