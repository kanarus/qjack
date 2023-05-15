/* Thanks to
    https://github.com/launchbadge/sqlx/blob/main/sqlx-macros-core/src/derives/attributes.rs
*/

use heck::{ToSnakeCase, ToShoutySnakeCase, ToKebabCase, ToLowerCamelCase, ToUpperCamelCase};
use syn::{Type, Attribute, Error, Meta, NestedMeta, MetaNameValue, Lit, spanned::Spanned};
use proc_macro2::{Span, Ident};


macro_rules! fail {
    ($t:expr, $m:expr) => {
        return Err(syn::Error::new_spanned($t, $m))
    };
}

macro_rules! try_set {
    ($i:ident, $v:expr, $t:expr) => {
        match $i {
            None => $i = Some($v),
            Some(_) => fail!($t, "duplicate attribute"),
        }
    };
}


#[derive(Clone, Copy)]
pub enum RenameAll {
    LowerCase,
    SnakeCase,
    UpperCase,
    ScreamingSnakeCase,
    KebabCase,
    CamelCase,
    PascalCase,
}

pub fn rename_all(s: &str, pattern: RenameAll) -> String {
    match pattern {
        RenameAll::LowerCase          => s.to_lowercase(),
        RenameAll::SnakeCase          => s.to_snake_case(),
        RenameAll::UpperCase          => s.to_uppercase(),
        RenameAll::ScreamingSnakeCase => s.to_shouty_snake_case(),
        RenameAll::KebabCase          => s.to_kebab_case(),
        RenameAll::CamelCase          => s.to_lower_camel_case(),
        RenameAll::PascalCase         => s.to_upper_camel_case(),
    }
}


pub struct ContainerAttributes {
    pub is_transparent: bool,
    pub type_name:      Option<TypeName>,
    pub rename_all:     Option<RenameAll>,
    pub repr:           Option<Ident>,
} pub struct TypeName {
    pub val:  String,
    pub span: Span,
}
// impl TypeName {
//     pub fn get(&self) -> TokenStream {
//         let val = &self.val;
//         quote!{ #val }
//     }
// }

pub fn parse_container_attributes(attrs: &[Attribute]) -> Result<ContainerAttributes, Error> {
    let mut transparent = None;
    let mut repr = None;
    let mut type_name = None;
    let mut rename_all = None;

    for attr in attrs.iter().filter(|a| a.path.is_ident("qjack") || a.path.is_ident("repr")) {
        match attr.parse_meta().map_err(|e| Error::new_spanned(attr, e))? {
            Meta::List(list) if list.path.is_ident("qjack") => for value in list.nested.iter() {
                match value {
                    NestedMeta::Meta(meta) => match meta {
                        Meta::Path(p) if p.is_ident("transparent") => try_set!(
                            transparent,
                            true,
                            value
                        ),
                        Meta::NameValue(MetaNameValue{path, lit: Lit::Str(val), ..}) if path.is_ident("rename_all") => try_set!(
                            rename_all,
                            match &*val.value() {
                                "lowercase" => RenameAll::LowerCase,
                                "snake_case" => RenameAll::SnakeCase,
                                "UPPERCASE" => RenameAll::UpperCase,
                                "SCREAMING_SNAKE_CASE" => RenameAll::ScreamingSnakeCase,
                                "kebab-case" => RenameAll::KebabCase,
                                "camelCase" => RenameAll::CamelCase,
                                "PascalCase" => RenameAll::PascalCase,
                                _ => fail!(meta, "unexpected value for rename_all"),
                            },
                            value
                        ),
                        Meta::NameValue(MetaNameValue{path, lit: Lit::Str(val), ..}) if path.is_ident("ytpe_name") => try_set!(
                            type_name,
                            TypeName {val: val.value(), span: value.span()},
                            value
                        ),
                        u => fail!(u, "unexpected_attribute"),
                    },
                    u => fail!(u, "unexpected attribute"),
                }
            }
            Meta::List(list) if list.path.is_ident("repr") => {
                if list.nested.len() != 1 {
                    fail!(&list.nested, "expected one value")
                }
                match list.nested.first().unwrap() {
                    NestedMeta::Meta(Meta::Path(p)) if p.get_ident().is_some() => try_set!(
                        repr,
                        p.get_ident().unwrap().clone(),
                        list
                    ),
                    u => fail!(u, "unexpected value")
                }
            }
            _ => ()
        }
    }

    Ok(ContainerAttributes {
        is_transparent: transparent.unwrap_or(false),
        type_name,
        rename_all,
        repr
    })
}


pub struct ChildAttributes {
    pub rename:     Option<String>,
    pub is_default: bool,
    pub is_flatten: bool,
    pub try_from:   Option<Type>,
    pub is_skip:    bool,
}

pub fn parse_child_attributes(attrs: &[Attribute]) -> Result<ChildAttributes, Error> {
    let mut rename =     None;
    let mut is_default = false;
    let mut is_flatten = false;
    let mut try_from =   None;
    let mut is_skip =    false;

    for attr in attrs.iter().filter(|a| a.path.is_ident("qjack")) {
        let meta = attr.parse_meta().map_err(|e| Error::new_spanned(attr, e))?;
        if let Meta::List(list) = meta {
            for value in list.nested.iter() {
                match value {
                    NestedMeta::Meta(meta) => match meta {
                        Meta::NameValue(MetaNameValue{path, lit: Lit::Str(val), ..}) if path.is_ident("rename") => try_set!(
                            rename,
                            val.value(),
                            value
                        ),
                        Meta::NameValue(MetaNameValue{path, lit: Lit::Str(val), ..}) if path.is_ident("try_from") => try_set!(
                            try_from,
                            val.parse()?,
                            value
                        ),
                        Meta::Path(path) if path.is_ident("default") => is_default = true,
                        Meta::Path(path) if path.is_ident("flatten") => is_flatten = true,
                        Meta::Path(path) if path.is_ident("skip")    => is_skip    = true,
                        u => fail!(u, "unexpected attribute")
                    },
                    u => fail!(u, "unexpected attribute")
                }
            }
        }
    }

    Ok(ChildAttributes { rename, is_default, is_flatten, try_from, is_skip })
}
