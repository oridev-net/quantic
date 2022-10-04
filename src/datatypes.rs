pub struct Vec3 {
	pub x: i32,
	pub y: i32,
	pub z: i32
}

impl std::fmt::Debug for Vec3 {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		write!(f, "x: {:?}, y: {:?}, z: {:?}", self.x, self.y, self.z)
	}
}

pub struct Transform {
	pub position: Vec3,
	pub rotation: Vec3,
	pub scale: Vec3
}

impl std::fmt::Debug for Transform {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
		write!(f, "Transform: {{position: {{{:?}}}, rotation: {{{:?}}}, scale: {{{:?}}}}}", self.position, self.rotation, self.scale)
	}
}