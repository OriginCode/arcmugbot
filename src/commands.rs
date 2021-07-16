use teloxide::utils::command::{BotCommand, ParseError};

pub type Results = Vec<(u32, u32, u32)>;

// Commands
#[derive(BotCommand)]
#[command(
    rename = "lowercase",
    description = "The following commands are available:"
)]
pub enum Command {
    #[command(description = "display help")]
    Help,
    #[command(
        description = "submit maimai course of current month",
        parse_with = "submission_parser"
    )]
    Submit {
        level: u32,
        results: Results,
    },
}

fn next_str_into_u32(from: Option<&str>) -> Result<u32, ParseError> {
    from
        .ok_or_else(|| ParseError::Custom("invalid submission".into()))?
        .parse::<u32>()
        .map_err(|e| ParseError::IncorrectFormat(e.into()))
}

macro_rules! yield_into {
    (($x:ident) = $v:expr) => {
        $x = next_str_into_u32($v.next())?;
    };
    (($x:ident, $($y:ident),+) = $v:expr) => {
        $x = next_str_into_u32($v.next())?;
        yield_into!(($($y),+) = $v);
    }
}

/// Parse a submission command
fn submission_parser(input: String) -> Result<(u32, Results), ParseError> {
    // The command should be in this pattern:
    // /submit <level> <[<GREAT>|<GOOD>|<MISS>]..>
    // For example:
    // /submit 1 10,3,1 13,2,0 3,0,0 0,0,0
    let mut parts = input.split_whitespace();
    let level = next_str_into_u32(parts.next())?;
    let mut results = Vec::new();
    for i in parts {
        let mut result = i.splitn(3, ',');
        let great;
        let good;
        let miss;
        yield_into!((great, good, miss) = result);
        results.push((great, good, miss))
    }

    Ok((level, results))
}
