mod field_ordered;
mod sql_columns;

use crate::field_ordered::ordered_fields;
use proc_macro::TokenStream;

/**
实现属性宏
*/
#[proc_macro_attribute]
pub fn to_ordered_vec(_attr: TokenStream, item: TokenStream) -> TokenStream {
    ordered_fields(item)
}
