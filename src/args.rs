use clap::{Parser,Subcommand, Args};

use std::path::PathBuf;

#[derive(Debug,Parser)]
pub struct Cli{
    #[command(subcommand)]
    pub commands:PngMeArgs,
}
#[derive(Subcommand,Debug)]
pub enum PngMeArgs{
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}
#[derive(Debug,Args)]
pub struct EncodeArgs{
    #[clap(value_parser)]
    pub src_path:PathBuf,

    #[clap(value_parser)]
    pub chunk_type:String,

    #[clap(value_parser)]
    pub message:String,
    #[clap(value_parser)]
    pub output_path:Option<PathBuf>
}
#[derive(Debug,Args)]
pub struct DecodeArgs{
    #[clap(value_parser)]
    pub src_path:PathBuf,

    #[clap(value_parser)]
    pub chunk_type:String,   
}
#[derive(Debug,Args)]
pub struct RemoveArgs{
    #[clap(value_parser)]
    pub src_path:PathBuf,

    #[clap(value_parser)]
    pub chunk_type:String,
}

#[derive(Debug,Args)]
pub struct PrintArgs{
    #[clap(value_parser)]
    pub src_path:PathBuf,  
}
