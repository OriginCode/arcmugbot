use serde::Deserialize;
use std::fmt;

/// maimai difficulties
#[derive(Deserialize, Debug)]
pub enum Difficulty {
    Easy,
    Advanced,
    Expert,
    Master,
    ReMaster,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "Easy"),
            Difficulty::Advanced => write!(f, "Advanced"),
            Difficulty::Expert => write!(f, "Expert"),
            Difficulty::Master => write!(f, "Master"),
            Difficulty::ReMaster => write!(f, "Re:Master"),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Song {
    pub title: String,
    pub difficulty: Difficulty,
    pub level: String,
}

#[derive(Deserialize, Debug)]
pub struct Course {
    pub name: String,
    pub life: u32,
    pub heal: u32,
    pub songs: Vec<Song>,
}

pub type Courses = Vec<Course>;