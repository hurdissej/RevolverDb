use super::command_error::CommandError as CommandError;
use super::test_table::Row;

pub fn prepare_command<'a>(parsed_input: &'a String) -> Result<Command, CommandError> {
    if parsed_input.starts_with("insert"){
        let a: Vec<&str> = parsed_input.split_whitespace().collect();
        return Ok(Command {
        command_type: CommandType::Insert,
        //TODO we need ber validation here or just replace with a proper grammar
        row_to_insert: Some(Row::new(a[1].parse::<i32>().unwrap(), a[2], a[3]))
        })
    }

    if parsed_input.starts_with("select"){
        return Ok(Command {
        command_type: CommandType::Select,
        row_to_insert: None
        })
    }

    Err(CommandError)
}

pub struct Command {
    pub command_type: CommandType,
    pub row_to_insert: Option<Row>
}

#[derive(Debug)]
pub enum CommandType {
    Insert, 
    Select
}