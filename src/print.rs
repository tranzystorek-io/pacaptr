//! Output messages and prompts.

#![allow(missing_docs, clippy::module_name_repetitions)]

use colored::Colorize;

use crate::exec::Cmd;

pub(crate) static PROMPT_CANCELED: &str = "Canceled";
pub(crate) static PROMPT_PENDING: &str = "Pending";
pub(crate) static PROMPT_RUN: &str = "Running";
pub(crate) static PROMPT_INFO: &str = "Info";
pub static PROMPT_ERROR: &str = "Error";

/// The right indentation to be applied on prompt prefixes.
static PROMPT_INDENT: usize = 9;

macro_rules! prompt_format {
    () => {
        "{:>indent$}"
    };
}

macro_rules! cmd_format {
    () => {
        concat!(prompt_format!(), " `{}`")
    };
}

macro_rules! msg_format {
    () => {
        concat!(prompt_format!(), " {}")
    };
}

macro_rules! question_format {
    () => {
        concat!(prompt_format!(), " {}? ")
    };
}

/// Prints out the command after the given prompt.
pub(crate) fn print_cmd(cmd: &Cmd, prompt: &str) {
    println!(
        cmd_format!(),
        prompt.green().bold(),
        cmd,
        indent = PROMPT_INDENT
    );
}

/// Prints out a message after the given prompt.
pub(crate) fn print_msg(msg: &str, prompt: &str) {
    println!(
        msg_format!(),
        prompt.green().bold(),
        msg,
        indent = PROMPT_INDENT
    );
}

/// Prints out an error after the given prompt.
pub fn print_err(err: impl std::fmt::Display, prompt: &str) {
    eprintln!(
        msg_format!(),
        prompt.bright_red().bold(),
        format_args!("{err:#}"),
        indent = PROMPT_INDENT
    );
}

/// Prints out a question after the given prompt.
pub(crate) fn print_question(question: &str, options: &str) {
    print!(
        question_format!(),
        question.yellow(),
        options.underline(),
        indent = PROMPT_INDENT
    );
}
