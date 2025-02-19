mod field_ordered;
mod sql_columns;

use crate::field_ordered::ordered_fields;
use crate::sql_columns::all_columns;
use proc_macro::TokenStream;

/**
实现属性宏
*/
#[proc_macro_attribute]
pub fn to_ordered_vec(_attr: TokenStream, item: TokenStream) -> TokenStream {
    ordered_fields(item)
}

#[proc_macro_attribute]
pub fn all_columns(_attr: TokenStream, item: TokenStream) -> TokenStream {
    all_columns(_attr, item)
}