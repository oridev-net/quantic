use crate::datatypes;

pub struct Node {
	pub transform: datatypes::Transform,
}

impl std::fmt::Debug for Node {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		write!(f, "Node: {{{:?}}}", self.transform)
	}
}