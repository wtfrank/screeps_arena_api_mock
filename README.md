This crate provides an x64-based mock implementation of the screeps arena api https://github.com/rustyscreeps/screeps-arena-game-api, allowing an instance of a screeps arena bot to be compiled against test tooling, simulators etc that run as x64 code instead of wasm. With some minor adjustments to Cargo.toml and possibly some minor adjustments to parts of the bot (e.g. web-sys console.log), you can choose to build your bot targeting wasm or x64.

An example use case is the screeps arena sim https://github.com/wtfrank/screeps_arena_sim which runs your bot against defined scenarios you control.

===

How to adapt the screeps arena starter kit to use this mock/sim framework.

```
diff --git a/Cargo.toml b/Cargo.toml
index d1c702d..fb6f5bc 100644
--- a/Cargo.toml
+++ b/Cargo.toml
@@ -11,12 +11,17 @@ crate-type = ["cdylib", "rlib"]
 js-sys = "0.3"
 log = "0.4"
 fern = "0.6"
-screeps-arena-game-api = { git = "https://github.com/rustyscreeps/screeps-arena-game-api.git" }
-# screeps-arena-game-api = "0.1"
-# screeps-arena-game-api = { path = "../screeps-arena-game-api" }
-wasm-bindgen = "0.2"
+
+[target.'cfg(target_arch = "wasm32")'.dependencies]
+screeps-arena-wasm = { package = "screeps-arena-game-api", path = "../screeps-arena-game-api" }
+wasm-bindgen-wasm = { package = "real-wasm-bindgen", path = "../arena_api_mock/real-wasm-bindgen" }
 web-sys = { version = "0.3", features = ["console"] }
 
+[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
+screeps-arena-mock = { package = "mock-screeps-arena", path = "../arena_api_mock/mock-screeps-arena" }
+wasm-bindgen-mock = { package = "wasm-bindgen", path = "../arena_api_mock/mock-wasm-bindgen" }
+
+
 # The `console_error_panic_hook` crate provides better debugging of panics by
 # logging them with `console.error`. This is great for development, but requires
 # all the `std::fmt` and `std::panicking` infrastructure, so isn't great for
@@ -41,6 +46,16 @@ wasm-opt = ["-O4", "-g"]
 [features]
 default = []
 
-season3-spawn_strike = ["screeps-arena-game-api/season3-spawn_strike"]
-season3-power_split = ["screeps-arena-game-api/season3-power_split"]
-season3-escort_run = ["screeps-arena-game-api/season3-escort_run"]
+season3-spawn_strike = [
+  "screeps-arena-wasm/season3-spawn_strike",
+  "screeps-arena-mock/season3-spawn_strike"
+]
+season3-power_split = [
+  "screeps-arena-wasm/season3-power_split",
+  "screeps-arena-mock/season3-power_split"
+]
+season3-escort_run = [
+  "screeps-arena-wasm/season3-escort_run",
+  "screeps-arena-mock/season3-escort_run"
+]
+
diff --git a/src/lib.rs b/src/lib.rs
index 73f03c6..f8466bc 100644
--- a/src/lib.rs
+++ b/src/lib.rs
@@ -1,3 +1,14 @@
+#[cfg(target_arch = "wasm32")]
+pub extern crate screeps_arena_wasm as screeps_arena;
+#[cfg(target_arch = "wasm32")]
+pub extern crate wasm_bindgen_wasm as wasm_bindgen;
+
+#[cfg(not(target_arch = "wasm32"))]
+pub extern crate screeps_arena_mock as screeps_arena;
+#[cfg(not(target_arch = "wasm32"))]
+pub extern crate wasm_bindgen_mock as wasm_bindgen;
+
+
 use log::*;
 use screeps_arena::{
     constants::{prototypes, Part},
diff --git a/src/logging.rs b/src/logging.rs
index 90319f3..e0d36e9 100644
--- a/src/logging.rs
+++ b/src/logging.rs
@@ -1,14 +1,23 @@
+#[cfg(target_arch = "wasm32")]
 use crate::wasm_bindgen;
+#[cfg(target_arch = "wasm32")]
 use std::panic::PanicHookInfo;
+#[cfg(target_arch = "wasm32")]
 use js_sys::JsString;
+#[cfg(target_arch = "wasm32")]
 use log::error;
 pub use log::LevelFilter::*;
+#[cfg(target_arch = "wasm32")]
 use std::fmt::Write;
+#[cfg(target_arch = "wasm32")]
 use std::panic;
+#[cfg(target_arch = "wasm32")]
 use web_sys::console;
 
+#[cfg(target_arch = "wasm32")]
 struct JsLog;
 
+#[cfg(target_arch = "wasm32")]
 impl log::Log for JsLog {
     fn enabled(&self, _: &log::Metadata<'_>) -> bool {
         true
@@ -19,6 +28,7 @@ impl log::Log for JsLog {
     fn flush(&self) {}
 }
 
+#[cfg(target_arch = "wasm32")]
 pub fn setup_logging(verbosity: log::LevelFilter) {
     fern::Dispatch::new()
         .level(verbosity)
@@ -36,6 +46,21 @@ pub fn setup_logging(verbosity: log::LevelFilter) {
     panic::set_hook(Box::new(panic_hook));
 }
 
+#[cfg(not(target_arch = "wasm32"))]
+pub fn setup_logging(default_verbosity: log::LevelFilter) {
+  let level = std::env::var("RUST_LOG")
+    .ok()
+    .and_then(|val| val.parse::<log::LevelFilter>().ok())
+    .unwrap_or(default_verbosity);
+
+  let _ = fern::Dispatch::new()
+    .level(level)
+    .format(|out, message, record| out.finish(format_args!("({}) {}: {}", record.level(), record.target(), message)))
+    .chain(std::io::stdout())
+    .apply();
+}
+
+#[cfg(target_arch = "wasm32")]
 fn panic_hook(info: &PanicHookInfo) {
     // import JS Error API to get backtrace info (backtraces don't work in wasm)
     // Node 8 does support this API: https://nodejs.org/docs/latest-v8.x/api/errors.html#errors_error_stack

```
