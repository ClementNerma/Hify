/// Base62 character set: 0-9, A-Z, a-z
const BASE62_CHARSET: &[u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// Encodes bytes to a base62 string
#[allow(
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::indexing_slicing,
    clippy::integer_division
)]
pub fn encode_base62(data: &[u8]) -> String {
    if data.is_empty() {
        return String::new();
    }

    // Count leading zeros (they need special handling)
    let leading_zeroes = data.iter().take_while(|&&b| b == 0).count();

    // Convert bytes to a big integer (simple implementation using Vec<u8>)
    let mut num = data.to_vec();
    let mut result = Vec::new();

    // Convert to base62 by repeated division
    while !(num.is_empty() || (num.len() == 1 && num[0] == 0)) {
        let mut remainder = 0_u32;
        let mut new_num = Vec::new();

        for &byte in &num {
            let current = remainder * 256 + u32::from(byte);
            let quotient = current / 62;
            remainder = current % 62;

            if !new_num.is_empty() || quotient > 0 {
                new_num.push(u8::try_from(quotient).unwrap());
            }
        }

        result.push(BASE62_CHARSET[remainder as usize]);
        num = new_num;
    }

    // Add leading '0's for each leading zero byte
    result.extend(std::iter::repeat_n(b'0', leading_zeroes));

    // Reverse and convert to string
    result.reverse();
    String::from_utf8(result).unwrap()
}

pub fn encode_base62_u64(value: u64) -> String {
    encode_base62(&value.to_le_bytes())
}

/// Decodes a base62 string to bytes
#[allow(
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::indexing_slicing
)]
pub fn decode_base62(encoded: &str) -> Result<Vec<u8>, String> {
    if encoded.is_empty() {
        return Ok(vec![]);
    }

    // Count leading '0's (they represent zero bytes)
    let leading_zeros = encoded.chars().take_while(|&c| c == '0').count();

    // Convert each character to its base62 value
    let mut num: Vec<u8> = Vec::new();

    for c in encoded.chars() {
        let digit = match c {
            '0'..='9' => c as u8 - b'0',
            'A'..='Z' => c as u8 - b'A' + 10,
            'a'..='z' => c as u8 - b'a' + 36,
            _ => {
                return Err(format!(
                    "Invalid character '{c}' in base62 string: {encoded}",
                ));
            }
        };

        // Multiply num by 62 and add digit
        let mut carry = u32::from(digit);
        for byte in num.iter_mut().rev() {
            let current = u32::from(*byte) * 62 + carry;
            *byte = (current & 0xFF) as u8;
            carry = current >> 8;
        }

        while carry > 0 {
            num.insert(0, (carry & 0xFF) as u8);
            carry >>= 8;
        }
    }

    // Prepend zero bytes for leading '0' characters
    let mut result = vec![0_u8; leading_zeros];
    result.extend(num);

    Ok(result)
}

pub fn decode_base62_u64(encoded: &str) -> Result<u64, String> {
    let bytes = decode_base62(encoded)?;

    <[u8; 8]>::try_from(bytes)
        .map(u64::from_le_bytes)
        .map_err(|_| format!("Invalid base62-encoded 64-bit integer: {encoded:?}"))
}

pub mod u64_base62_serialization {
    use serde::{
        Deserializer, Serializer,
        de::{Error, Visitor},
    };

    #[allow(clippy::trivially_copy_pass_by_ref)] // BUG: doesn't work here
    pub fn serialize<S: Serializer>(value: &u64, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(&super::encode_base62_u64(*value))
    }

    struct StrU64Visitor;

    impl<'de> Visitor<'de> for StrU64Visitor {
        type Value = u64;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string containing an integer between -2^64 and 2^64-1")
        }

        fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            super::decode_base62_u64(v).map_err(E::custom)
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<u64, D::Error> {
        de.deserialize_str(StrU64Visitor)
    }
}

// pub mod str_base62_serialization {
//     use serde::{
//         Deserializer, Serializer,
//         de::{Error, Visitor},
//     };

//     pub fn serialize<S: Serializer>(value: &str, ser: S) -> Result<S::Ok, S::Error> {
//         ser.serialize_str(&super::encode_base62(value.as_bytes()))
//     }

//     struct Base64StrVisitor;

//     impl<'de> Visitor<'de> for Base64StrVisitor {
//         type Value = String;

//         fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//             formatter.write_str("a string containing an integer between -2^64 and 2^64-1")
//         }

//         fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
//         where
//             E: Error,
//         {
//             super::decode_base62(v)
//                 .ok()
//                 .and_then(|bytes| String::from_utf8(bytes).ok())
//                 .ok_or_else(|| E::custom("Failed to decode base62 buffer as an UTF-8 string"))
//         }
//     }

//     pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<String, D::Error> {
//         de.deserialize_str(Base64StrVisitor)
//     }
// }
