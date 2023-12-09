/****main.rs****/
use std::fs::*;
use std::io::{BufRead, BufReader};
use std::env::*;
#[derive(Clone)]
struct Config {
    text: Vec<String>,
    line: i32,
}
impl Config {
    fn new() -> Config {
        Config {
            text: Vec::new(),
            line: 0,
        }
    }
}
fn all_file(config:&Config){
    for text in &config.text {
        let text = text.trim(); 
        File::create(text);
    }
}
fn all_mkdir(config: &Config) {
    for text in &config.text {
        // 改行文字を取り除く
        let text = text.trim(); // create_dir_allでディレクトリを作成 
        match create_dir_all(text) {
            Ok(_) => println!("{} directory created", text),
            Err(err) => println!("{} directory creation failed: {}", text, err),
        }
    }
}
fn main() {
    let args:Vec<String> = args().collect();
    let default_args:String = String::from("not found mkFiles");
    let path = args.get(1).unwrap_or(&default_args);
    let mut file = File::open(&path);
    if !path.is_empty(){
        file = File::open(path);
    }
    match file {
        Ok(val) => {
            let mut reader = BufReader::new(val);
            let mut conf = Config::new();
            let mut file_line = String::new();
            let mut line = 0;
            loop {
                file_line.clear();
                let file_len = reader.read_line(&mut file_line).unwrap();
                if file_len == 0 {
                    break;
                }
                line += 1;
                conf.line = line;
                conf.text.push(file_line.clone());
            }
            all_file(&conf);
            all_mkdir(&conf);
            println!("text:{:?} line:{:?}", conf.text, conf.line);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
