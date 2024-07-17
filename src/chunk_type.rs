use std::str::FromStr;

/// A 4-byte chunk type code. Type codes are restricted to consist of uppercase and lowercase ASCII letters
/// (A-Z and a-z, or 65-90 and 97-122 decimal). However, encoders and decoders must treat the codes as fixed
/// binary values, not character strings. For example, it would not be correct to represent the type code
/// IDAT by the EBCDIC equivalents of those letters.
#[derive(Debug, PartialEq, Eq)]
struct ChunkType {
    ancillary: u8,
    private: u8,
    reserved: u8,
    safe_to_copy: u8,
}

impl ChunkType {
    /// Creates a Chunk Type from an array of bytes
    pub fn bytes(&self) -> [u8; 4] {
        [self.ancillary, self.private, self.reserved, self.safe_to_copy]
    }

    /// Checks whether the Chunk Type is a valid chunk type
    /// A valid chunk type has:
    /// - ancillary byte as ascii alphabetic
    /// - private byte as ascii alphabetic
    /// - reserved byte as ascii Uppercase
    /// - safe to copy byte as ascii alphabetic
    pub fn is_valid(&self) -> bool {
        self.ancillary.is_ascii_alphabetic() &&
        self.private.is_ascii_alphabetic() &&
        self.reserved.is_ascii_uppercase() &&
        self.safe_to_copy.is_ascii_alphabetic()
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        for i in 0..4 {
            let byte = value[i];
            if !byte.is_ascii_uppercase() && !byte.is_ascii_lowercase() {
                return Err("Invalid Type Code");
            }
        }
        Ok(ChunkType {
            ancillary: value[0],
            private: value[1],
            reserved: value[2],
            safe_to_copy: value[3]
        })
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let str_bytes = value.as_bytes();
        for i in 0..4 {
            let byte = str_bytes[i];
            if !byte.is_ascii_uppercase() && !byte.is_ascii_lowercase() {
                return Err("Invalid Type Code");
            }
        }
        Ok(ChunkType {
            ancillary: str_bytes[0],
            private: str_bytes[1],
            reserved: str_bytes[2],
            safe_to_copy: str_bytes[3],
        })
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

    // #[test]
    // pub fn test_chunk_type_is_critical() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(chunk.is_critical());
    // }
    //
    // #[test]
    // pub fn test_chunk_type_is_not_critical() {
    //     let chunk = ChunkType::from_str("ruSt").unwrap();
    //     assert!(!chunk.is_critical());
    // }
    //
    // #[test]
    // pub fn test_chunk_type_is_public() {
    //     let chunk = ChunkType::from_str("RUSt").unwrap();
    //     assert!(chunk.is_public());
    // }
    //
    // #[test]
    // pub fn test_chunk_type_is_not_public() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(!chunk.is_public());
    // }
    //
    // #[test]
    // pub fn test_chunk_type_is_reserved_bit_valid() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(chunk.is_reserved_bit_valid());
    // }
    //
    // #[test]
    // pub fn test_chunk_type_is_reserved_bit_invalid() {
    //     let chunk = ChunkType::from_str("Rust").unwrap();
    //     assert!(!chunk.is_reserved_bit_valid());
    // }
    //
    // #[test]
    // pub fn test_chunk_type_is_safe_to_copy() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert!(chunk.is_safe_to_copy());
    // }
    //
    // #[test]
    // pub fn test_chunk_type_is_unsafe_to_copy() {
    //     let chunk = ChunkType::from_str("RuST").unwrap();
    //     assert!(!chunk.is_safe_to_copy());
    // }

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

    // #[test]
    // pub fn test_chunk_type_string() {
    //     let chunk = ChunkType::from_str("RuSt").unwrap();
    //     assert_eq!(&chunk.to_string(), "RuSt");
    // }
    //
    // #[test]
    // pub fn test_chunk_type_trait_impls() {
    //     let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
    //     let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
    //     let _chunk_string = format!("{}", chunk_type_1);
    //     let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    // }
}
