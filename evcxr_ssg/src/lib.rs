use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use std::path::PathBuf;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{parse_macro_input, Expr, Token, Type};

// pub extern crate rand;

struct Args {
    fn_name: Expr,
    elems: Vec<Expr>,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let punctuated: Punctuated<Expr, Comma> = Punctuated::parse_terminated(input)?;
        let mut punctuated = punctuated.into_iter();
        Ok(Self {
            fn_name: punctuated.next().ok_or(syn::Error::new(input.span(), "no arguments"))?,
            elems: punctuated.collect(),
        })
    }
}



#[proc_macro]
pub fn wasm_call(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(input as Args);
    let fn_name = args.fn_name;
    let elems = args.elems.iter();

    let expanded = quote! { (|| -> Result<(), String> {
        use sha2::Digest;
        use rand::distributions::DistString;
        use serde::Serialize;
        let serialize_argument = |arg| -> Result<String, String> {
            let bytes = postcard::to_stdvec(arg).map_err(|_| "postcard could not serialize".to_owned())?;

            if bytes.len() < 100 {
                let arg: String = postcard::from_bytes(&bytes).unwrap();
                if arg.starts_with("$") {
                    if arg == "$" {
                        return Ok("args.push(root);".to_owned())
                    } else if arg == "$cwd" {
                        return Ok("args.push(evcxr_cwd);".to_owned())
                    }
                    let mut chars = arg.chars();
                    chars.next();
                    return Ok(format!("args.push(root.{});", chars.as_str()))
                }
            }

            // If the struct is small, we inject it into the html.
            if bytes.len() < 100000 {
                return Ok(format!("args.push(new Uint8Array({:?}));", bytes));
            }

            // If the struct is large, we save it to the file system.
            let mut hasher = sha2::Sha256::new();
            hasher.update(&bytes);
            let path = std::path::PathBuf::from(format!("evcxr_pkg/{:x}.postcard", hasher.finalize()));

            if !path.exists() {
                std::fs::create_dir_all("evcxr_pkg").map_err(|_| "could not make evcxr_pkg dir".to_owned())?;
                std::fs::write(&path, bytes).map_err(|_| "could not write to file".to_owned())?;
            }
            // Fetch the saved file in javascript
            Ok(format!(
                "resp = await fetch(window.evcxr_cwd + '{}');
                args.push(new Uint8Array(await resp.arrayBuffer()));",
                path.into_os_string().into_string().unwrap()
            ))
        };

        let add_args = vec![ #( serialize_argument(#elems)?),* ];
        let id = format!(
            "{}_{}",
            #fn_name,
            rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 8)
        );

        println!(
            "EVCXR_BEGIN_CONTENT text/html\n
<div id='{id}'></div>
<script>
async function __evcxr_display_{id}() {{
args = [];
{add_args}
window.evcxr.{}(
document.getElementById('{id}'),
window.evcxr_cwd,
...args
);
}};
if (Object.keys(window.evcxr)) {{
__evcxr_display_{id}();
}} else {{
window.addEventListener('load', __evcxr_display_{id});
}}
</script>
\nEVCXR_END_CONTENT",
                    #fn_name
        );
    })()};

    proc_macro::TokenStream::from(expanded)
}


#[proc_macro]
pub fn wasm_call_test(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(input as Args);
    let elems = args.elems.iter();
    let fn_name = args.fn_name;

    let expanded = quote! { (|| -> Result<String, String> {
        use sha2::Digest;
        use rand::distributions::DistString;
        use serde::Serialize;
        let serialize_argument = |arg| -> Result<String, String> {
            let bytes = postcard::to_stdvec(arg).map_err(|_| "postcard could not serialize".to_owned())?;

            if bytes.len() < 100 {
                let arg: String = postcard::from_bytes(&bytes).unwrap();
                if arg.starts_with("$") {
                    if arg == "$" {
                        return Ok("args.push(root);".to_owned())
                    } else if arg == "$cwd" {
                        return Ok("args.push(evcxr_cwd);".to_owned())
                    }
                    let mut chars = arg.chars();
                    chars.next();
                    return Ok(format!("args.push(root.{});", chars.as_str()))
                }
            }

            // If the struct is small, we inject it into the html.
            if bytes.len() < 100000 {
                return Ok(format!("args.push(new Uint8Array({:?}));", bytes));
            }

            // If the struct is large, we save it to the file system.
            let mut hasher = sha2::Sha256::new();
            hasher.update(&bytes);
            let path = std::path::PathBuf::from(format!("evcxr_pkg/{:x}.postcard", hasher.finalize()));

            if !path.exists() {
                std::fs::create_dir_all("evcxr_pkg").map_err(|_| "could not make evcxr_pkg dir".to_owned())?;
                std::fs::write(&path, bytes).map_err(|_| "could not write to file".to_owned())?;
            }
            // Fetch the saved file in javascript
            Ok(format!(
                "resp = await fetch(window.evcxr_cwd + '{}');
                args.push(new Uint8Array(await resp.arrayBuffer()));",
                path.into_os_string().into_string().unwrap()
            ))
        };

        let add_args = vec![ #( serialize_argument(#elems)?),* ].join("\n");
        let id = format!(
            "{}_{}",
            #fn_name,
            rand::distributions::Alphanumeric.sample_string(&mut rand::thread_rng(), 8)
        );

        Ok(format!(
            "EVCXR_BEGIN_CONTENT text/html\n
<div id='{id}'></div>
<script>
async function __evcxr_display_{id}() {{
args = [];
{add_args}
window.evcxr.{}(
document.getElementById('{id}'),
window.evcxr_cwd,
...args
);
}};
if (Object.keys(window.evcxr)) {{
__evcxr_display_{id}();
}} else {{
window.addEventListener('load', __evcxr_display_{id});
}}
</script>
\nEVCXR_END_CONTENT", #fn_name
        ))
    })()};

    proc_macro::TokenStream::from(expanded)
}
