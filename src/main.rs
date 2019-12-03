use std::io::stdin;
use std::io::Write;
use std::io::stdout;
use std::process::*;

mod errors;
use errors::CommandError::CommandError as CommandError;

fn main() {
    println!("Welcome to RevolverDB");
    while true {
        let input = get_input();        
        println!("{}", input);
        let command = prepare_command(input).expect("Error whilst parsing input");
        execute_command(command);
    }
}

//TODO - We need to replace this with a proper parser and grammer
fn prepare_command(parsed_input: String) -> Result<Command, CommandError> {
    
    if parsed_input.starts_with("insert"){
        return Ok(Command {
        command_type: CommandType::Insert
        })
    }

    if parsed_input.starts_with("select"){
        return Ok(Command {
        command_type: CommandType::Select
        })
    }

    Err(CommandError)
}

fn execute_command(cmd: Command) {
    println!("This command is a {:?}", cmd.command_type);
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


struct Command {
    command_type: CommandType
}

#[derive(Debug)]
enum CommandType {
    Insert, 
    Select
}

