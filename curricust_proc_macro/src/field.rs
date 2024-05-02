use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma,
    AngleBracketedGenericArguments, AttrStyle, ExprPath, Field,
    GenericArgument, Ident, Path, PathArguments, PathSegment, Type, TypePath,
};


pub struct MyField {
    pub field_name: Ident,
    pub ty: TokenStream,
    pub is_optional: bool,
    pub has_text_with_attributes_attr: bool,
    pub span: Span,
}

pub fn dummy_string_field(name: &str) -> MyField {
    MyField {
        field_name: Ident::new(&name, Span::call_site()),
        ty: quote! { std::string::String },
        is_optional: false,
        has_text_with_attributes_attr: false,
        span: Span::call_site(),
    }
}

pub fn parse_fields(fields: &Punctuated<Field, Comma>) -> impl Iterator<Item = MyField> + '_ {
    fields.iter().map(|f| {
        let field_name = f.ident.as_ref().expect("Missing field name").clone();
        let (ty, is_optional) = parse_type(&f.ty);
        let has_text_with_attributes_attr = f
            .attrs
            .iter()
            .filter(|attr| matches!(attr.style, AttrStyle::Outer))
            .filter_map(|attr| attr.meta.require_list().ok())
            .any(|attr| {
                attr.path.is_ident("cv_element_builder")
                    && if let Ok(path) = &attr.parse_args::<ExprPath>() {
                        path.path.is_ident("text_with_attributes")
                    } else {
                        false
                    }
            });

        MyField {
            field_name,
            ty: ty.into_token_stream(),
            is_optional,
            has_text_with_attributes_attr,
            span: f.span(),
        }
    })
}

pub fn parse_type(ty: &Type) -> (Type, bool) {
    if let Type::Path(TypePath {
        qself: None,
        path: Path { segments, .. },
    }) = ty
    {
        let segment_path = segments.iter().fold(String::new(), |mut acc, v| {
            acc.push_str(&v.ident.to_string());
            acc.push(':');
            acc
        });

        if vec!["Option:", "std:option:Option:", "core:option:Option:"]
            .contains(&segment_path.as_str())
        {
            if let Some(PathSegment {
                arguments:
                    PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }),
                ..
            }) = segments.last()
            {
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
