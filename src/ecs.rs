#[path = "uuid.rs"]
mod uuid;

use uuid::UUID;

pub struct Scene {
	offset: [i32; 3],
	id: UUID
}

pub fn default() -> Scene {
	Scene {
		offset: [0, 0, 0],
		id: uuid::generate_UUID()
	}
}