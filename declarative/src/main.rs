struct Base {}

impl Base {
    fn execute() -> i32 {
        println!("Base executed");
        0
    }
}

struct Minteable {}

impl Minteable {
    fn execute() -> i32 {
        println!("Minteable executed");
        12
    }
}

struct Burnable {}

impl Burnable {
    fn execute() -> i32 {
        println!("Burnable executed");
        32
    }
}

macro_rules! compose {
	// ($name:ident($($parent:ident),+) {$($field_name:ident : $field_type:ty),+}) => {
	// 	#[allow(dead_code)]
	// 	#[derive(Debug)]
	// 	struct $name { $($field_name: $field_type),+ }
	// 	impl $name {
	// 		fn execute(&self) {
	// 			let results = [$($parent :: execute()),+];
	// 			println!("{} executed, internals {:?}, results {:?}", stringify!($name), self, results);
	// 		}
	// 	}
	// };
	($name:ident($($parent:ident),+) {$($field_name:ident : $field_type:ty),+} $($custom:expr),*) => {
		#[allow(dead_code)]
		#[derive(Debug)]
		struct $name { $($field_name: $field_type),+ }
		impl $name {
			fn execute(&self) {
				let results = [$($parent :: execute()),+$(, $custom())*];
				println!("{} executed, internals {:?}, results {:?}", stringify!($name), self, results);
			}
		}
	};
}

fn main() {
    {
		println!("--------------------------------------------------------------------------------");
        println!("Testing CsdScript(Base)...");
        compose! { CsdScript(Base) { sample_data: i32 } }
        let contract = CsdScript { sample_data: 13i32 };
        contract.execute();
    }
	{
		println!("--------------------------------------------------------------------------------");
        println!("Testing CsdScript(Minteable)...");
        compose! { CsdScript(Minteable) { sample_data: i32 } }
        let contract = CsdScript { sample_data: 13i32 };
        contract.execute();
    }
	{
		println!("--------------------------------------------------------------------------------");
        println!("Testing CsdScript(Burnable)...");
        compose! { CsdScript(Burnable) { sample_data: i32 } }
        let contract = CsdScript { sample_data: 13i32 };
        contract.execute();
    }
	{
		println!("--------------------------------------------------------------------------------");
        println!("Testing CsdScript(Base,Minteable,Burnable)...");
        compose! { CsdScript(Base,Minteable,Burnable) { sample_data: i32 } }
        let contract = CsdScript { sample_data: 13i32 };
        contract.execute();
    }
	{
		println!("--------------------------------------------------------------------------------");
        println!("Testing CsdScript(Base) |custom| ...");
        compose! { CsdScript(Base,Minteable,Burnable) { sample_data: i32 } || { 
			println!("Custom executed");
        	1
		}}
        let contract = CsdScript { sample_data: 13i32 };
        contract.execute();
    }
	println!("--------------------------------------------------------------------------------");
}
