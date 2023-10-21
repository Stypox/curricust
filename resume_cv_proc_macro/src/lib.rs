mod field;

use std::iter::once;

use field::{parse_fields, dummy_string_field, MyField};

use quote::{quote, quote_spanned};
use syn::{DataStruct, Fields, Ident, parse_macro_input, DeriveInput};

#[proc_macro_derive(CvElementBuilder, attributes(cv_element_builder))]
pub fn derive_cv_element_builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let visibility = input.vis;
    let name = &input.ident;
    let builder_name = Ident::new(
        (input.ident.to_string() + "Builder").as_str(),
        input.ident.span(),
    );
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let name_generics = if input.generics.params.is_empty() {
        quote! {}
    } else {
        quote! {
            ::#ty_generics
        }
    };

    if let syn::Data::Struct(DataStruct {
        fields: Fields::Named(ref fields),
        ..
    }) = &input.data
    {
        let recurse_fields = parse_fields(&fields.named)
            .chain(once(dummy_string_field("id")))
            .map(|f| {
                let MyField { field_name, ty, is_optional: _, has_text_with_attributes_attr, span } = f;
                if has_text_with_attributes_attr {
                    quote_spanned! {span=>
                        #field_name: std::vec::Vec<crate::attr::text_with_attributes::TextWithAttributes>,
                    }
                } else {
                    quote_spanned! {span=>
                        #field_name: std::option::Option<#ty>,
                    }
                }
            });

        let recurse_functions = parse_fields(&fields.named)
            .chain(once(dummy_string_field("id")))
            .map(|f| {
                let MyField { field_name, ty, is_optional: _, has_text_with_attributes_attr, span } = f;

                if has_text_with_attributes_attr {
                    let fun_name = Ident::new(("add_".to_string() + &field_name.to_string()).as_str(), field_name.span());
                    quote_spanned! {span=>
                        pub fn #fun_name(&mut self, e: crate::attr::text_with_attributes::TextWithAttributes) -> &mut Self {
                            self.#field_name.push(e);
                            self
                        }
                    }
                } else {
                    quote_spanned! {span=>
                        pub fn #field_name(&mut self, e: #ty) -> &mut Self {
                            self.#field_name = Some(e);
                            self
                        }
                    }
                }
            });

        let recurse_build = parse_fields(&fields.named)
            .map(|f| {
                let MyField { field_name, ty: _, is_optional, has_text_with_attributes_attr, span } = f;
                let field_name_error = format!("Missing {field_name}");

                match (has_text_with_attributes_attr, is_optional) {
                    (false, false) => {
                        quote_spanned! {span=>
                            #field_name: self.#field_name.ok_or(#field_name_error)?,
                        }
                    },
                    (false, true) => {
                        quote_spanned! {span=>
                            #field_name: self.#field_name,
                        }
                    },
                    (true, false) => {
                        quote_spanned! {span=>
                            #field_name: crate::attr::text_with_attributes::TextWithAttributesCollection::into_best_matching(self.#field_name, &active_attrs)
                                .ok_or(#field_name_error)?,
                        }
                    },
                    (true, true) => {
                        quote_spanned! {span=>
                            #field_name: crate::attr::text_with_attributes::TextWithAttributesCollection::into_best_matching(self.#field_name, &active_attrs),
                        }
                    },
                }
            });

        let recurse_constructor = parse_fields(&fields.named)
            .chain(once(dummy_string_field("id")))
            .map(|f| {
                let field_name = f.field_name;
                if f.has_text_with_attributes_attr {
                    quote_spanned! {f.span=>
                        #field_name: std::vec::Vec::new(),
                    }
                } else {
                    quote_spanned! {f.span=>
                        #field_name: std::option::Option::None,
                    }
                }
            });

        quote! {
            #visibility struct #builder_name #ty_generics #where_clause {
                #(#recurse_fields)*
            }

            impl #impl_generics #builder_name #ty_generics #where_clause {
                #(#recurse_functions)*

                pub fn build(self, ctx: &crate::attr::context::Context) -> std::result::Result<#name #name_generics, std::string::String> {
                    let active_attrs = ctx.get_active_attrs(self.id);

                    std::result::Result::Ok(#name #name_generics {
                        #(#recurse_build)*
                    })
                }
            }

            impl #impl_generics #name #ty_generics #where_clause {
                #visibility fn builder() -> #builder_name #name_generics {
                    #builder_name #name_generics {
                        #(#recurse_constructor)*
                    }
                }
            }
        }.into()
    } else {
        unimplemented!()
    }
}
