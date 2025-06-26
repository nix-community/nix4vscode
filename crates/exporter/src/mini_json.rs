#![allow(unused_variables)]
use serde::{
    Serializer,
    ser::{SerializeMap, SerializeSeq, SerializeStruct},
};

pub fn to_string<T: serde::Serialize>(v: &T) -> String {
    let s = MiniSerializer {
        dst: Default::default(),
        is_value: false,
        len: 0,
        with_sem: false,
    };
    v.serialize(s).unwrap()
}

#[derive(Default)]
pub struct MiniSerializer {
    dst: String,
    is_value: bool,
    len: usize,
    with_sem: bool,
}

impl Serializer for MiniSerializer {
    type Ok = String;

    type Error = toml_edit::ser::Error;

    type SerializeSeq = Self;

    type SerializeTuple = serde::ser::Impossible<Self::Ok, Self::Error>;

    type SerializeTupleStruct = serde::ser::Impossible<Self::Ok, Self::Error>;

    type SerializeTupleVariant = serde::ser::Impossible<Self::Ok, Self::Error>;

    type SerializeMap = Self;

    type SerializeStruct = Self;

    type SerializeStructVariant = serde::ser::Impossible<Self::Ok, Self::Error>;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        Ok("true".into())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        Ok(format!("{v:?}"))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        let dst = self.dst + &value.serialize(Self::default())?;
        Ok(dst)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        let Self {
            dst,
            is_value,
            len: _,
            with_sem,
        } = self;
        let dst = dst + "[\n";
        Ok(Self {
            dst,
            is_value,
            len: len.unwrap_or_default(),
            with_sem,
        })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(mut self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.dst += "{";
        self.len = len.unwrap_or(0);
        Ok(self)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        let Self {
            mut dst,
            is_value,
            len: _,
            with_sem,
        } = self;
        dst += "{";
        Ok(Self {
            dst,
            is_value,
            len,
            with_sem,
        })
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

impl SerializeStruct for MiniSerializer {
    type Ok = String;

    type Error = toml_edit::ser::Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.dst += &format!("\"{key}\"");
        self.dst += ":";
        {
            let x = value.serialize(Self::default())?;
            self.dst += &x;
        }
        self.len -= 1;
        if self.len != 0 {
            self.dst += ",";
        }
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.dst += "}";
        Ok(self.dst)
    }
}

impl SerializeSeq for MiniSerializer {
    type Ok = String;

    type Error = toml_edit::ser::Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        let v = value.serialize(Self {
            is_value: true,
            ..Default::default()
        })?;
        self.dst += &v;
        self.len -= 1;
        if self.len != 0 {
            self.dst += ",";
        }
        self.dst += "\n";
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.dst += "]";
        if self.with_sem {
            self.dst += ",";
        }
        self.dst += "\n";
        Ok(self.dst)
    }
}

impl SerializeMap for MiniSerializer {
    type Ok = String;

    type Error = toml_edit::ser::Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        let key = key.serialize(Self::default())?;
        self.dst += &key;
        self.dst += ":";

        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + serde::Serialize,
    {
        self.len -= 1;
        let value = value.serialize(Self {
            with_sem: self.len != 0,
            ..Default::default()
        })?;
        self.dst += &value;
        Ok(())
    }

    fn end(mut self) -> Result<Self::Ok, Self::Error> {
        self.dst += "}";
        Ok(self.dst)
    }
}
