struct Core {}

impl Core {
    fn execute() -> i32 {
        println!("Core executed");
        12
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

macro_rules! define_script {
	($name:ident($($parent:ident),+) {$($field_name:ident : $field_type:ty),*} $($custom:expr),*) => {
		#[allow(dead_code)]
		#[derive(Debug)]
		struct $name { $($field_name: $field_type),* }
		impl $name {
			fn execute(&self) {
				let results = [$($parent :: execute()),+$(, $custom())*];
				println!("{} executed, internals {:?}, results {:?}", stringify!($name), self, results);
			}
		}
	};
}

/*
Core que crea el NFT con un hash
Agregamos NotBurnable (plugin)

Cambiamos de Burnable a NotBurnable
Salimos con la primera falla

Pensar que pasaría si queremos bypassear lo que está en el core...!
*/

fn main() {
    // test case #1
	{
        // type definition
        define_script! { ComposedScript(Core, Minteable) { } }
        // instantiation and execution
        let contract = ComposedScript { };
        contract.execute();
    }
    // test case #2
	{
        // type definition
        define_script! { ComposedScript(Core, Minteable, Burnable) { sample_attribute: i32 } }
        // instantiation and execution
        let script = ComposedScript { sample_attribute: 13i32 };
        script.execute();
    }
    // test case #3
	{
        // type definition
        define_script! { ComposedScript(Core, Minteable) { } || { 
			println!("Custom behavior");
        	0
		}}
        // instantiation and execution
        let contract = ComposedScript { };
        contract.execute();
    }
}
