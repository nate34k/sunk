#![macro_use]

use json::{self, Value};
use json::value::{Map, Index};

use error;

macro_rules! fetch {
    ($j:ident->$i:ident: $t:ident) => (
        $j[stringify!($i)].$t().ok_or(
            Error::ParseError(stringify!(failed parsing $i.$t))
        )?
    );
    ($j:ident->$i:ident: $t:ident, $u:ty) => (
        fetch!($j->$i: $t).parse::<$u>().map_err(
            |_| Error::ParseError(stringify!(not a $u))
        )?
    );
}

macro_rules! fetch_maybe {
    ($j:ident->$i:ident: $t:ident) => {
        $j[stringify!($i)].$t()
    };
    ($j:ident->$i:ident: $t:ident, $u:ty) => {
        match fetch_maybe!($j->$i: $t).map(|v| v.parse::<$u>().map_err(
            |_| Error::ParseError(stringify!(not a $u))
        )) {
            Some(Ok(v)) => Some(v),
            _ => None
        }
    }
}

macro_rules! pointer {
    ($json:ident, $path:expr) => (
        $json.pointer($path)
            .ok_or(Error::ParseError(stringify!(nothing found at $path)))?
    )
}

macro_rules! impl_cover_art {
    () => {
        pub fn cover_art(&self, sunk: &mut Sunk, size: Option<u64>) -> Result<String> {
            let args = Query::new()
                .arg("id", self.id)
                .maybe_arg("size", size)
                .build();
            sunk.try_binary("getCoverArt", args)
        }
    }
}

pub(crate) fn map_str<T>(v: Option<T>) -> Option<String>
where
    T: ::std::string::ToString
{
    v.map(|v| v.to_string())
}

fn map_some_vec<T, U, F>(
    sv: Option<Vec<T>>, mut f: F
) -> Option<Vec<U>>
where
    F: FnMut(&T) -> U,
{
    sv.map(|v| {
        v.iter().map(|n| {
            f(n)
        }).collect::<Vec<U>>()
    })
}

pub(crate) fn map_vec_string<T>(sv: Option<Vec<T>>) -> Option<Vec<String>>
where
    T: ::std::string::ToString,
{
    map_some_vec(sv, T::to_string)
}

pub trait ValueExt {
    fn try_get<I: Index>(&self, index: I) -> error::Result<&Value>;
    fn try_array(&self) -> error::Result<Vec<Value>>;
    fn try_map(&self) -> error::Result<Map<String, Value>>;
}

impl ValueExt for Value {
    fn try_get<I: Index>(&self, index: I) -> error::Result<&Value> {
        self.get(index)
            .ok_or(error::Error::JsonError(format!("missing index in {}", self)))
    }

    fn try_array(&self) -> error::Result<Vec<Value>> {
        self.as_array()
            .ok_or(error::Error::JsonError(format!("{} not an array", self)))
            .map(|a| a.clone())
    }

    fn try_map(&self) -> error::Result<Map<String, Value>> {
        self.as_object()
            .ok_or(error::Error::JsonError(format!("{} not a map", self)))
            .map(|m| m.clone())
    }
}