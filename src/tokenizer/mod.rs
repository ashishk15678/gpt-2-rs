#![allow(unused)]

// The key idea is to make something like tiktoken-rs
// which is an abstraction of tiktokens python api
//
// TikToken was used in gpt2 and many other projects
// It is basically a byte pair encoder
//
// The thing is , Can I make it ?
//

use std::collections::HashMap;

use rand::Rng;

pub struct TokenContainer {
    pub blob: Vec<String>,
    pub vocab_size: usize,
    pub str_token: HashMap<String, Token>,
    pub int_ch: HashMap<usize, Token>,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Token {
    data: String,
    count: usize,
    int: usize,
}

/// This function returns a TokenContainer
/// which contains tokens
pub fn encode(dataset: String) -> TokenContainer {
    let mut blob = vec![];
    let mut vocab_size: usize = 0;
    let mut str_token: HashMap<String, Token> = HashMap::new();
    let mut int_ch: HashMap<usize, Token> = HashMap::new();

    // mercy street
    //
    let mut bpe: usize;
    for i in dataset.split_whitespace() {
        let mut t = "";

        if !blob.contains(&i.to_string()) {
            bpe = rand::thread_rng().gen();
            let temp = str_token.insert(
                i.to_string(),
                Token {
                    data: i.to_string(),
                    count: 0,
                    int: bpe,
                },
            );
            let t1 = int_ch.insert(
                bpe,
                Token {
                    data: i.to_string(),
                    count: 1,
                    int: bpe,
                },
            );
            blob.push(i.to_string());
            vocab_size += 1;
            bpe += 1;
        } else {
            str_token.entry(i.to_string()).and_modify(|m| m.count += 1);
        };
    }
    // print!("{:?}", str_token);
    // at this point we have iterated through the whole dataset once
    // now for the second iteration , we are going to look for every possible substring in
    // a word and try to find in the blob vector as to increase the count
    //
    for i in dataset.split_whitespace() {
        let mut t3 = "".to_string();
        let window_size = 5;

        for n in i.chars() {
            t3 = format!("{t3}{n}");

            if (str_token.contains_key(&t3)) {
                // println!("before changing , {:?}", str_token.get(&t3.clone()));
                str_token.entry(t3.clone()).and_modify(|m| m.count += 1);
                //println!("after changing ,{:?}", str_token.get(&t3.clone()));
            }
        }
        //        println!("{t3}");
    }

    // println!("{:?}", str_token);
    TokenContainer {
        blob,
        vocab_size,
        str_token,
        int_ch,
    }
}
