use crate::utils::pckg;
use colored::{Color, ColoredString, Colorize};
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::env::Env;
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

enum LookingFor {
    Flag,
    TextColor,
    BackgroundColor,
    Style,
}

fn add_styles(string: ColoredString, styles: Vec<String>) -> ColoredString {
    let mut styled_string = string;

    for style in &styles {
        styled_string = match style.as_str() {
            "bold" => styled_string.bold(),
            "underline" => styled_string.underline(),
            "italic" => styled_string.italic(),
            "dimmed" => styled_string.dimmed(),
            "blink" => styled_string.blink(),
            "strikethrough" => styled_string.strikethrough(),
            _ => styled_string,
        };
    }

    styled_string
}

fn add_color(
    string: ColoredString,
    color_option: Option<String>,
    background: bool,
) -> ColoredString {
    match color_option {
        Some(color) => {
            if color.starts_with("rgb_") {
                let split = color[4..].split("_");
                let rgb_values = split.collect::<Vec<&str>>();

                if rgb_values.len() == 3 {
                    let red: u8 = rgb_values[0].parse().unwrap_or(255);
                    let green: u8 = rgb_values[1].parse().unwrap_or(255);
                    let blue: u8 = rgb_values[2].parse().unwrap_or(255);

                    if background {
                        string.on_truecolor(red, green, blue)
                    } else {
                        string.truecolor(red, green, blue)
                    }
                } else {
                    string
                }
            } else {
                let bright = color.starts_with("bright_");
                let color_string = if bright {
                    color[7..].to_string()
                } else {
                    color
                };

                let color_parse_result: Result<Color, _> = color_string.parse();
                match color_parse_result {
                    Ok(color_parsed) => {
                        if bright {
                            match color_parsed {
                                Color::Red => {
                                    if background {
                                        string.on_bright_red()
                                    } else {
                                        string.bright_red()
                                    }
                                }
                                Color::Black => {
                                    if background {
                                        string.on_bright_black()
                                    } else {
                                        string.bright_black()
                                    }
                                }
                                Color::Green => {
                                    if background {
                                        string.on_bright_green()
                                    } else {
                                        string.bright_green()
                                    }
                                }
                                Color::Yellow => {
                                    if background {
                                        string.on_bright_yellow()
                                    } else {
                                        string.bright_yellow()
                                    }
                                }
                                Color::Blue => {
                                    if background {
                                        string.on_bright_blue()
                                    } else {
                                        string.bright_blue()
                                    }
                                }
                                Color::Magenta => {
                                    if background {
                                        string.on_bright_magenta()
                                    } else {
                                        string.bright_magenta()
                                    }
                                }
                                Color::Cyan => {
                                    if background {
                                        string.on_bright_cyan()
                                    } else {
                                        string.bright_cyan()
                                    }
                                }
                                _ => string,
                            }
                        } else if background {
                            string.on_color(color_parsed)
                        } else {
                            string.color(color_parsed)
                        }
                    }

                    Err(_) => string,
                }
            }
        }
        None => string,
    }
}

pub(crate) fn run_print(env: &mut Env, arguments: Vec<String>) -> CommandResult {
    // collect options
    let mut styles = vec![];
    let mut text_color = None;
    let mut background_color = None;
    let mut index = 0;
    let mut looking_for = LookingFor::Flag;
    for argument in &arguments {
        index = index + 1;

        match looking_for {
            LookingFor::Flag => match argument.as_str() {
                "--style" | "-s" => looking_for = LookingFor::Style,
                "--color" | "-c" => looking_for = LookingFor::TextColor,
                "--background-color" | "-bgc" => looking_for = LookingFor::BackgroundColor,
                _ => break,
            },
            LookingFor::Style => {
                styles.push(argument.to_string());
                looking_for = LookingFor::Flag;
            }
            LookingFor::TextColor => {
                text_color = Some(argument.to_string());
                looking_for = LookingFor::Flag;
            }
            LookingFor::BackgroundColor => {
                background_color = Some(argument.to_string());
                looking_for = LookingFor::Flag;
            }
        }
    }
    if index > 0 {
        index = index - 1;
    }

    // generate whole string
    let mut string = String::new();
    let mut count = 0;
    for argument in &arguments[index..] {
        count = count + 1;
        string.push_str(argument);
        string.push(' ');
    }
    if count > 0 {
        string.remove(string.len() - 1);
    }

    // add colors
    let mut styled_string = string.normal();
    styled_string = add_color(styled_string, text_color, false);
    styled_string = add_color(styled_string, background_color, true);
    styled_string = add_styles(styled_string, styles);

    match write!(env.out, "{}", styled_string) {
        Ok(_) => CommandResult::Continue(Some(count.to_string())),
        Err(error) => CommandResult::Error(error.to_string()),
    }
}

#[derive(Clone)]
pub(crate) struct CommandImpl {
    package: String,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        pckg::concat(&self.package, "Print")
    }

    fn aliases(&self) -> Vec<String> {
        vec!["print".to_string()]
    }

    fn help(&self) -> String {
        include_str!("help.md").to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn requires_context(&self) -> bool {
        true
    }

    fn run_with_context(
        &self,
        arguments: Vec<String>,
        _state: &mut HashMap<String, StateValue>,
        _variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        _line: usize,
        env: &mut Env,
    ) -> CommandResult {
        run_print(env, arguments)
    }
}

pub(crate) fn create(package: &str) -> Box<dyn Command> {
    Box::new(CommandImpl {
        package: package.to_string(),
    })
}
