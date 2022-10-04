use crate::datatypes;
use crate::script::Script;

pub struct Node {
	pub transform: datatypes::Transform,
	pub script: dyn Script
}

impl std::fmt::Debug for Node {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		write!(f, "{{transform: {:?}}}", self.transform)
	}
}