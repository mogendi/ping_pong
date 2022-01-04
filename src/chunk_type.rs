use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use std::str::{from_utf8, FromStr};

#[derive(Eq, PartialEq, Clone)]
pub struct ChunkType {
    pub chunk_type: [u8; 4],

    // Flags
    pub is_valid: bool,
    pub is_public: bool,
    pub is_critical: bool,
    pub is_safe_to_copy: bool,
    pub is_reserved_bit_valid: bool,
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", from_utf8(&self.chunk_type).unwrap())
    }
}

impl Debug for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", from_utf8(&self.chunk_type).unwrap())
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.chunk_type
    }
    // most of these methods can be determined on
    // chunk creation. Reading it from a flag instead
    // of storing and re-reading the source makes more sense
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
    pub fn is_critical(&self) -> bool {
        self.is_critical
    }
    fn is_public(&self) -> bool {
        self.is_public
    }
    fn is_reserved_bit_valid(&self) -> bool {
        self.is_reserved_bit_valid
    }
    fn is_safe_to_copy(&self) -> bool {
        self.is_safe_to_copy
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        match from_utf8(&value) {
            Ok(_) => {
                let mut chunk = ChunkType {
                    chunk_type: value,
                    is_valid: false,
                    is_public: false,
                    is_critical: false,
                    is_safe_to_copy: false,
                    is_reserved_bit_valid: false,
                };
                chunk.is_public = value[1].is_ascii_uppercase();
                chunk.is_reserved_bit_valid = value[2].is_ascii_uppercase();
                chunk.is_safe_to_copy = value[3].is_ascii_lowercase();
                chunk.is_valid = chunk.is_reserved_bit_valid;
                chunk.is_critical = value[0].is_ascii_uppercase();
                Ok(chunk)
            }
            Err(_) => Err("Invalid utf8 characters in value"),
        }
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for i in 0..4 {
            if !s.chars().nth(i).unwrap().is_alphabetic() {
                return Err("Invalid utf8 characters in value");
            }
        }
        let value: [u8; 4] = s.as_bytes().to_owned().try_into().unwrap();
        let mut chunk = ChunkType {
            chunk_type: value,
            is_valid: false,
            is_public: false,
            is_critical: false,
            is_safe_to_copy: false,
            is_reserved_bit_valid: false,
        };
        chunk.is_public = value[1].is_ascii_uppercase();
        chunk.is_reserved_bit_valid = value[2].is_ascii_uppercase();
        chunk.is_safe_to_copy = value[3].is_ascii_lowercase();
        chunk.is_valid = chunk.is_reserved_bit_valid;
        chunk.is_critical = value[0].is_ascii_uppercase();
        Ok(chunk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
        assert!(_are_chunks_equal);
    }
}
