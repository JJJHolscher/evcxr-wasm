# Evcxr

[![Binder](https://mybinder.org/badge_logo.svg)](https://mybinder.org/v2/gh/evcxr/evcxr/main?filepath=evcxr_jupyter%2Fsamples%2Fevcxr_jupyter_tour.ipynb)

A fork of the evaluation context for Rust.

This project consists of several related crates.

* [evcxr\_jupyter](evcxr_jupyter/README.md) - A Jupyter Kernel

* [evcxr\_repl](evcxr_repl/README.md) - A Rust REPL

* [evcxr](evcxr/README.md) - Common library shared by the above crates, may be useful for other
  purposes.

* [evcxr\_runtime](evcxr_runtime/README.md) - Functions and traits for interacting with Evcxr from
  libraries that users may use from Evcxr.
  
If you think you'd like a REPL, I'd definitely recommend checking out the Jupyter kernel. It's
pretty much a REPL experience, but in a web browser.

To see what it can do, it's probably best to start with a [tour of the Jupyter
kernel](evcxr_jupyter/samples/evcxr_jupyter_tour.ipynb). Github should allow you to preview this, or
you can load it from Jupyter Notebook and run it yourself.

## Wasm Cells

This fork includes the `:wasm` command, which is only useful in a Jupyter context.
The wasm command does the following:

1. It temporarily makes a new state for the cell that has no access to any pre-existing variables or dependencies.
2. It compiles the cell to wasm with `wasm-pack build --target web --out-dir ./evcxr_pkg/$(build_num)/`.
3. It injects some javascript glue into your browser window that loads the compiled wasm functions into `window.evcxr`.
4. It does not execute anything, nor does it remember any variables or dependencies for future cells.

Once that is done, you can call the wasm functions from javascript, which you can access through evcxr_display:

```rust
:wasm
:dep wasm-bindgen = "*"

#[wasm_bindgen]
fn add(x: i32, y: i32) -> i32 {
    return x + y
}
```

```rust
struct Add { 
    x: i32,
    y: i32
}

impl Add {
    fn evcxr_display(&self) {
        println!("EVCXR_BEGIN_CONTENT text/html\nwindow.evcxr.add({}, {})\nEVCXR_END_CONTENT", self.x, self.y);
    }
}

add = Add { x: 1, y: 1 }
add // this should show "2" in the output cell now
```

This also enables you to use front-end libraries like Dioxus or Yew to make widgets inside your notebooks, as long as you can pass any Rust types you want to visualize into a form compatible with javascript.

## Fork Q&A

> Do I need to write out my dependencies again for every new wasm cell?

Yes, sorry. It's probably possible to have wasm cells share dependencies and I might implement that at some point.

> What is that `evcxr_pkg` folder suddenly popping up?

It's made whenever you run a wasm cell. You can safely discard it, though if you to transform your notebook into a web page, like with quarto, you will want to keep that folder as it contains the compiled wasm and necessary javascript glue.

> Can my non-wasm rust cells access any variables from wasm cells?

No, only the wasm functions that end up exposed to the browser's javascript can be called from evcxr_display, the rest is inaccessible.

> Why?

I want to convert my notebooks to webpages that can run code on the client's device without me needing an online jupyter server. The jupyter server can run arbitrary code after all.

> Project status?

It probably won't be merged into main and no one is paying me to do this, so I'll maintain it as I use it. If you see there haven't been commits here for a while then I might have stopped using this.

## License

This software is distributed under the terms of both the MIT license and the Apache License (Version
2.0).

See LICENSE for details.
