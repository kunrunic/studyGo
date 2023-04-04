// trait OrError {
//     type Output;
//     fn or_error<F>(self, f: F) -> Self::Output
//     where
//         F: FnOnce() -> ErrorCode;
// }

// impl<T> OrError for Option<T> {
//     type Output = Try<T>;

//     fn or_error<F>(self, f: F) -> Self::Output
//     where
//         F: FnOnce() -> ErrorCode,
//     {
//         match self {
//             Some(v) => Ok(v),
//             None => Err(f()),
//         }
//     }
// }

use std::fmt::Error;

use bytes::Bytes;

trait OrError {
    type Output;
    fn or_error<F>(self, f: F) -> Self::Output
    where
        //   F: FnOnce() -> ErrorCode;
        F: FnOnce() -> Error;
}

impl<T> OrError for Option<T> {
    type Output = Result<T, Error>;

    fn or_error<F>(self, f: F) -> Self::Output
    where
        F: FnOnce() -> Error,
    {
        match self {
            Some(v) => Ok(v),
            None => Err(f()),
        }
    }
}

pub trait ComBuf {
    type ComBufType;
    fn to_own(t: Self::ComBufType) -> Self::ComBufType;
}

impl ComBuf for Bytes {
    type ComBufType = Bytes;
    fn to_own(t: Self::ComBufType) -> Self::ComBufType {
        t
    }
}

impl ComBuf for Vec<u8> {
    type ComBufType = Vec<u8>;
    fn to_own(t: Self::ComBufType) -> Self::ComBufType {
        t
    }
}

fn main() -> Result<(), String> {
    let s = "test;test1;test2;test3";
    let sp: Vec<&str> = s.split(';').collect();
    let v = *sp.get(1).ok_or_else(|| format!("invalid config : {}", s))?;
    println!("{:?}", v);
    let v = sp.get(1).or_error(|| Error::from(Error::default())); // error 를 구현하던지.. 바꾸면 원하는 유형으로 남겨줄수도있음..
    println!("{:?}", v);
    Ok(())
}
