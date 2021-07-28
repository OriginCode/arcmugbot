use teloxide::utils::command::{BotCommand, ParseError};

pub type Results = Vec<(u32, u32, u32)>;

// Commands
#[derive(BotCommand)]
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
        description = "calculate the life remains (/calc LIFE [[GREAT|GOOD|MISS]..])",
        parse_with = "score_parser"
    )]
    Calc { life: u32, results: Results },
    #[command(
        description = "submit maimai course of current month (/submit LEVEL [[GREAT|GOOD|MISS]..])",
        parse_with = "score_parser"
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
}

fn next_str_into_u32(from: Option<&str>) -> Result<u32, ParseError> {
    from.ok_or_else(|| ParseError::Custom("invalid submission".into()))?
        .parse::<u32>()
        .map_err(|e| ParseError::IncorrectFormat(e.into()))
}

macro_rules! yield_into {
    ($v:expr => ($x:ident)) => {
        $x = next_str_into_u32($v.next())?;
    };
    ($v:expr => ($x:ident, $($y:ident),+)) => {
        $x = next_str_into_u32($v.next())?;
        yield_into!($v => ($($y),+));
    }
}

/// Parse a score calc command
fn score_parser(input: String) -> Result<(u32, Results), ParseError> {
    // The command should satisfy this pattern:
    // /command MARKER [[GREAT|GOOD|MISS]..]
    //
    // For example:
    // /calc 500 10,3,1 13,2,0 3,0,0 0,0,0
    let mut parts = input.split_whitespace();
    let marker = next_str_into_u32(parts.next())?;
    let mut results = Vec::new();
    for i in parts {
        let mut result = i.splitn(3, ',');
        let great;
        let good;
        let miss;
        yield_into!(result => (great, good, miss));
        results.push((great, good, miss))
    }

    Ok((marker, results))
}
