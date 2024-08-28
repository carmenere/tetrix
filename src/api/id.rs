use std::fmt::UpperHex;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use std::str::FromStr;
use std::num::ParseIntError;

use super::{endpoints::arch::Arch, errors::AppError};

pub trait ResourceId {
    const PREFIX: &'static str;
    type Id: std::fmt::Debug + UpperHex + FromStr<Err = ParseIntError>;

    fn id(&self) -> Self::Id;
    fn new(id: Self::Id) -> Self;
    fn parse(s: &str) -> Result<Self::Id, ParseIntError>;
}

#[derive(Debug, Clone)]
pub struct Id<T>(pub T);

#[derive(Debug, Clone)]
pub struct Rid<T>(pub T);

impl<T> Rid<T>
where
    T: ResourceId
{
    fn new(rid: T) -> Self {
        Rid::<T>(rid)
    }
}

impl<T> Serialize for Rid<T>
where
    T: ResourceId
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
    where
        S: Serializer
    {
        let s = format!("{}-{:X}", T::PREFIX, self.0.id());
        serializer.serialize_str(&s)
    }
}

impl<'de, T> Deserialize<'de> for Rid<T>
where
    T: ResourceId
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        dbg!(&s);
        s.parse::<Self>().map_err(serde::de::Error::custom)
    }
}

impl<T> FromStr for Rid<T>
where
    T: ResourceId,
    T::Id: UpperHex + FromStr<Err = ParseIntError>
{
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let id = T::parse(s)?;
        dbg!(&id);
        Ok(Rid::<T>(T::new(id)))
    }
}