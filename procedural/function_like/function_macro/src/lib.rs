#![allow(unused)]
extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro::TokenTree::Ident;
use proc_macro::TokenTree::Literal;
use proc_macro::TokenTree::Group;

#[proc_macro]
pub fn make_answer(input: TokenStream) -> TokenStream {
    let mut code = "".to_owned();
    for token in input.clone().into_iter() {
        // match token {
        //     Group(s) => {
        //         println!("\n\ns {:?}", s);
        //         for inner in s.stream() {
        //             match inner {
        //                 Literal(name) => {
        //                     code.push_str(&format!("struct Contract{};", name).to_owned());
        //                 },
        //                 _ => {
        //                 }
        //             }
        //         }
        //     }
        //     _ => {
        //     }
        // }
        match token {
            Ident(name) => {
                code.push_str(&format!("struct Contract{};", name).to_owned());
                //println!("{}", name);
            },
            _ => {
                
            }
        }
    }
    println!("");
    println!("");
    println!("{:?}", input);
    println!("{}", input);
    println!("");
    println!("");
    let to_parse = code.as_str();
    println!("to_parse {}", to_parse);
    to_parse.parse().unwrap()
    /*
    "
    fn answer() -> u32 {
        41
    }
    "
    .parse()
    .unwrap()
    */
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
