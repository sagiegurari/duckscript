use crate::sdk::std::flowcontrol::get_line_key;
use crate::utils::state::get_core_sub_state_for_command;
use duckscript::runner;
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::env::Env;
use duckscript::types::instruction::{
    Instruction, InstructionMetaInfo, InstructionType, ScriptInstruction,
};
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

static END_STATE_KEY: &str = "end";
pub(crate) static END_COMMAND_NAME: &str = "end";

fn get_command(line: usize, state: &mut HashMap<String, StateValue>) -> Option<String> {
    let key = get_line_key(line, state);
    let sub_state = get_core_sub_state_for_command(state, END_STATE_KEY.to_string());

    match sub_state.get(&key) {
        Some(state_value) => match state_value {
            StateValue::String(command) => Some(command.clone()),
            _ => {
                // remove corrupted data
                sub_state.remove(&key);

                None
            }
        },
        None => None,
    }
}

pub(crate) fn set_command(line: usize, state: &mut HashMap<String, StateValue>, command: String) {
    let key = get_line_key(line, state);
    let sub_state = get_core_sub_state_for_command(state, END_STATE_KEY.to_string());

    sub_state.insert(key, StateValue::String(command));
}

#[derive(Clone)]
pub(crate) struct CommandImpl {}

impl Command for CommandImpl {
    fn name(&self) -> String {
        END_COMMAND_NAME.to_string()
    }

    fn help(&self) -> String {
        "".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn requires_context(&self) -> bool {
        true
    }

    fn run_with_context(
        &self,
        _arguments: Vec<String>,
        state: &mut HashMap<String, StateValue>,
        variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        instructions: &Vec<Instruction>,
        commands: &mut Commands,
        line: usize,
        env: &mut Env,
    ) -> CommandResult {
        match get_command(line, state) {
            Some(command) => {
                let mut script_instruction = ScriptInstruction::new();
                script_instruction.command = Some(command);
                let mut instruction_meta_info = InstructionMetaInfo::new();
                instruction_meta_info.line = Some(line);
                let instruction = Instruction {
                    meta_info: instruction_meta_info,
                    instruction_type: InstructionType::Script(script_instruction),
                };

                let (command_result, _) = runner::run_instruction(
                    commands,
                    variables,
                    state,
                    instructions,
                    instruction,
                    line,
                    env,
                );

                command_result
            }
            None => CommandResult::Continue(None),
        }
    }
}

pub(crate) fn create() -> Box<dyn Command> {
    Box::new(CommandImpl {})
}
