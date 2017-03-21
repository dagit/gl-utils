#![recursion_limit = "128"]
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[allow(non_snake_case)]
#[proc_macro_derive(VertexAttribFields)]
pub fn derive_VertexAttribFields(input: TokenStream) -> TokenStream
{
    let s   = input.to_string();
    let ast = syn::parse_macro_input(&s).unwrap();
    let gen = impl_VertexAttribFields(&ast);
    gen.parse().unwrap()
}

#[allow(non_snake_case)]
fn impl_VertexAttribFields(ast: &syn::MacroInput) -> quote::Tokens
{
    use syn::Body::*;
    let ident = &ast.ident;

    match &ast.body {
        &Enum(..)        => panic!("enums are not supported."),
        &Struct(ref var) => {
            let fields = var.fields();
            let field_ident = &fields.iter()
                .filter(|f| f.ident.is_some() )
                .map(|f| f.clone().ident.unwrap())
                .collect::<Vec<syn::Ident>>();

            quote! {
                implement_vertex_attrib!(#ident, #(#field_ident),*);
            }
        },
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
