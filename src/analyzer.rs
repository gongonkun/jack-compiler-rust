mod tokenizer;

use clap::Parser;
use std::fs::{self, File};
use std::io::Write;


#[derive(Parser, Debug)]
struct Cli {
    path: std::path::PathBuf,
}


/* cargo run --bin JackAnalyzer ../nand2tetris/projects/10/ArrayTest/Main.jack */
/* TODO: 諸々のエラー制御 */
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input_path = Cli::parse().path;
    if input_path.is_dir() {
        /* TODO: ディレクトリが渡された場合の処理 */
        return Ok(());
    }
    
    let source = fs::read_to_string(&input_path)?;
    let mut tokenizer = tokenizer::Tokenizer::new(&source);
    let mut output_path = input_path.clone();
    let ouput_file = String::new() + &input_path.file_name().unwrap().to_str().unwrap().split_once('.').unwrap().0 + ".xml";
    output_path.set_file_name(ouput_file);
    let mut output_xml = File::create(output_path)?;
    println!("{:?}", output_path);

    while tokenizer.has_more_token() {
        println!("{:?}", tokenizer.next_token());
    };

    Ok(())
}
