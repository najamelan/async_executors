# async_executors - CHANGELOG

## 0.1.0.alpha.2 - 2020-02-19

  - fix error in feature name.
  - clean up readme.

## 0.1.0.alpha.1 - 2020-02-19

This is an alpha release because:

  - tracing integration should be tested
  - examples for tracing/bindgen/spawn_handle/using futures executors
  - remote_handle currently vendored until: https://github.com/rust-lang/futures-rs/pull/2081 lands
  - tracing-futures currently patched until 0.2.3 to get the needed PR's
  - no cross framework support for spawn_blocking yet.
  - wasm-bindgen-cli does not compile on windows, breaking CI: https://github.com/rustwasm/wasm-bindgen/issues/2006



