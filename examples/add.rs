use quantic;
use quantic::Run;
mod add_mod;

fn main() {
	quantic::default()
		.load_nodes(vec![quantic::ecs::Node {
			transform: quantic::datatypes::Transform {
				position: quantic::datatypes::Vec3 {
					x: 0,
					y: 0,
					z: 0
				},
				rotation: quantic::datatypes::Vec3 {
					x: 0,
					y: 0,
					z: 0
				},
				scale: quantic::datatypes::Vec3 {
					x: 0,
					y: 0,
					z: 0
				}
			}
		}])
		.load_nodes(vec![quantic::ecs::Node {
			transform: quantic::datatypes::Transform {
				position: quantic::datatypes::Vec3 {
					x: 0,
					y: 0,
					z: 0
				},
				rotation: quantic::datatypes::Vec3 {
					x: 0,
					y: 0,
					z: 0
				},
				scale: quantic::datatypes::Vec3 {
					x: 0,
					y: 0,
					z: 0
				}
			}
		}])
		.load_nodes(vec![quantic::ecs::Node {
			transform: quantic::datatypes::Transform {
				position: quantic::datatypes::Vec3 {
					x: 0,
					y: 0,
					z: 0
				},
				rotation: quantic::datatypes::Vec3 {
					x: 0,
					y: 0,
					z: 0
				},
				scale: quantic::datatypes::Vec3 {
					x: 0,
					y: 0,
					z: 0
				}
			}
		}])
		.run()
		.expect("QuanticInternal");
}