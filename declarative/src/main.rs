struct Minteable {
}

impl Minteable {
	fn execute() -> i32 {
		println!("Minteable executed");
		0
	}
}

struct Burnable {
}

impl Burnable {
	fn execute() -> i32 {
		println!("Burnable executed");
		32
	}
}

macro_rules! define_contract {
	($name:ident($($parent:ident),+) {$($field_name:ident : $field_type:ty),+}) => {
		#[allow(dead_code)]
		#[derive(Debug)]
		struct $name { $($field_name: $field_type),+ }
		impl $name {
			fn execute(&self) {
				let results = [$($parent :: execute()),+];
				println!("{} executed, internals {:?}, results {:?}", stringify!($name), self, results);
			}
		}
	};
}

define_contract! { CsdScript(Minteable,Burnable) { sample_data: i32 } }

fn main() {
	let contract = CsdScript { sample_data: 13i32 };
	contract.execute();
}
