pub mod ecs;

use ecs::Scene;

pub fn default() -> Runner {
    Runner {
        loaded_scenes: vec![]
    }
}

pub struct Runner {
    loaded_scenes: Vec<Scene>
}

pub trait RunnerImpl {
    fn run(self) -> Result<(), &'static str>;
}

impl RunnerImpl for Runner {
    fn run(mut self) -> Result<(), &'static str> {
        loop {
            println!("{}", "p");
        }
    }
}