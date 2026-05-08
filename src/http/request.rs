use super::method::Method;
use std::str;
use std::error::Error;
use std::convert::TryFrom;
use std::str::Utf8Error;
use std::fmt::{Formatter, Display, Debug, Result as FmtResult};
pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}
impl TryFrom<&[u8]> for Request{
    type Error = ParseError;

    fn try_from(buffer: &[u8]) -> Result<Self, Self::Error> {
        match str::from_utf8(buffer).or(Err(ParseError::InvalidEncoding)){
            Ok(request_str)=>{

            },
            Err(e)=>return Err(e)
        }

        let result = str::from_utf8(buffer)?;
        unimplemented!()
    }
}

pub enum ParseError{
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod
}



impl ParseError{
    fn message(&self) -> &str{
        match self{
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Method",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.message())
    }
}

impl Debug for ParseError{
    fn fmt(&self, f : &mut Formatter) -> FmtResult{
        write!(f,"{}",self.message())
    }
}

impl From<Utf8Error> for ParseError{
    fn from(value: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
impl Error for ParseError{

}
