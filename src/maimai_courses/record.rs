use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

/// An enum showing if the course is passed
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Passed,
    Failed,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Status::Passed => write!(f, "Passed"),
            Status::Failed => write!(f, "Failed"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Record {
    pub life: u32,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRecords {
    pub fullname: String,
    pub records: HashMap<u32, Record>,
}

pub type Records = HashMap<u64, UserRecords>;
