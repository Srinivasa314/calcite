extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{format_ident, quote};

/// A macro that takes the functions to be exported for use in Deno as arguments
/// # Syntax
/// ```
/// export!(function1,function2,...);
/// ```
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

/// When placed above a function it converts the function to a deno op of the same name.
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

    let fn_args = match function.next().unwrap() {
        TokenTree::Group(g) => g.stream(),
        _ => panic!(),
    };
    let fn_args: Vec<TokenTree> = fn_args.into_iter().collect();
    let mut passed_args = vec![];
    let mut arg_count = if isasync { 1usize } else { 0usize };

    for (i, token) in fn_args.iter().enumerate() {
        if let TokenTree::Punct(p) = token {
            if p.to_string() == "," && i != fn_args.len() - 1 {
                passed_args.push(quote! {
                    calcite::to_argument_type(&args[#arg_count])
                });
                arg_count += 1;
            }
        }
    }
    if !fn_args.is_empty() {
        passed_args.push(quote! {
            calcite::to_argument_type(&args[#arg_count])
        });
    }

    let fn_rest: Vec<TokenTree> = function.collect();
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
            let args = args.to_vec();
            let fut = async move {
                let command_id:usize = calcite::to_argument_type(&args[0]);
                let result = #__impl_fn_name(#(#passed_args),*).await;
                let result = calcite::AsyncResult {
                    command_id,
                    result
                };
                Into::<calcite::ReturnBuffer>::into(result).inner()
            };
            deno_core::plugin_api::Op::Async(fut.boxed())
        }
    } else {
        quote! {deno_core::plugin_api::Op::Sync(Into::<calcite::ReturnBuffer>::into(#__impl_fn_name(#(#passed_args),*)).inner() )}
    };

    (quote! {
        #pub_token fn #fn_name (_: &mut dyn deno_core::plugin_api::Interface, args: &mut [deno_core::plugin_api::ZeroCopyBuf]) -> deno_core::plugin_api::Op {
            #generated_fn_body
        }

        #async_token fn #__impl_fn_name ( #(#fn_args)* ) #(#fn_rest)*
    }).into()
}
