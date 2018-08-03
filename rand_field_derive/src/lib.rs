extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate rand;

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use rand::Rng;
use syn::{Data, Field, Fields, Type};
use proc_macro2::Punct;

#[proc_macro_derive(RandField, attributes(choices, convert))]
pub fn rand_field(input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();

    let output: proc_macro2::TokenStream = {
        let ast = syn::parse2(input).unwrap();
        let result = impl_rand_field(&ast);

        result
    };

    output.into()
}

fn find_rand_type(data: &Data) -> Option<&Type> {
    if let Data::Struct(struct_data) = data {
        if let Fields::Unnamed(un_field) = &struct_data.fields {
            for field in &un_field.unnamed {
                return Some(&field.ty);
            }
        }
    }

    None
}

fn impl_rand_field(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let attributes: &Vec<syn::Attribute> = &ast.attrs;

    let data: &syn::Data = &ast.data;

    let mut convert = None;

    if let Some(type_name) = find_rand_type(data) {
        let mut choices = Vec::new();

        for attr in attributes {
            for seg in &attr.path.segments {
                if seg.ident == "choices" {
                    let token_root = attr.tts.clone().into_iter();

                    for token_tree in token_root {
                        if let TokenTree::Group(group) = token_tree {
                            for token_tree in group.stream() {
                                match token_tree {
                                    tt => {
                                        choices.push(tt);
                                    }
                                }
                            }
                        }
                    }
                }

                if seg.ident == "convert" {
                    let token_root = attr.tts.clone().into_iter();

                    for token_tree in token_root {
                        if let TokenTree::Group(group) = token_tree {
                            for token_tree in group.stream() {
                                if let TokenTree::Ident(ident) = token_tree {
                                    convert = Some(ident);
                                }
                            }
                        }
                    }
                }
            }
        }

        let index = rand::thread_rng().gen_range(0, choices.len());

        let return_statement = {
            if let Some(convert) = convert {
                quote! {
                    #name(#type_name::#convert(choice))
                }
            } else {
                quote! {
                    #name(choice)
                }
            }
        };

            quote! {
            impl RandField for #name {
                fn random() -> Self {
                    let choices = &[
                        #(#choices)*
                    ];
                    use ::rand_field::rand::{thread_rng, Rng};
                    let index = thread_rng().gen_range(0, choices.len());
                    let choice = choices[index];

                    #return_statement
                }
            }
            }
    } else {
        panic!("Could not find a good type for random derive")
    }
}
