use super::gpu::WINDOW_SIZE;
use serde::{
    de::{Error, SeqAccess, Visitor},
    ser::SerializeTuple,
    Deserializer, Serializer,
};
use std::fmt;

pub fn serialize<S: Serializer>(mask: &[bool], serializer: S) -> Result<S::Ok, S::Error> {
    let mut tuple = serializer.serialize_tuple(WINDOW_SIZE)?;
    for value in mask {
        tuple.serialize_element(value)?;
    }
    tuple.end()
}

pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Box<[bool]>, D::Error> {
    struct MaskVisitor;
    impl<'de> Visitor<'de> for MaskVisitor {
        type Value = Box<[bool]>;
        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{} object-window pixels", WINDOW_SIZE)
        }
        fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
            let mut mask = Vec::with_capacity(WINDOW_SIZE);
            for i in 0..WINDOW_SIZE {
                mask.push(
                    seq.next_element()?
                        .ok_or_else(|| A::Error::invalid_length(i, &self))?,
                );
            }
            Ok(mask.into_boxed_slice())
        }
    }
    deserializer.deserialize_tuple(WINDOW_SIZE, MaskVisitor)
}
