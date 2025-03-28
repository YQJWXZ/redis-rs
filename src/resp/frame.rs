use bytes::BytesMut;
use enum_dispatch::enum_dispatch;

use super::{
    array::{RespArray, RespNullArray},
    bulk_string::{BulkString, RespNullBulkString},
    map::RespMap,
    null::RespNull,
    set::RespSet,
    simple_error::SimpleError,
    simple_string::SimpleString,
};

use crate::{RespDecode, RespError};

#[enum_dispatch(RespEncode)]
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(BulkString),
    NullBulkString(RespNullBulkString),
    Array(RespArray),
    Null(RespNull),
    NullArray(RespNullArray),
    Boolean(bool),
    Double(f64),
    Map(RespMap),
    Set(RespSet),
}

impl RespDecode for RespFrame {
    const PREFIX: &'static str = "";
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError> {
        let mut iter = buf.iter().peekable();
        match iter.peek() {
            Some(b'+') => Ok(SimpleString::decode(buf)?.into()),
            Some(b'-') => Ok(SimpleError::decode(buf)?.into()),
            Some(b':') => Ok(i64::decode(buf)?.into()),
            Some(b'$') => {
                // try null bulk string first
                match RespNullBulkString::decode(buf) {
                    Ok(frame) => Ok(frame.into()),
                    Err(RespError::NotComplete) => Err(RespError::NotComplete),
                    Err(_) => Ok(BulkString::decode(buf)?.into()),
                }
            }
            Some(b'*') => {
                // try null array first
                match RespNull::decode(buf) {
                    Ok(frame) => Ok(frame.into()),
                    Err(RespError::NotComplete) => Err(RespError::NotComplete),
                    Err(_) => Ok(RespArray::decode(buf)?.into()),
                }
            }
            Some(b'_') => Ok(RespNull::decode(buf)?.into()),
            Some(b'#') => Ok(bool::decode(buf)?.into()),
            Some(b',') => Ok(f64::decode(buf)?.into()),
            Some(b'%') => Ok(RespMap::decode(buf)?.into()),
            Some(b'~') => Ok(RespSet::decode(buf)?.into()),
            None => Err(RespError::NotComplete),
            _ => Err(RespError::InvalidFrameType(format!(
                "expect_length: unknown frame type: {:?}",
                buf
            ))),
        }
    }
    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let mut iter = buf.iter().peekable();
        match iter.peek() {
            Some(b'+') => Ok(SimpleString::expect_length(buf)?),
            Some(b'-') => Ok(SimpleError::expect_length(buf)?),
            Some(b':') => Ok(i64::expect_length(buf)?),
            Some(b'$') => Ok(BulkString::expect_length(buf)?),
            Some(b'*') => Ok(RespArray::expect_length(buf)?),
            Some(b'_') => Ok(RespNull::expect_length(buf)?),
            Some(b'#') => Ok(bool::expect_length(buf)?),
            Some(b',') => Ok(f64::expect_length(buf)?),
            Some(b'%') => Ok(RespMap::expect_length(buf)?),
            Some(b'~') => Ok(RespSet::expect_length(buf)?),
            _ => Err(RespError::NotComplete),
        }
    }
}

impl From<&str> for RespFrame {
    fn from(s: &str) -> Self {
        SimpleString(s.to_string()).into()
    }
}

impl From<&[u8]> for RespFrame {
    fn from(value: &[u8]) -> Self {
        BulkString(value.to_vec()).into()
    }
}

impl<const N: usize> From<&[u8; N]> for RespFrame {
    fn from(value: &[u8; N]) -> Self {
        BulkString(value.to_vec()).into()
    }
}

#[cfg(test)]
mod tests {
    // TODO: Add tests
}
