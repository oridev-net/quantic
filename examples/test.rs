use quantic;
use quantic::RunnerImpl;

fn main() {
	quantic::default()
		.add(6900000000)
		.add(42000000)
		.run()
		.expect("QuanticInternal");
}