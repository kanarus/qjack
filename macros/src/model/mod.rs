/* Thanks to
    https://github.com/launchbadge/sqlx/blob/main/sqlx-macros-core/src/derives/row.rs
*/

mod attributes;
use attributes::{parse_child_attributes, parse_container_attributes, rename_all};

use quote::quote;
use proc_macro2::{TokenStream, Span};
use syn::{Error, parse2, ItemStruct, Lifetime, parse_quote, Stmt, Expr};


fn is_tuple_struct(item_struct: &ItemStruct) -> bool {
    item_struct.fields.iter()
        .all(|field| field.ident.is_none())
}

pub(super) fn derive_model(input: TokenStream) -> Result<TokenStream, Error> {
    let input = parse2::<ItemStruct>(input)?;

    if is_tuple_struct(&input) {
        derive_for_tuple_struct(input)
    } else {
        derive_for_key_value_struct(input)
    }
}

fn derive_for_key_value_struct(input: ItemStruct) -> Result<TokenStream, Error> {
    let original_generics = input.clone().generics;

    let fields = input.fields;
    let ident = &input.ident;

    let generics = &input.generics;
    let (lifetime, lifeteim_is_provided) = generics.lifetimes()
        .next()
        .map(|def| (def.lifetime.clone(), false))
        .unwrap_or_else(|| (Lifetime::new("'a", Span::call_site()), true));
    let (_, type_generics, _) = generics.split_for_impl();

    let mut generics = generics.clone();
    generics.params.insert(0, parse_quote!(R: ::qjack::__private__::Row));
    if lifeteim_is_provided {generics.params.insert(0, parse_quote!{ #lifetime })}

    let predicates = &mut generics.make_where_clause().predicates;
    predicates.push(parse_quote!{ &#lifetime ::std::primitive::str: ::qjack::__private__::ColumnIndex<R> });

    let containser_attributes = parse_container_attributes(&input.attrs)?;

    let reads: Vec<Stmt> = fields.iter()
        .filter_map(|field| -> Option<Stmt> {
            let ident = &field.ident.as_ref()?;
            let ty    = &field.ty;
            let attrs = parse_child_attributes(&field.attrs).unwrap();

            if attrs.is_skip {
                return Some(parse_quote!{ let #ident: #ty = Default::default(); })
            }

            let expr: Expr = match (attrs.is_flatten, attrs.try_from) {
                (true, None) => {
                    predicates.push(parse_quote!{ #ty: ::qjack::__private__::FromRow<#lifetime, R> });
                    parse_quote!{ <#ty as ::qjack::__private__::FromRow<#lifetime, R>>::from_row(row) }
                }
                (false, None) => {
                    predicates.push(parse_quote!{ #ty: ::qjack::__private__::Decode<#lifetime, <R as ::qjack::__private__::Row>::Database> });
                    predicates.push(parse_quote!{ #ty: ::qjack::__private__::Type<<R as ::qjack::__private__::Row>::Database> });
                    let ident_s = attrs.rename
                        .or_else(|| Some(ident.to_string().trim_start_matches("r#").to_owned()))
                        .map(|s| match containser_attributes.rename_all {
                            Some(pattern) => rename_all(&s, pattern),
                            None          => s,
                        })
                        .unwrap();
                    parse_quote!{ row.try_get(#ident_s) }
                }
                (true, Some(try_from)) => {
                    predicates.push(parse_quote!{ #try_from: ::qjack::__private__::FromRow<#lifetime, R> });
                    parse_quote!{ <#try_from as ::qjack::__private__::FromRow<#lifetime, R>>::from_row(row)
                        .and_then(|v|
                            <#ty as ::std::convert::TryFrom::<#try_from>>::try_from(v)
                            .map_err(|e| ::qjack::Error::ColumnNotFound("FromRow: try_from failed".to_string()))
                        )
                    }
                }
                (false, Some(try_from)) => {
                    predicates.push(parse_quote!{ #ty: ::qjack::__private__::Decode<#lifetime, <R as ::qjack::__private__::Row>::Database> });
                    predicates.push(parse_quote!{ #ty: ::qjack::__private__::Type<<R as ::qjack::__private__::Row>::Database> });
                    let ident_s = attrs.rename
                        .or_else(|| Some(ident.to_string().trim_start_matches("r#").to_owned()))
                        .map(|s| match containser_attributes.rename_all {
                            Some(pattern) => rename_all(&s, pattern),
                            None          => s,
                        })
                        .unwrap();
                    parse_quote!{ row.try_get(#ident_s)
                        .and_then(|v|
                            <#ty as ::std::convert::TryFrom::<#try_from>>::try_from(v)
                            .map_err(|e| ::qjack::Error::ColumnNotFound("FromRow: try_from failed".to_string()))
                        )
                    }
                }
            };

            if attrs.is_default {
                Some(parse_quote!{
                    let #ident: #ty = #expr.or_else(|e| match e {
                        ::qjack::Error::ColumnNotFound(_) => ::std::result::Result::Ok(Default::default()),
                        other_err => ::std::result::Result::Error(other_err),
                    })?;
                })
            } else {
                Some(parse_quote!{ let #ident: #ty = #expr?; })
            }
        })
        .collect();

    let (impl_generics, _, where_clause) = generics.split_for_impl();
    let names = fields.iter().map(|field| &field.ident);

    Ok(quote!{
        #[automatically_derived]
        impl #impl_generics ::qjack::__private__::FromRow<#lifetime, R> for #ident #type_generics #where_clause {
            fn from_row(row: &#lifetime R) -> ::std::result::Result<Self, ::qjack::Error> {
                #( #reads )*
                ::std::result::Result::Ok(
                    #ident { #( #names ),* }
                )
            }
        }

        #[automatically_derived]
        impl #original_generics ::qjack::Model for #ident #type_generics {}
    })
}

fn derive_for_tuple_struct(input: ItemStruct) -> Result<TokenStream, Error> {
    let (ident, original_generics)   = (&input.ident, &input.generics);
    let (_, ty_generics, _) = original_generics.split_for_impl();
    let (lifetime, lifetime_is_provided) = original_generics
        .lifetimes().next()
        .map(|def| (def.lifetime.clone(), false))
        .unwrap_or_else(|| (Lifetime::new("'a", Span::call_site()), true));

    let mut generics = original_generics.clone();
    generics.params.insert(0, parse_quote!{ R: ::qjack::__private__::Row });
    if lifetime_is_provided {
        generics.params.insert(0, parse_quote!{ #lifetime });
    }

    let predicates = &mut generics.make_where_clause().predicates;
    predicates.push(parse_quote!{
        ::std::primitive::usize: ::qjack::__private__::ColumnIndex<R>
    });
    for field in &input.fields {
        let ty = &field.ty;
        predicates.push(parse_quote!{ #ty: ::qjack::__private__::Decode<#lifetime, <R as ::qjack::__private__::Row>::Database> });
        predicates.push(parse_quote!{ #ty: ::qjack::__private__::Type<<R as ::qjack::__private__::Row>::Database> });
    }

    let (impl_generics, _, where_clause) = generics.split_for_impl();
    let gets = input.fields.iter()
        .enumerate()
        .map(|(i, _)| quote!{ row.try_get(#i)? });

    Ok(quote!{
        #[automatically_derived]
        impl #impl_generics ::qjack::__private__::FromRow<#lifetime, R> for #ident #ty_generics
        #where_clause
        {
            fn from_row(row: &#lifetime R) -> ::std::result::Result<Self, ::qjack::Error> {
                ::std::result::Result::Ok(
                    #ident( #( #gets ),* )
                )
            }
        }

        #[automatically_derived]
        impl #original_generics ::qjack::Model for #ident #ty_generics {}
    })
}
