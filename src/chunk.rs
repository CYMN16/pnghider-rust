use std::{fmt::Display, str::FromStr, string};

use crate::chunk_type::ChunkType;

use crc::*;
#[derive(Debug)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    //fn crc(&self) -> u32{
    //    x^32+x^26+x^23+x^22+x^16+x^12+x^11+x^10+x^8+x^7+x^5+x^4+x^2+x+1
    //}
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let length: u32 = data.len() as u32;
        let mut whole_chunk = Self {
            length,
            chunk_type,
            chunk_data: data,
            crc: 0,
        };
        let crc = whole_chunk.crc();
        whole_chunk.crc = crc;
        whole_chunk
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        result.append(&mut self.length.to_be_bytes().to_vec());
        result.append(&mut self.chunk_type.bytes().to_vec());
        result.append(&mut self.chunk_data.clone());
        result.append(&mut self.crc.to_be_bytes().to_vec());
        result
    }
    pub fn crc(&self) -> u32 {
        let mut chunk_data = self.chunk_data.clone();
        let mut data = self.chunk_type.bytes().to_vec();
        data.append(&mut chunk_data);
        let crc_calculator = crc::Crc::<u32>::new(&CRC_32_ISO_HDLC);
        crc_calculator.checksum(&data)
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }
    pub fn data_as_string(&self) -> Result<String, string::FromUtf8Error> {
        String::from_utf8(self.chunk_data.clone())
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = String;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        //let chunk_data: Vec<u8> = data_length
        //    .to_be_bytes()
        //    .iter()
        //    .chain(chunk_type.iter())
        //    .chain(message_bytes.iter())
        //    .chain(crc.to_be_bytes().iter())
        //    .copied()
        //    .collect();
        let mut chnks = value.chunks(4);
        let length: u32 = u32::from_be_bytes(
            chnks
                .next()
                .expect("None type chunk length")
                .try_into()
                .expect("Error converting length"),
        );
        let chunk_type = ChunkType::from_str(
            &String::from_utf8(chnks.next().expect("None received").to_vec())
                .expect("String conversion error"),
        )
        .expect("ChunkType conversion error");
        //let (crc, chunk_data) = chnks
        let chunk_data_crc: Vec<&[u8]> = chnks.collect();
        let crc: Vec<u8> = chunk_data_crc.concat().split_off(length as usize);
        let mut chunk_data: Vec<u8> = chunk_data_crc.concat();
        chunk_data.resize(length as usize, 0);
        //let (chunk_data, crc) = chunk_data_crc
        //    .as_slice()
        //    .split_last_chunk::<1>()
        //    .expect("None type chunk");

        //todo!()

        //    .split_last_chunk::<4>()
        //    .expect("None type chunk");

        //crc.resize(4, 0);
        //println!("length {:?}", length);
        //println!("chunk_type{:?}", chunk_type.to_string());
        //println!("chunk_data len {:?}", chunk_data.len());
        //println!(
        //    "chunk_data {:?}",
        //    String::from_utf8(chunk_data.clone()).expect("Utf8 conversion error")
        //);
        //println!("crc {:?}", crc);
        let crc: u32 = u32::from_be_bytes(crc.try_into().expect("Error converting crc"));
        let whole_chunk = Self {
            length,
            chunk_type,
            chunk_data,
            crc,
        };
        if crc != whole_chunk.crc() {
            Err("Wrong crc".to_string())
        } else {
            Ok(whole_chunk)
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.data_as_string()
                .expect("Could not convert chunk to string")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
