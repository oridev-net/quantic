#[path = "uuid.rs"]
mod uuid;

use uuid::UUID;

pub struct QuanticScene {
	offset: [i32; 3],
	id: UUID
}

pub fn default() -> QuanticScene {
	QuanticScene {
		offset: [0, 0, 0],
		id: uuid::generate_UUID()
	}
}