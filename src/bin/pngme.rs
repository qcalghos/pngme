use clap::Parser;
use pngme::args::{Cli,PngMeArgs};
use pngme::chunk::Chunk;
use pngme::chunk_type::ChunkType;
use pngme::error::PngError;
use pngme::png::Png;
use std::fs;
use std::str::FromStr;

fn main()->Result<(),PngError>{
    let cli=Cli::parse();
    println!("cli:\n{:?}",cli);
    match cli.commands{
        // pngme encode ./dice.png ruSt "This is a seret message!"
        PngMeArgs::Encode(encode_args)=>{
            let input_file=&encode_args.src_path;
            let chunk_type=ChunkType::from_str(&encode_args.chunk_type)?;
            let chunk_data=encode_args.message.as_bytes().to_vec();
            let chunk=Chunk::new(chunk_type,chunk_data);
            let mut png=Png::from_file(input_file)?;
            png.append_chunk(chunk);
            //判断输出路径是否存在
            if let Some(output_path)=encode_args.output_path{
                fs::write(output_path, png.as_bytes())?;
            }
        
        },
        //pngme decode ./dice.png ruSt
        PngMeArgs::Decode(decode_args)=>{
            let input_file=&decode_args.src_path;
            let chunk_type=decode_args.chunk_type;
            let png=Png::from_file(input_file)?;
            let find_chunk=png.chunk_by_type(&chunk_type);
            match find_chunk{
                Some(chunk)=>{
                    let message=String::from_utf8(chunk.data().to_vec())?;
                    println!("隐藏在chunk类型为 {} 块中的信息为 {}",chunk_type,message);
                },
                None=>println!("没有发现chunk 类型为 {} 的 chunk块",chunk_type),
            }
        },
    
        PngMeArgs::Print(print_args)=>{
            let input_file=&print_args.src_path;
            let png=Png::from_file(input_file)?;
            println!("{}",png);
        },
        //pngme remove ./dice.png ruSt
        PngMeArgs::Remove(remove_args)=>{
            let input_file=&remove_args.src_path;
            let chunk_type=remove_args.chunk_type;
            let mut png=Png::from_file(input_file)?;
            let removed_chunk=png.remove_chunk(&chunk_type)?;
            let removed_message=String::from_utf8(removed_chunk.data().to_vec())?;
            println!("删除chunk类型为 {} 的块完成，其中的隐藏信息为 {}",chunk_type,removed_message);
        },
    }
    Ok(())
}