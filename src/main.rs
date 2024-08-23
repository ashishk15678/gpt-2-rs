#![allow(special_module_name)]

mod Graph;
mod gpt;
pub mod lib;
//mod optimizer;
mod tensor;
mod test;
mod tokenizer;

use std::fs;

use tokenizer::encode;

fn main() {
    println!("INFO : You started the model's learning");
    // println!("Hello world");
    //    let mut data = "I am ashish and this is not fun to code, i swear to god , i i i  i i ";
    println!("INFO : Reading data from source");
    let data = fs::read_to_string("dataset.txt").expect("Not able to read dataset");
    println!("INFO : Read data from source");
    println!("INFO : Tokenization Started");
    let d = encode(data.to_string());
    println!(
        "INFO : Tokenization done ,  vocab size is {:?}",
        d.blob.len()
    );
    let batch_size = 64;
    let num_tokens = 128;
    let vocab_size = d.blob.len();
    let embedding_degree = 128;
    let nums_heads = 8;
    let num_layers = 8;
    let head_size = embedding_degree / nums_heads;
}
