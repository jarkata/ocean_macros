mod field_ordered;
mod sql_columns;

use crate::field_ordered::{list_fix_fields, ordered_fields};
use crate::sql_columns::make_all_columns;
use proc_macro::TokenStream;

/**
实现属性宏
*/
#[proc_macro_attribute]
pub fn to_ordered_vec(attr: TokenStream, item: TokenStream) -> TokenStream {
    eprintln!("属性列表:{}", attr.clone());
    ordered_fields(item)
}

#[proc_macro_attribute]
pub fn db_field_name(attr: TokenStream, item: TokenStream) -> TokenStream {
    eprintln!("属性列表:{}", attr.clone());
    item
}

#[proc_macro_derive(fields_list, attributes(db_field_name))]
pub fn to_fix_field_list(item: TokenStream) -> TokenStream {
    list_fix_fields(item)
}

#[proc_macro_attribute]
pub fn all_columns(_attr: TokenStream, item: TokenStream) -> TokenStream {
    make_all_columns(_attr, item)
}