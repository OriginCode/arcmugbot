use std::{collections::VecDeque, str::SplitWhitespace};
use teloxide::utils::command::{BotCommand, ParseError};

use crate::maimai_courses::Submission;

pub type Results = VecDeque<[u32; 3]>;

// Commands
#[derive(BotCommand, Clone)]
#[command(
    rename = "lowercase",
    description = "The following commands are available:"
)]
pub enum Command {
    #[command(description = "pong!")]
    Ping,
    #[command(description = "display help")]
    Help,
    #[command(description = "display about")]
    About,
    #[command(
        description = "calculate the life remains (/calc LIFE HEAL [[GREAT,GOOD,MISS]..])",
        parse_with = "calc_parser"
    )]
    Calc { submission: Submission },
    #[command(
        description = "calculate the life remains using custom rule (/calc LIFE HEAL (RULE: [GREAT,GOOD,MISS]) [[GREAT,GOOD,MISS]..])",
        parse_with = "calc_parser"
    )]
    CalcCustom { submission: Submission },
    #[command(
        description = "submit maimai course of current month (/submit LEVEL [[GREAT,GOOD,MISS]..])",
        parse_with = "submit_parser"
    )]
    Submit { level: u32, results: Results },
    #[command(
        description = "check your course score (/score LEVEL)",
        parse_with = "split"
    )]
    Score { level: u32 },
    #[command(
        description = "get course details (/query LEVEL)",
        parse_with = "split"
    )]
    Query { level: u32 },
    #[command(description = "get players' passed courses")]
    Passed,
    #[command(
        description = "get rank for the course level (/rank LEVEL)",
        parse_with = "split"
    )]
    Rank { level: u32 },
    #[command(
        description = "get user's profile on Arcana with given game version and DJ name/IIDX ID (/iidxprofile VERSION DJ_NAME/IIDX_ID)",
        parse_with = "split"
    )]
    IIDXProfile { version: u32, param: String },
    #[command(
        description = "search IIDX music with given version and title (/iidxmusic VERSION TITLE)",
        parse_with = "split_into_two"
    )]
    IIDXMusic { version: u32, title: String },
    #[command(
        description = "get recent score (/iidxrecent VERSION DJ_NAME/IIDX_ID)",
        parse_with = "split"
    )]
    IIDXRecent { version: u32, param: String },
}

fn next_str_into_u32(from: Option<&str>) -> Result<u32, ParseError> {
    from.ok_or_else(|| ParseError::Custom("invalid input".into()))?
        .parse::<u32>()
        .map_err(|e| ParseError::IncorrectFormat(e.into()))
}

fn parse_score(parts: SplitWhitespace) -> Result<Results, ParseError> {
    let mut results = VecDeque::new();
    for i in parts {
        let mut result = i.splitn(3, ',');
        results.push_back([
            next_str_into_u32(result.next())?,
            next_str_into_u32(result.next())?,
            next_str_into_u32(result.next())?,
        ])
    }
    Ok(results)
}

/// Parse a score calc command
fn calc_parser(input: String) -> Result<(Submission,), ParseError> {
    // The command should satisfy this pattern:
    // /command LIFE HEAL [[GREAT,GOOD,MISS]..]
    //
    // For example:
    // /calc 500 30 10,3,1 13,2,0 3,0,0 0,0,0
    let mut parts = input.split_whitespace();
    let marker = next_str_into_u32(parts.next())?;
    let heal = next_str_into_u32(parts.next())?;
    let mut results = parse_score(parts)?;
    let mut rule = [2, 3, 5];
    if results.len() == 4 {
        rule = results.pop_front().unwrap();
    }

    Ok((Submission {
        life: marker,
        heal,
        rule,
        results,
    },))
}

/// Parse a submit command
fn submit_parser(input: String) -> Result<(u32, Results), ParseError> {
    // The command should satisfy this pattern:
    // /submit LEVEL [[GREAT,GOOD,MISS]..]
    //
    // For example:
    // /submit 10 10,3,1 13,2,0 3,0,0 0,0,0
    let mut parts = input.split_whitespace();
    let level = next_str_into_u32(parts.next())?;
    let results = parse_score(parts)?;

    Ok((level, results))
}

fn split_into_two(input: String) -> Result<(u32, String), ParseError> {
    let mut parts = input.splitn(2, ' ');
    Ok((
        next_str_into_u32(parts.next())?,
        parts.next().unwrap_or("").to_owned(),
    ))
}
