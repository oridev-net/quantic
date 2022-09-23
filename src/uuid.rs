pub type UUID = u32;

use rand::Rng;

pub fn generate_UUID() -> UUID {
	let mut rng = rand::thread_rng();
	rng.gen()
}