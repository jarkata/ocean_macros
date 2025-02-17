use log::info;
use proc_macro::TokenStream;
use quote::quote;
use serde_json::json;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/**
实现属性宏
*/
#[proc_macro_attribute]
pub fn to_ordered_vec(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析输入的结构体
    let input = parse_macro_input!(item as DeriveInput);

    // 获取结构体的名称
    let struct_name = &input.ident;
    info!("结构体的名称:{}",struct_name);
    // 检查是否为结构体
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("Only structs with named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    info!("字段名称:{}",fields.len());
    // 提取字段名称
    let field_names: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            quote! { self.#field_name.clone() }
        })
        .collect();

    // 生成代码
    let expanded = quote! {
        // 原始结构体定义
        #input

        // 为结构体实现方法
        impl #struct_name {
            pub fn to_ordered_vec(&self) -> Vec<serde_json::Value> {
                vec![
                    #(serde_json::Value::from(#field_names)),*
                ]
            }
        }
    };
    info!("生成的代码:{}",expanded);
    // 返回生成的代码
    TokenStream::from(expanded)
}
