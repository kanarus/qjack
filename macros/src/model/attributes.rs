use proc_macro2::{Span, TokenStream, Ident};
use syn::{Type, Attribute, Error};
use quote::quote;


#[derive(Clone)]
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
} impl TypeName {
    pub fn get(&self) -> TokenStream {
        let val = &self.val;
        quote!{ #val }
    }
}

pub fn parse_container_attributes(attrs: &[Attribute]) -> Result<ContainerAttributes, Error> {
    todo!(/* https://github.com/launchbadge/sqlx/blob/main/sqlx-macros-core/src/derives/attributes.rs#L69 */)
}


pub struct ChildAttributes {
    pub rename:     Option<String>,
    pub is_default: bool,
    pub is_flatten: bool,
    pub try_from:   Option<Type>,
    pub is_skip:    bool,
}

pub fn parse_child_attributes(attrs: &[Attribute]) -> Result<ChildAttributes, Error> {
    todo!(/* https://github.com/launchbadge/sqlx/blob/main/sqlx-macros-core/src/derives/attributes.rs#L154 */)
}
