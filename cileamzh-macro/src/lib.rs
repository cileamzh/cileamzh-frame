extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn show(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 解析输入的TokenStream为一个函数项
    println!("this is item{:#?}", item);
    println!("this is attr:{:#?}", attr);
    item
}

#[proc_macro_attribute]
pub fn post(_attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("{}", item);
    let tokens = quote! {
        ("post","/",fn(){println("hello")})
    };
    TokenStream::from(tokens)
}
