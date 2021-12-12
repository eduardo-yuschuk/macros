#![allow(unused)]
extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro::TokenTree::Ident;
use proc_macro::TokenTree::Punct;
use proc_macro::TokenTree::Literal;
use proc_macro::TokenTree::Group;

#[proc_macro]
pub fn make_answer(input: TokenStream) -> TokenStream {
    println!("-- input tokens -----------------------------------------");
    println!("{:?}", input);
    println!("---------------------------------------------------------");
    let mut code = "".to_owned();
    for token in input.clone().into_iter() {
        match token {
            Ident(name) => {
                code.push_str(&format!("struct Contract{};\n", name).to_owned());
            },
            Punct(_) => {},
            _ => panic!("Invalid input token {:?}", token)
        }
    }
    let to_parse = code.as_str();
    println!("-- generated code ---------------------------------------");
    println!("{}", to_parse);
    println!("---------------------------------------------------------");
    to_parse.parse().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
