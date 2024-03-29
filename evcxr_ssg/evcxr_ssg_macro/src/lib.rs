use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, Expr};


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
pub fn call_wasm(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(input as Args);
    let fn_name = args.fn_name;

    let expanded = if !args.elems.is_empty() {
        let elems = args.elems.iter();
        quote! { (|| {
            use evcxr_ssg::rand::distributions::DistString;
            use evcxr_ssg::serde::Serialize;

            let all_bytes = vec![#(
                evcxr_ssg::postcard::to_stdvec(#elems).map_err(|_| "postcard could not serialize".to_owned()).unwrap()
            ),*];

            let add_args: Vec<String> = all_bytes
                .into_iter()
                .map(|b| evcxr_ssg::glue_bytes_to_js(b).unwrap())
                .collect();
            let add_args = add_args.join("\n");

            let id = format!(
                "{}_{}",
                #fn_name,
                evcxr_ssg::rand::distributions::Alphanumeric.sample_string(&mut evcxr_ssg::rand::thread_rng(), 8)
            );

            println!(
                "EVCXR_BEGIN_CONTENT text/html\n
<div id='{id}'></div>
<script>
    async function __evcxr_display_{id}() {{
        root = document.getElementById('{id}');
        args = [];
        {add_args}
        window.evcxr.{}(...args);
    }};
    if (Object.keys(window.evcxr) == true) {{
        __evcxr_display_{id}();
    }} else {{
        window.addEventListener('load', __evcxr_display_{id});
    }}
</script>
\nEVCXR_END_CONTENT",
                        #fn_name
            );
        })()}
    } else {
        quote! { (|| {
            println!(
                "EVCXR_BEGIN_CONTENT text/html\n
<script>
if (Object.keys(window.evcxr) == true) {{
    window.evcxr.{}();
}} else {{
    window.addEventListener('load', window.evcxr.{});
}}
</script>
\nEVCXR_END_CONTENT",
                        #fn_name, #fn_name
            );
        })()}
    };

    proc_macro::TokenStream::from(expanded)
}
