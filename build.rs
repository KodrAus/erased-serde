/*!
Make sure that we have at least one of the `std` or `alloc` features.

It's possible to use `erased-serde` with just `alloc` on the `nightly` channel,
but not with `core` on its own.
*/

use std::env;

fn main() {
    let is_std = env::var("CARGO_FEATURE_STD").is_ok();
    let is_alloc = env::var("CARGO_FEATURE_ALLOC").is_ok();

    if !is_std && !is_alloc {
        let version = env::var("CARGO_PKG_VERSION").unwrap_or("*".to_owned());

        let mut s = String::new();

        s.push_str("\n`erased_serde` requires either the `std` or `alloc` features.\n");
        s.push_str("If you're compiling with `default-features = false`, then enable the `alloc` feature:\n\n");
        s.push_str("```\n");
        s.push_str("[dependencies]\n");
        s.push_str(&format!("erased-serde = {{ version = \"{}\", default-features = false, features = [\"alloc\"] }}\n", version));
        s.push_str("```\n");

        panic!(s);
    }
}