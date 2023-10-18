use proc_macro2::Span;
use syn::{parse_macro_input, DeriveInput, Fields, spanned::Spanned, AttrStyle, DataStruct, Ident, Type, punctuated::Punctuated, Field, token::Comma, ExprPath, TypePath, Path, PathSegment, PathArguments, AngleBracketedGenericArguments, GenericArgument};
use quote::{quote, quote_spanned};

#[proc_macro_derive(CvElementBuilder, attributes(cv_element_builder))]
pub fn derive_cv_element_builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let visibility = input.vis;
    let name = &input.ident;
    let builder_name = Ident::new((input.ident.to_string() + "Builder").as_str(), input.ident.span());
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    if let syn::Data::Struct(DataStruct { fields: Fields::Named(ref fields), .. }) = &input.data {

        let recurse_fields = parse_fields(&fields.named).into_iter().map(|f| {
            let MyField { field_name, ty, is_optional: _, has_text_with_attributes_attr, span } = f;
            if has_text_with_attributes_attr {
                quote_spanned! {span=>
                    #field_name: std::vec::Vec<crate::element::text_with_attributes::TextWithAttributes>,
                }
            } else {
                quote_spanned! {span=>
                    #field_name: std::option::Option<#ty>,
                }
            }
        });

        let recurse_functions = parse_fields(&fields.named).into_iter().map(|f| {
            let MyField { field_name, ty, is_optional: _, has_text_with_attributes_attr, span } = f;

            if has_text_with_attributes_attr {
                let fun_name = Ident::new(("add_".to_string() + &field_name.to_string()).as_str(), field_name.span());
                quote_spanned! {span=>
                    pub fn #fun_name(&mut self, e: crate::element::text_with_attributes::TextWithAttributes) -> &mut Self {
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

        let recurse_build = parse_fields(&fields.named).into_iter().map(|f| {
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
                        #field_name: crate::element::text_with_attributes::TextWithAttributesCollection::into_best_matching(self.#field_name, active_attrs)
                            .ok_or(#field_name_error)?,
                    }
                },
                (true, true) => {
                    quote_spanned! {span=>
                        #field_name: crate::element::text_with_attributes::TextWithAttributesCollection::into_best_matching(self.#field_name, active_attrs),
                    }
                },
            }
        });

        let recurse_constructor = parse_fields(&fields.named).into_iter().map(|f| {
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

                pub fn build(self, active_attrs: &[String]) -> std::result::Result<#name::#ty_generics, std::string::String> {
                    std::result::Result::Ok(#name::#ty_generics {
                        #(#recurse_build)*
                    })
                }
            }

            impl #impl_generics #name #ty_generics #where_clause {
                #visibility fn builder() -> #builder_name::#ty_generics {
                    #builder_name::#ty_generics {
                        #(#recurse_constructor)*
                    }
                }
            }
        }.into()
    } else {
        unimplemented!()
    }
}

struct MyField {
    field_name: Ident,
    ty: Type,
    is_optional: bool,
    has_text_with_attributes_attr: bool,
    span: Span,
}

fn parse_fields(fields: &Punctuated<Field, Comma>) -> Vec<MyField> {
    fields.iter().map(|f| {
        let field_name = f.ident.as_ref().expect("Missing field name").clone();
        let (ty, is_optional) = parse_type(&f.ty);
        let has_text_with_attributes_attr = f.attrs.iter()
            .filter(|attr| matches!(attr.style, AttrStyle::Outer))
            .filter_map(|attr| attr.meta.require_list().ok())
            .any(|attr| attr.path.is_ident("cv_element_builder") && if let Ok(path) = &attr.parse_args::<ExprPath>() {
                path.path.is_ident("text_with_attributes")
            } else {
                false
            });

        MyField { field_name, ty, is_optional, has_text_with_attributes_attr, span: f.span() }
    })
    .collect()
}

fn parse_type(ty: &Type) -> (Type, bool) {
    if let Type::Path(TypePath { qself: None, path: Path { segments, .. } }) = ty {
        let segment_path = segments.iter().fold(String::new(), |mut acc, v| {
            acc.push_str(&v.ident.to_string());
            acc.push(':');
            acc
        });

        if vec!["Option:", "std:option:Option:", "core:option:Option:"].contains(&segment_path.as_str()) {
            if let Some(PathSegment { arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }), .. }) = segments.last() {
                if let Some(GenericArgument::Type(res)) = args.first() {
                    if args.len() == 1 {
                        return (res.clone(), true);
                    }
                }
            }
        }
    };

    (ty.clone(), false)
}