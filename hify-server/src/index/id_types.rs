use std::hash::Hash;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Visitor};

pub trait IdType:
    std::fmt::Debug + Clone + Copy + Hash + PartialEq + Serialize + Deserialize<'static>
{
    fn encode(&self) -> String;
    fn decode(input: &str) -> Result<Self, IdTypeDecodingErr>;
}

pub struct IdTypeDecodingErr;

#[macro_export]
macro_rules! define_id_type {
    ($typename: ident) => {
        #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
        pub struct $typename(pub u64);

        impl $crate::index::IdType for $typename {
            fn encode(&self) -> String {
                $crate::index::u64_to_base36_str_no_alloc(self.0, |str| str.to_owned())
            }

            fn decode(input: &str) -> Result<Self, $crate::index::IdTypeDecodingErr> {
                u64::from_str_radix(input, 36).map(Self).map_err(|_| $crate::index::IdTypeDecodingErr)
            }
        }

        impl ::serde::Serialize for $typename {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                $crate::index::serialize_id_type(&self.0, serializer)
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $typename {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                $crate::index::deserialize_id_type(deserializer).map(Self)
            }
        }

        $crate::define_scalar_string!($typename);
    };

    ($($typename: ident),+) => {
        $($crate::define_id_type!($typename);)+
    }
}

pub fn u64_to_base36_str_no_alloc<T>(mut num: u64, finish: impl FnOnce(&str) -> T) -> T {
    let mut str = [0u8; 20];

    let mut i = 20;

    while num > 0 {
        let digit = num % 36;
        num = (num - digit) / 36;

        i -= 1;
        str[i] = match digit {
            0 => b'0',
            1 => b'1',
            2 => b'2',
            3 => b'3',
            4 => b'4',
            5 => b'5',
            6 => b'6',
            7 => b'7',
            8 => b'8',
            9 => b'9',
            10 => b'a',
            11 => b'b',
            12 => b'c',
            13 => b'd',
            14 => b'e',
            15 => b'f',
            16 => b'g',
            17 => b'h',
            18 => b'i',
            19 => b'j',
            20 => b'k',
            21 => b'l',
            22 => b'm',
            23 => b'n',
            24 => b'o',
            25 => b'p',
            26 => b'q',
            27 => b'r',
            28 => b's',
            29 => b't',
            30 => b'u',
            31 => b'v',
            32 => b'w',
            33 => b'x',
            34 => b'y',
            35 => b'z',
            _ => unreachable!(),
        };
    }

    finish(str::from_utf8(&str[i..=19]).unwrap())
}

pub fn serialize_id_type<S: Serializer>(value: &u64, serializer: S) -> Result<S::Ok, S::Error> {
    u64_to_base36_str_no_alloc(*value, |str| str.serialize(serializer))
}

pub fn deserialize_id_type<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
    struct IdVisitor;

    impl<'de> Visitor<'de> for IdVisitor {
        type Value = u64;

        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("a valid ID string")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }

        fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
            u64::from_str_radix(v, 36).map_err(|_| E::custom("not a valid unsigned 64-bit integer"))
        }
    }

    deserializer.deserialize_str(IdVisitor)
}
