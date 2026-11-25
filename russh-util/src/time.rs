#[cfg(not(all(target_arch = "wasm32", not(target_os = "wasi"))))]
pub use std::time::Instant;

// `web-time` uses `Performance.now()` on wasm32-unknown-unknown (where
// `std::time::Instant::now()` panics). Unlike the previous `chrono::Utc::now()`
// based implementation, it is monotonic.
#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
pub use web_time::Instant;
