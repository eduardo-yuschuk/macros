#![allow(unused)]
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data::Struct, DataStruct, DeriveInput, Field, Fields::Named, FieldsNamed,
    Path, Type, TypePath,
};

#[proc_macro_derive(Contract)]
pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input as DeriveInput);
    // let fields = if let Struct(DataStruct {
    //     fields: Named(FieldsNamed { ref named, .. }),
    //     ..
    // }) = data
    // {
    //     named
    // } else {
    //     panic!("Unsupported!")
    // };
    // let entity = Entity {
    //     name: format!("{}", ident),
    //     fields: fields
    //         .iter()
    //         .filter_map(|field| get_entity_field(field))
    //         .collect(),
    // };
    // let fields: Vec<String> = entity.fields.iter().map(|f| f.name.to_string()).collect();
    // let columns = fields.join(", ");
    // let select_string = format!("select {} from {};", &columns, &entity.name);

    let result = quote! {
        impl #ident {
        }
    };
    result.into()
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
