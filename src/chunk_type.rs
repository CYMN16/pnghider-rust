use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

#[derive(PartialEq, Eq)]
#[derive(Debug)]
pub struct ChunkType {
    critical: u8,
    public: u8,
    reserved: u8,
    safe_to_copy: u8,
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = String;
    fn try_from(arr: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(Self {
            critical: arr[0],
            public: arr[1],
            reserved: arr[2],
            safe_to_copy: arr[3],
        })
    }
}

impl FromStr for ChunkType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for byte in s.as_bytes(){
            if !byte.is_ascii_alphabetic() {return Err("String is not alphabetic".to_string());}
        }
        if s.len() == 4 {
            Ok(Self {
                critical: s.as_bytes()[0],
                public: s.as_bytes()[1],
                reserved: s.as_bytes()[2],
                safe_to_copy: s.as_bytes()[3],
            })
        } else {
            Err("Incorrect string size!".to_string())
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            String::from_utf8(self.bytes().to_vec()).expect("Utf-8 conversion error")
        )
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        [self.critical, self.public, self.reserved, self.safe_to_copy]
    }

    fn is_critical(&self) -> bool {
        self.critical.is_ascii_uppercase()
    }
    fn is_public(&self) -> bool {
        self.public.is_ascii_uppercase()
    }
    fn is_reserved_bit_valid(&self) -> bool {
        self.reserved.is_ascii_uppercase()
    }
    fn is_safe_to_copy(&self) -> bool {
        self.safe_to_copy.is_ascii_lowercase()
    }
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
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
    }
}
