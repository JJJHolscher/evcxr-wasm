pub use evcxr_ssg_macro::*;
pub use postcard;
pub use rand;
pub use serde;
use sha2::Digest;

pub fn glue_bytes_to_js(bytes: Vec<u8>) -> Result<String, String> {
    if bytes.len() > 100000 {
        // If the struct is large, we save it to the file system.
        let mut hasher = sha2::Sha256::new();
        hasher.update(&bytes);
        let path = std::path::PathBuf::from(format!("evcxr_pkg/{:x}.postcard", hasher.finalize()));

        if !path.exists() {
            std::fs::create_dir_all("evcxr_pkg")
                .map_err(|_| "could not make evcxr_pkg dir".to_owned())?;
            std::fs::write(&path, bytes).map_err(|_| "could not write to file".to_owned())?;
        }
        // Fetch the saved file in javascript
        return Ok(format!(
            "resp = await fetch(window.evcxr_cwd + '/{}');
            args.push(new Uint8Array(await resp.arrayBuffer()));",
            path.into_os_string().into_string().unwrap()
        ));
    }

    if let Ok(arg) = postcard::from_bytes::<String>(&bytes) {
        if arg.starts_with('$') {
            if arg == "$" {
                return Ok("args.push(root);".to_owned());
            }
            let mut chars = arg.chars();
            chars.next();
            return Ok(format!("args.push(root.{});", chars.as_str()));
        } else if arg.starts_with("./") | arg.starts_with("../") {
            return Ok(format!("args.push(window.evcxr_cwd + '/{}');", arg));
        }

        return Ok(format!("args.push('{}')", arg));
    }

    // If the struct is small, we inject it into the html.
    return Ok(format!("args.push(new Uint8Array({:?}));", bytes));
}

pub fn stylesheet(href: &str) {
    println!("
        EVCXR_BEGIN_CONTENT text/html\n
        <link id='evcxr_stylesheet' rel='stylesheet' href='{href}'>
        \nEVCXR_END_CONTENT
    ");
}
