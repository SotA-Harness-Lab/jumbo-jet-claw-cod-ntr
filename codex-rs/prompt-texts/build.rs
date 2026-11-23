// SPDX-FileCopyrightText: 2026 Tianyu (Sky) Lu
// SPDX-License-Identifier: Apache-2.0

//! Writes the `text!` macro with the absolute place of the repository's `prompts/` directory,
//! so that every crate that uses it reads the same files, whatever its own place.

use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let prompts = Path::new(&manifest_dir).join("../../prompts");
    let prompts = prompts
        .canonicalize()
        .map_err(|err| format!("{}: {err}", prompts.display()))?;
    let prefix = format!("{}/", prompts.display());
    let out_dir = std::env::var("OUT_DIR")?;
    let macro_source = format!(
        "/// The text of `prompts/<path>`, read when the program is built: an expression of the\n\
         /// same kind as a string literal, so it serves as a `const`, in `concat!`, and as the\n\
         /// format string of `format!` and its kin (whose arguments are then named or given in\n\
         /// order: a format string from a macro cannot capture variables).\n\
         #[macro_export]\n\
         macro_rules! text {{\n    ($path:literal) => {{\n        \
         ::core::include_str!(::core::concat!({prefix:?}, $path))\n    }};\n}}\n"
    );
    std::fs::write(Path::new(&out_dir).join("text_macro.rs"), macro_source)?;
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
