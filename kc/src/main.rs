use std::process::exit;

mod compiler;


const VERSION:&str = "1.0.0";
fn main() {
    //cli args
    let args: Vec<String> = std::env::args().collect();

    //cli handling
    if args.len() == 1{
        println!("kc {VERSION}");
        println!("the compiler for kartikeya language");
        println!("\nfor usage:\n kc help");
    }else if args.len() == 2 {
        if args[1] == "version" || args[1] == "--version" || args[1] == "-v" {
            println!("kc {VERSION}");
        }else if args[1] == "help" || args[1] == "--help" || args[1] == "-h" {
            println!("kc commands:");
            println!(" 1. kc <no args>    -> prints about the compiler");
            println!(" 2. kc version      -> prints the version of the compiler");
            println!(" 3. kc help         -> prints this commands list");
            println!(" 4. kc <filename.k> -> compiles k source file into c source file");
            
        }else if args[1].len() >= 3{
            let filename = &args[1];
            if &filename[filename.len()-2..] != ".k" {
                println!("error: invalid source file!");
                exit(1);
            }
            compiler::compile(filename);
        }else {
            println!("error: invalid command!");
            exit(1);
        }
    }else {
        println!("error: too many arguments!");
        exit(1);
    }


}
