use std::io::stdin;
use std::io::Write;
use std::io::stdout;
use std::process::*;

fn main() {
    println!("Welcome to RevolverDB");
    while true {
        let input = get_input();        
        println!("{}", input);
    }
}

fn print_stub() {
     print!("RDB > ");
     stdout().flush().unwrap();
}

fn get_input() -> String {
    let mut input = String::new();
    print_stub();
    return match stdin().read_line(&mut input) {
        Ok(n) => {
            if input.trim().eq_ignore_ascii_case(".exit")  {
                exit(0);
            } 
            println!("{} bytes read", n);
            input
        },
        Err(err) =>{
            println!("Error: {}", err);
            exit(0);
        }
    }
}
