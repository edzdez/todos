use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Tasks {
    pub tasks: Vec<Task>
}

impl Tasks {
    pub fn new() -> Tasks {
        Tasks {
            tasks: Vec::<Task>::new()
        }
    }

    pub fn sort_tasks(&mut self) {
        self.tasks.sort_by_key(|task| task.urgency);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    contents: String,
    urgency: Urgency,
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.urgency {
            Urgency::High => write!(f, "\x1b[31;1;4m{}\x1b[0m", self.contents),
            Urgency::Medium => write!(f, "\x1b[33;1m{}\x1b[0m", self.contents),
            Urgency::Low => write!(f, "\x1b[32m{}\x1b[0m", self.contents),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum Urgency {
    Low,
    Medium,
    High,
}