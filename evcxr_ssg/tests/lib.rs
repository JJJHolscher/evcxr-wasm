use evcxr_ssg::wasm_call_test;

#[test]
fn cwd() {
    let out = wasm_call_test!("$cwd");
    assert_eq!(out.unwrap(), "args.push(evcxr_cwd);".to_owned())
}

#[test]
fn root() {
    assert_eq!(
        wasm_call_test!("$").unwrap(),
        "args.push(root);".to_owned()
    )
}

#[test]
fn id() {
    assert_eq!(
        wasm_call_test!("$id").unwrap(),
        "args.push(root.id);".to_owned()
    )
}

#[test]
fn hi() {
    assert_eq!(
        wasm_call_test!("hi").unwrap(),
        "args.push(new Uint8Array([2, 104, 105]));".to_owned()
    )
}
