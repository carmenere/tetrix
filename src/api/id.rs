use std::{fmt::UpperHex, marker::PhantomData};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt::{self, Debug};

#[derive(Debug, Serialize, Clone)]
pub struct Id<I>
{
    id: I
}

impl<I> Id<I> {
    pub fn new(id: I) -> Self {
        Self {
            id
        }
    }
}

pub trait Entity {
    const ENTITY: &'static str;
}

pub trait Prefix {
    const PREFIX: &'static str;
    fn prefix() -> String {
        format!("{}_id-", Self::PREFIX)
    }
}

pub trait ParseId: Sized {
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError>;
}

impl ParseId for i64 {
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError> {
        dbg!(&s);
        Self::from_str_radix(s, radix)
    }
}

impl ParseId for u64 {
    fn from_str_radix(s: &str, radix: u32) -> Result<Self, ParseIntError> {
        dbg!(&s);
        Self::from_str_radix(s, radix)
    }
}

#[derive(Debug, Clone)]
pub struct Rid<I,T>
where
    T: Entity,
    I: std::fmt::Debug + UpperHex + FromStr<Err = ParseIntError> + Copy + ParseId
{
    pub id: I,
    _phantom: PhantomData<T>
}

impl<T,I> Rid<I,T>
where
    T: Entity,
    I: std::fmt::Debug + UpperHex + FromStr<Err = ParseIntError> + Copy + ParseId
{
    pub fn new(id: I) -> Self {
        Self {
            id: id,
            _phantom: PhantomData
        }
    }
}

impl<I,T> Prefix for Rid<I,T>
where
    T: Entity,
    I: std::fmt::Debug + UpperHex + FromStr<Err = ParseIntError> + Copy + ParseId
{
    const PREFIX: &'static str = T::ENTITY;
}

impl<I,T> From<I> for Rid<I,T>
where
    T: Entity,
    I: std::fmt::Debug + UpperHex + FromStr<Err = ParseIntError> + Copy + ParseId
{
    fn from(v: I) -> Self {
        Self::new(v)
    }
}

impl<I,T> Serialize for Rid<I,T>
where
    T: Entity,
    I: std::fmt::Debug + UpperHex + FromStr<Err = ParseIntError> + Copy + ParseId
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
    where
        S: Serializer
    {
        let s = format!("{}-{:X}", Self::PREFIX, self.id);
        serializer.serialize_str(&s)
    }
}

impl<'de,I,T> Deserialize<'de> for Rid<I,T>
where
    T: Entity,
    I: std::fmt::Debug + UpperHex + FromStr<Err = ParseIntError> + Copy + ParseId
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

impl<I,T> FromStr for Rid<I,T>
where
    T: Entity,
    I: std::fmt::Debug + UpperHex + FromStr<Err = ParseIntError> + Copy + ParseId
{
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, ParseIntError> {
        let id = match s.strip_prefix(&Self::prefix()) {
            Some(s) => I::from_str_radix(s, 16)?,
            None => I::from_str_radix(s, 10)?,
        };
        dbg!(&id);
        Ok(Rid::<I,T>::new(id))
    }
}