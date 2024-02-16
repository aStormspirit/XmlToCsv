use std::io;
use std::env;
use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};

fn is_url(input: &str) -> bool {
    let url_regex = Regex::new(r"^(https?|ftp)://[^\s/$.?#].[^\s]*$").unwrap();
    url_regex.is_match(input)
}

fn main() {
    let file = get_file();
    load_file(&file);
}

fn get_file() -> String {
    let mut input = String::new();
    let current_dir = env::current_dir().expect("Не удалось получить текущую директорию");

    println!("Введите ссылку или название файла в директории");
    
    io::stdin().read_line(&mut input)
        .expect("Не удалось прочитать строку");

    let link_or_path = input.trim().to_string();
    
    if is_url(&link_or_path) {
        link_or_path
    } else {
        format!("{}\\{}", current_dir.display(), link_or_path)
    }
}

fn load_file(file: &str){
    let file = File::open(file).expect("Unable to open file");
    let file = BufReader::new(file);
  
    let parser = EventReader::new(file);

    for event in parser {
        println!("{:?}", event.unwrap());
    }
}