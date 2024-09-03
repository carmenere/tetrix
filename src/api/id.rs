use std::{fmt::UpperHex, marker::PhantomData};
use serde::{de, Serialize, Serializer, Deserialize, Deserializer};
use std::num::ParseIntError;
use std::fmt::Debug;

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

pub trait IdPrefix {
    const ENTITY: &'static str;
    fn prefix() -> String {
        format!("{}_id-", Self::ENTITY)
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
    I: std::fmt::Debug + UpperHex + Copy + ParseId
{
    pub id: I,
    _phantom: PhantomData<T>
}

impl<T,I> Rid<I,T>
where
    T: Entity,
    I: std::fmt::Debug + UpperHex + Copy + ParseId
{
    pub fn new(id: I) -> Self {
        Self {
            id: id,
            _phantom: PhantomData
        }
    }
}

impl<I,T> IdPrefix for Rid<I,T>
where
    T: Entity,
    I: std::fmt::Debug + UpperHex + Copy + ParseId
{
    const ENTITY: &'static str = T::ENTITY;
}

impl<I,T> From<I> for Rid<I,T>
where
    T: Entity,
    I: std::fmt::Debug + UpperHex + Copy + ParseId
{
    fn from(v: I) -> Self {
        Self::new(v)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

impl<I,T> Serialize for Rid<I,T>
where
    T: Entity,
    I: std::fmt::Debug + UpperHex + Copy + ParseId
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
    where
        S: Serializer
    {
        let s = format!("{}{:X}", Self::prefix(), self.id);
        serializer.serialize_str(&s)
    }
}

impl<'de,T,I> Deserialize<'de> for Rid<I,T>
where
    T: Entity,
    I: std::fmt::Debug + UpperHex + Copy + ParseId + Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;

        let id = match s.strip_prefix(&Self::prefix()) {
            Some(s) => {
                I::from_str_radix(s, 16).map_err(de::Error::custom)
            },
            None => Err(de::Error::custom("Id must have prefix!")),
        };
        Ok(Rid::<I,T>::new(id?))
    }

}
