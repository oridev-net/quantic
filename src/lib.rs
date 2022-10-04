pub mod ecs;
pub mod datatypes;
pub mod script;

pub fn default() -> Runner {
    Runner {
        loaded_nodes: vec![]
    }
}

pub struct Runner {
    loaded_nodes: Vec<ecs::Node>
}

pub trait Run {
    fn run(self) -> Result<(), &'static str>;
}

impl Run for Runner {
    fn run(mut self) -> Result<(), &'static str> {
        loop {
			self.loaded_nodes.push(ecs::Node {
				transform: datatypes::Transform {
					position: datatypes::Vec3 {
						x: 0,
						y: 0,
						z: 0
					},
					rotation: datatypes::Vec3 {
						x: 0,
						y: 100,
						z: 0
					},
					scale: datatypes::Vec3 {
						x: 0,
						y: 0,
						z: 0
					},
				}
			});
            println!("{:?}", self.loaded_nodes);
        }
    }
}