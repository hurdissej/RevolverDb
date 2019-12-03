use super::command_error::CommandError as CommandError;

pub fn prepare_command(parsed_input: String) -> Result<Command, CommandError> {
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

pub struct Command {
    pub command_type: CommandType
}

#[derive(Debug)]
pub enum CommandType {
    Insert, 
    Select
}