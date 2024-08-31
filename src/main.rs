mod Graph;
mod gpt;
pub mod lib;
//mod optimizer;
mod tensor;
mod test;
mod tokenizer;

use std::{
    env,
    fs::{self, create_dir_all, read_to_string, File},
    io::Write,
    process,
};

use tokenizer::encode;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "-t" {
        println!("INFO : You started tokenization");
        println!("INFO : Reading data from source");
        let data = fs::read_to_string("dataset.txt").expect("Not able to read dataset");
        let d = encode(data.to_string());

        println!("INFO : Tokenization done , Writing tokens to a file");
        // creating a folder named tokens
        create_dir_all("tokens").expect("ERROR : Cannot create directory , exiting");

        let mut file = File::create("tokens/token.txt").expect("ERROR : Cannot create file ");
        // let d = "sometext new text".to_string();
        for i in d.str_token {
            println!("{:?}", i);
            let temp = format!("{:?}\n", i);
            file.write(&temp.into_bytes())
                .expect("Cannot write to file");
            // file.write(b"\n").expect("Cannot write to file , exiting");
        }
        process::exit(0)
    }
    println!("INFO : Read data from source");
    println!("INFO : You started the model's learning");
    println!("INFO : Tokenization Started"); // println!("Hello world");
                                             //    let mut data = "I am ashish and this is not fun to code, i swear to god , i i i  i i ";
    println!("INFO : Reading data from source");
    let data = fs::read_to_string("dataset.txt").expect("Not able to read dataset");
    println!("INFO : Read data from source");

    println!("INFO : Tokenization Started");
    let data = read_to_string("dataset.txt").expect("ERROR : Cannot read from file dataaset");
    let d = encode(data.to_string());
    println!(
        "INFO : Tokenization done ,  vocab size is {:?}",
        d.blob.len()
    );

    println!("INFO : Initiating word embeddings");

    let batch_size = 64;
    let num_tokens = 128;
    let vocab_size = d.blob.len();
    let embedding_degree = 128;
    let nums_heads = 8;
    let num_layers = 8;
    let head_size = embedding_degree / nums_heads;
}
