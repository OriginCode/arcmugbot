use crate::commands::Results;
use std::collections::VecDeque;

pub const RULE: [u32; 3] = [2, 3, 5];

#[derive(Clone)]
pub struct Submission {
    pub life: u32,
    pub heal: u32,
    pub rule: [u32; 3],
    pub results: Results,
}

impl Default for Submission {
    fn default() -> Self {
        Self {
            life: 900,
            heal: 20,
            rule: RULE,
            results: VecDeque::new(),
        }
    }
}
