#[derive(Clone, Debug)]
pub enum ParseError {
    Empty,
    Msg(String),
}

#[derive(Clone, Debug)]
pub enum EntityError {
    Parse(ParseError),
}
