use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Result};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Expr, Ident, Token, Type, Visibility};


struct WasmFnCall {

}


pub fn serialize_argument<T>(arg: &T) -> Result<String>
where T: Serialize + Sized
{
    let bytes = postcard::to_stdvec(arg)?;
    // If the struct is small, we inject it into the html.
    if bytes.len() < 100000 {
        return Ok(format!(
            "args.push(new Uint8Array({:?}));",
            bytes
        ));
    }

    // If the struct is large, we save it to the file system.
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    let path = PathBuf::from(format!(
        "evcxr_pkg/{:x}.postcard",
        hasher.finalize()
    ));
            
    if !path.exists() {
        std::fs::create_dir_all("evcxr_pkg")?;
        std::fs::write(&path, bytes)?;
    }
    // Fetch the saved file in javascript
    Ok(format!(
        "resp = await fetch(window.evcxr_cwd + '{}');
        args.push(new Uint8Array(await resp.arrayBuffer()));",
        path.into_os_string().into_string().unwrap()
    ))
}
