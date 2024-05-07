use std::convert::TryFrom;
use std::str::FromStr;
use std::fmt::Display;
use crate::error::PngError;

#[derive(Debug,PartialEq,Eq)]
pub struct ChunkType(u8,u8,u8,u8);
impl ChunkType{
    pub fn bytes(&self)->[u8;4]{
        [self.0,self.1,self.2,self.3]
    }
    //判断是否为有效类型，每个字母必须是ascii大写，小写字母。z这个判断逻辑不能通过测试，只需判断保留位是否为大写
    pub fn is_valid(&self)->bool{
        // let bytes=self.bytes();
        // for c in bytes{
        //     if ! c.is_ascii_lowercase() && !c.is_ascii_uppercase(){
        //         return false;
        //     }
        // }
        // true
        self.is_reserved_bit_valid()
    }
    //第一个字母为大写则为关键
    pub fn is_critical(&self)->bool{
        self.0.is_ascii_uppercase()
    }
    //第一个字母大写为共有
    pub fn is_public(&self)->bool{
        self.1.is_ascii_uppercase()
    }
    //
    pub fn is_reserved_bit_valid(&self)->bool{
        self.2.is_ascii_uppercase()
    }
    pub fn is_safe_to_copy(&self)->bool{
        self.3.is_ascii_lowercase()
    }
}
impl TryFrom<[u8;4]> for ChunkType{
    type Error=PngError;
    fn try_from(value: [u8;4]) -> Result<Self, Self::Error> {
        //判断是否为有效类型,四个字母必须是大写或小写字母
        for c in value{
            if !c.is_ascii_alphabetic(){
                return Err(PngError::ChunkTypeError);
            }
        }
        Ok(ChunkType(value[0],value[1],value[2],value[3]))
    }
}
impl FromStr for ChunkType{
    type Err=PngError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len()!=4{
            return Err(PngError::ChunkTypeError);
        }
        let vec:Vec<char>=s.chars().collect();
        for c in &vec{
            if !c.is_ascii_alphabetic(){
                return Err(PngError::ChunkTypeError);
            }
        }
        Ok(ChunkType(vec[0] as u8,vec[1] as u8,vec[2] as u8,vec[3] as u8))

    }
}
impl Display for ChunkType{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes=self.bytes();
        write!(f,"{}",String::from_utf8(bytes.to_vec()).expect("ChunkType to String Failed"))
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


