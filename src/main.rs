mod compiler;
use compiler::simple_compiler::*;
use std::io::stdin;
use std::io::Write;
use std::io::stdout;
use std::process::exit;
use std::str;

fn main() {
    println!("Welcome to RevolverDB");
    while true {
        let input = get_input();        
        println!("{}", input);
        let command = prepare_command(input).expect("Error whilst parsing input");
        execute_command(command);
    }
}

fn execute_command(cmd: Command) {
    match cmd.command_type {
        CommandType::Insert => {
            let row_to_insert = cmd.row_to_insert.unwrap();
            println!("This is an insert");
            println!("ID {}", row_to_insert.id);
            println!("Email {}", str::from_utf8(&row_to_insert.email).unwrap());
            println!("Username {}", str::from_utf8(&row_to_insert.username).unwrap());
        },
        CommandType::Select => {
            println!("This is a select");
        }
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

