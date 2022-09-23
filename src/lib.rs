pub mod ecs;
pub mod uuid;

use ecs::{QuanticScene};

pub fn default() -> QuanticRunner {
    QuanticRunner {
        val: 0,
        loaded_scenes: vec![]
    }
}

pub struct QuanticRunner {
    val: u32,
    loaded_scenes: Vec<QuanticScene>
}

pub trait QuanticRunnerImpl {
    fn add(self, amount: u32) -> Self;
    fn run(self) -> Result<(), &'static str>;
}

impl QuanticRunnerImpl for QuanticRunner {
    fn add(mut self, amount: u32) -> Self {
        self.val += amount;
        self
    }
    fn run(mut self) -> Result<(), &'static str> {
        loop {
            println!("{}", self.val);
            self.val += 1;
        }
    }
}