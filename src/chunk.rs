
use std::convert::TryFrom;
use std::fmt;

use crate::chunk_type::ChunkType;
use crate::error::PngError;

use crc::Crc;
pub struct Chunk{
    len:u32, //len 前边要加pub吗，表示数据域字节数
    chunk_type:ChunkType,//块类型
    data:Vec<u8>,
    crc:u32,//crc计算的是chunk type+data 的crc。
}

impl Chunk{
    pub fn new(chunk_type:ChunkType,data:Vec<u8>)->Chunk{
        let crc_source=chunk_type.bytes().iter().chain(data.iter()).copied().collect::<Vec<u8>>();
        let  crc=Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&crc_source);
        Chunk{len:data.len() as u32,chunk_type,data,crc}
    }
    //返回数据域长度
    pub fn length(&self)->u32{
        self.len
    }
    pub fn chunk_type(&self)->&ChunkType{
        &self.chunk_type
    }
    pub fn data(&self)->&[u8]{
        &self.data
    }
    pub fn crc(&self)->u32{
        self.crc
    }
    pub fn data_as_string(&self)->Result<String,PngError>{
        let s=String::from_utf8(self.data.clone())?;
        Ok(s)
    }
    pub fn as_bytes(&self)->Vec<u8>{
        self.len.to_be_bytes().iter()
        .chain(self.chunk_type.bytes().iter())
        .chain(self.data.iter())
        .chain(self.crc.to_be_bytes().iter())
        .copied()
        .collect::<Vec<u8>>()
    }
}

impl TryFrom<&[u8]> for Chunk{
    type Error=PngError;
    fn try_from(value:&[u8]) -> Result<Self, Self::Error> {
        if value.len()<12{
            return Err(PngError::ChunkError);
        }
        let chunk_type_bits:[u8;4]=value[4..8].try_into().expect("error &[u8] to chunk_type_bits");
        let chunk_type=ChunkType::try_from(chunk_type_bits)?;
        let data=value[8..value.len()-4].to_vec();
        let crc_bits:[u8;4]=value[value.len()-4..].try_into().expect("error &[u8] to [u8;4]");
        let crc=u32::from_be_bytes(crc_bits);
        let chunk=Chunk::new(chunk_type,data);
        if chunk.crc() !=crc{
            return Err(PngError::CRCError);
        }
        Ok(chunk)

    }
}
impl fmt::Display for Chunk{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{{{},{},{:?},{}}}",self.len,self.chunk_type,self.data,self.crc)
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
        let data = "This is where your secret message will be!".as_bytes().to_vec();
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
