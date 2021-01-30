use crate::*;

pub trait Decode<'a>: Sized {
    fn decode(contents: &'a Vec<u8>) -> Option<Self>;
}

pub trait Encode {
    fn encode(&self) -> Vec<u8>;
}

impl Encode for &str {
    fn encode(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl<'a> Decode<'a> for &'a str {
    fn decode(contents: &'a Contents) -> Option<Self> {
        match std::str::from_utf8(contents.as_slice()) {
            Ok(x) => Some(x),
            _ => None,
        }
    }
}

impl Encode for i64 {
    fn encode(&self) -> Vec<u8> {
        format!("{}", self).into_bytes()
    }
}

impl<'a> Decode<'a> for i64 {
    fn decode(contents: &'a Contents) -> Option<Self> {
        match std::str::from_utf8(contents.as_slice()) {
            Ok(x) => match x.parse::<i64>() {
                Ok(x) => Some(x),
                _ => None,
            },
            _ => None,
        }
    }
}

impl Encode for bool {
    fn encode(&self) -> Vec<u8> {
        if *self {
            b"true".to_vec()
        } else {
            b"false".to_vec()
        }
    }
}

impl<'a> Decode<'a> for bool {
    fn decode(contents: &'a Contents) -> Option<Self> {
        match std::str::from_utf8(contents.as_slice()) {
            Ok("true") => Some(true),
            Ok("false") => Some(false),
            _ => None,
        }
    }
}
