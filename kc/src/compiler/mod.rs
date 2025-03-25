use std::{path::Path, process::exit};

mod lexer;

pub fn compile(filename:&str){
    let code = read_file(filename);
    lexer::lexer(&code);
}

fn read_file(filename:&str)-> String{
    let file_path = Path::new(filename);
    let content = match std::fs::read_to_string(file_path){
        Ok(content) => content,
        Err(_) => {
            println!("error: '{filename}' not found!");
            exit(1);
        }
        
    }; 
    content
}