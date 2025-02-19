use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

/**
实现属性宏
*/
pub fn list_fix_fields(item: TokenStream) -> TokenStream {
    // 解析输入的结构体
    let input = parse_macro_input!(item as DeriveInput);

    // 获取结构体的名称
    let struct_name = &input.ident;
    eprintln!("结构体名称: {:?}", struct_name); // 打印输入的结构体信息
    // 检查是否为结构体
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("Only structs with named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

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
            pub fn fields_list(&self) -> Vec<serde_json::Value> {
                vec![
                    #(serde_json::Value::from(#field_names)),*
                ]
            }
        }
    };
    eprintln!("生成的代码: {:?}", TokenStream::from(expanded.clone()).to_string()); // 打印输入的结构体信息
    // 返回生成的代码
    TokenStream::from(expanded)
}

/**
实现属性宏
*/
pub fn ordered_fields(item: TokenStream) -> TokenStream {
    // 解析输入的结构体
    let input = parse_macro_input!(item as DeriveInput);

    // 获取结构体的名称
    let struct_name = &input.ident;
    eprintln!("结构体名称: {:?}", struct_name); // 打印输入的结构体信息
    // 检查是否为结构体
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("Only structs with named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

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
    eprintln!("生成的代码: {:?}", TokenStream::from(expanded.clone()).to_string()); // 打印输入的结构体信息
    // 返回生成的代码
    TokenStream::from(expanded)
}
