extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{format_ident, quote};

#[proc_macro]
pub fn export(tokens: TokenStream) -> TokenStream {
    let tokens = proc_macro2::TokenStream::from(tokens)
        .into_iter()
        .filter(|token| match token {
            TokenTree::Ident(_) => true,
            _ => false,
        });
    (quote! {
        #[no_mangle]
        pub fn deno_plugin_init(interface: &mut dyn deno_core::plugin_api::Interface) {
            #(interface.register_op(stringify!(#tokens), #tokens);)*
        }
    })
    .into()
}

#[proc_macro_attribute]
pub fn deno_op(_attr: TokenStream, function: TokenStream) -> TokenStream {
    let mut function = proc_macro2::TokenStream::from(function).into_iter();
    let mut ispub = false;
    let mut isasync = false;
    loop {
        match function.next() {
            Some(TokenTree::Ident(ident)) => match &*ident.to_string() {
                "fn" => break,
                "pub" => ispub = true,
                "async" => isasync = true,
                _ => continue,
            },
            _ => continue,
        }
    }
    let fn_name = function.next().expect("function name missing");
    let fn_name = match fn_name {
        TokenTree::Ident(ident) => ident,
        _ => panic!(),
    };
    let __impl_fn_name = format_ident!("__impl_{}", fn_name);
    let fn_body: Vec<TokenTree> = function.collect();
    let pub_token = if ispub {
        quote! {pub}
    } else {
        quote! {}
    };
    let async_token = if isasync {
        quote! {async}
    } else {
        quote! {}
    };
    let generated_fn_body = if isasync {
        quote! {
            use calcite::futures::future::FutureExt;
            let args=args.to_vec();
            let fut = async move {
                #__impl_fn_name(&args).await
            };
            deno_core::plugin_api::Op::Async(fut.boxed())
        }
    } else {
        quote! {deno_core::plugin_api::Op::Sync(#__impl_fn_name(args))}
    };
    (quote! {
        #pub_token fn #fn_name (_: &mut dyn deno_core::plugin_api::Interface, args: &mut [deno_core::plugin_api::ZeroCopyBuf]) -> deno_core::plugin_api::Op {
            #generated_fn_body
        }

        #async_token fn #__impl_fn_name #(#fn_body)*
    }).into()
}
