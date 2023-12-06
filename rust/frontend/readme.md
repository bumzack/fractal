# fucking nightmare

if youu see this when running ```trunk serve```

```
warning: `fractal-frontend` (bin "fractal-frontend") generated 4 warnings (run `cargo fix --bin "fractal-frontend"` to apply 3 suggestions)
Finished dev [unoptimized + debuginfo] target(s) in 0.04s
2023-07-15T21:23:48.313150Z  INFO fetching cargo artifacts
2023-07-15T21:23:48.386280Z  INFO processing WASM for fractal-frontend
2023-07-15T21:23:48.395368Z  INFO calling wasm-bindgen for fractal-frontend
2023-07-15T21:23:48.395931Z ERROR ❌ error
error from HTML pipeline

Caused by:
0: error from asset pipeline
1: error spawning wasm-bindgen call
2: Bad CPU type in executable (os error 86)
2023-07-15T21:25:37.274082Z  INFO 📦 starting build
2023-07-15T21:25:37.330984Z  INFO spawning asset pipelines
2023-07-15T21:25:37.744014Z  INFO building fractal-frontend
```

then try this

```
cargo install wasm-bindgen-cli
cargo install wasm-opt
```

taking from this issue

```
https://github.com/rustwasm/wasm-pack/issues/1098
```


