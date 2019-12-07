mod compiler;
use compiler::test_table::Table as Table;
use compiler::simple_compiler::*;
use std::io::stdin;
use std::io::Write;
use std::io::stdout;
use std::process::exit;
use std::str;

fn main() {
        println!("Welcome to RevolverDB");
        let mut table = Table::new();
        while true {
            let input = get_input();        
            println!("{}", input);
            let command = prepare_command(&input).expect("Error whilst parsing input");
            execute_command(&command, &mut table);
        }
    }


fn execute_command(cmd: &Command, table: &mut Table) {
    match cmd.command_type {
        CommandType::Insert => {
            let row_to_insert = cmd.row_to_insert.clone().unwrap();
            println!("This is an insert");
            table.insert(row_to_insert);
        },
        CommandType::Select => {
            
            for i in 0..table.pages.len() {
                for j in 0..table.pages[i].rows.len() {
                    let row = &table.pages[i].rows[j];
                    let a = str::from_utf8(&row.username).unwrap();
                    let b = str::from_utf8(&row.email).unwrap();
                    println!("ID: {}, Username: {}, Email {}", row.id, a, b);
                }
            }
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

