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
	fn load_nodes(self, nodes: Vec<ecs::Node>) -> Runner;
}

impl Run for Runner {
    fn run(self) -> Result<(), &'static str> {
        for loaded_node in self.loaded_nodes {
			println!("{:?}", loaded_node);
		}
		Ok(())
    }
	fn load_nodes(mut self, nodes: Vec<ecs::Node>) -> Runner {
		for node in nodes {
			self.loaded_nodes.push(node);
		}
		self
	}
}