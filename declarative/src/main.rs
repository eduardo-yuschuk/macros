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

macro_rules! define_script {
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
        println!("Testing MyComposedScript(Minteable)...");
        // type definition
        define_script! { MyComposedScript(Minteable) { some_attribute: i32 } }
        // instantiation
        let contract = MyComposedScript { some_attribute: 13i32 };
        // execution
        contract.execute();
    }
	{
		println!("--------------------------------------------------------------------------------");
        println!("Testing MyComposedScript(Minteable,Burnable)...");
        // type definition
        define_script! { MyComposedScript(Minteable,Burnable) { some_attribute: i32 } }
        // instantiation
        let contract = MyComposedScript { some_attribute: 13i32 };
        // execution
        contract.execute();
    }
	{
		println!("--------------------------------------------------------------------------------");
        println!("Testing MyComposedScript(Minteable) |custom| ...");
        // type definition
        define_script! { MyComposedScript(Minteable) { some_attribute: i32 } || { 
			println!("Custom executed");
        	1
		}}
        // instantiation
        let contract = MyComposedScript { some_attribute: 13i32 };
        // execution
        contract.execute();
    }
	println!("--------------------------------------------------------------------------------");
}
