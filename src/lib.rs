pub mod ecs;
pub mod uuid;

use ecs::Scene;

pub fn default() -> Runner {
    Runner {
        val: 0,
        loaded_scenes: vec![]
    }
}

pub struct Runner {
    val: u64,
    loaded_scenes: Vec<Scene>
}

pub trait RunnerImpl {
    fn add(self, amount: u64) -> Self;
    fn run(self) -> Result<(), &'static str>;
}

impl RunnerImpl for Runner {
    fn add(mut self, amount: u64) -> Self {
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