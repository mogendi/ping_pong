use crate::chunk_type::{self, ChunkType};
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use std::str::{FromStr, from_utf8};
use std::string::{FromUtf8Error, String};

#[derive(Clone)]
pub struct Chunk {
    chunk_length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    chunk_crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc = crc32fast::hash(
            &chunk_type
                .chunk_type
                .iter()
                .chain(data.iter())
                .copied()
                .collect::<Vec<u8>>()[..],
        );
        Chunk {
            chunk_length: data.len() as u32,
            chunk_type: chunk_type,
            chunk_data: data,
            chunk_crc: crc,
        }
    }

    pub fn new_no_state(chunk_type: String, data: Vec<u8>) -> Result<Chunk, &'static str> { 
        match ChunkType::from_str(&chunk_type[..]) {
            Ok(chunk_type) => {
                let crc = crc32fast::hash(
                    &chunk_type
                        .chunk_type
                        .iter()
                        .chain(data.iter())
                        .copied()
                        .collect::<Vec<u8>>()[..],
                );
                Ok (
                    Chunk {
                        chunk_length: data.len() as u32,
                        chunk_type: chunk_type,
                        chunk_data: data,
                        chunk_crc: crc
                    }
                )
            }
            Err(msg) => {
                Err(msg)
            }
        }
    }
    pub fn length(&self) -> u32 {
        self.chunk_data.len().try_into().unwrap()
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data(&self) -> &[u8] {
        &self.chunk_data[..]
    }
    pub fn crc(&self) -> u32 {
        self.chunk_crc
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        self.chunk_length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.chunk_data.iter())
            .chain(self.chunk_crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
    pub fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        String::from_utf8(self.chunk_data.clone())
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;

    fn try_from(source: &[u8]) -> Result<Self, Self::Error> {
        if source.len() < 12 {
            // Every PNG chunk byte source needs to
            // at leats have the PNG chunk header data
            Err("Invalid PNG chunk data")
        } else {
            let chunk_length: u32 = u32::from_be_bytes(source[0..4].try_into().unwrap());
            let chunk_type: [u8; 4] = source[4..8].try_into().unwrap();
            let chunk_data: Vec<u8> = source[8..8 + chunk_length as usize]
                .iter()
                .copied()
                .collect();
            let chunk_crc =
                u32::from_be_bytes(source[8 + chunk_length as usize..].try_into().unwrap());
            println!("{}", chunk_crc);
            if crc32fast::hash(&source[4..8 + chunk_length as usize]) != chunk_crc {
                return Err("Invalid chunk CRC");
            }
            let chunk = Chunk {
                chunk_length: chunk_length,
                chunk_type: ChunkType::try_from(chunk_type).unwrap(),
                chunk_data: chunk_data,
                chunk_crc: chunk_crc,
            };
            Ok(chunk)
        }
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", from_utf8(&self.chunk_data).unwrap())
    }
}

impl Debug for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match from_utf8(&self.chunk_data) {
            Ok(fstr) => write!(f, "{}", fstr),
            Err(_) => write!(f, "{:?}", &self.chunk_data)
        }
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

    #[test]
    pub fn create_new_chunk() {
        let chunk_type = "RuSt".as_bytes();
        let msg = "This is a secret message!".as_bytes();
        let crc = crc32fast::hash(
            &chunk_type
                .iter()
                .chain(msg.iter())
                .copied()
                .collect::<Vec<u8>>()[..],
        );
        assert_eq!(
            Chunk::new(
                ChunkType::from_str("RuSt").unwrap(),
                msg.iter().copied().collect()
            )
            .chunk_crc,
            crc
        )
    }
}
