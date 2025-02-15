// Copyright 2021, pdfium-sys Developers
// Copyright 2022, pdfium-render Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

#[cfg(feature = "bindings")]
extern crate bindgen;

fn main() {
    #[cfg(feature = "bindings")]
    build_bindings();

    #[cfg(feature = "static")]
    statically_link_pdfium();
}

#[cfg(feature = "bindings")]
fn build_bindings() {
    // AJRC - 3/1/22 - modified from the pdfium-sys version to removing explicit linking to
    // a system-provided pdfium. We still want the bindings generated by rust-bindgen,
    // since they provide various constants that are useful, but we will load functions
    // dynamically at runtime using libloading.

    // AJRC - 13/1/22 - docs.rs runs cargo doc in a read-only sandbox, so we can't
    // generate bindings. Skip bindings generation entirely if the DOCS_RS environment
    // variable is set, as per https://docs.rs/about/builds#detecting-docsrs.

    // AJRC - 22/1/22 - expanded bindings generation to cover all Pdfium modules, not
    // just the viewing and rendering functionality defined in fpdfview.h.

    if std::env::var("DOCS_RS").is_err() {
        // The DOCS_RS environment variable is _not_ set.

        // Tell cargo to invalidate the built crate whenever the wrapper changes.
        println!("cargo:rerun-if-changed=include/rust-import-wrapper.h");

        let bindings = bindgen::Builder::default()
            // The input header we would like to generate bindings for.
            .header("include/rust-import-wrapper.h")
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            // Try to keep original C++ comments for docs.
            .clang_args(
                [
                    "-fretain-comments-from-system-headers",
                    "-fparse-all-comments",
                ]
                .iter(),
            )
            .generate_comments(true)
            .generate()
            .expect("Unable to generate bindings");

        // Write the bindings to src/bindgen.rs.

        let out_path = std::path::PathBuf::from("src");

        bindings
            .write_to_file(out_path.join("bindgen.rs"))
            .expect("Unable to write bindings");
    }
}

#[cfg(feature = "static")]
fn statically_link_pdfium() {
    if let Ok(path) = std::env::var("PDFIUM_STATIC_LIB_PATH") {
        // Instruct cargo to statically link the given library during the build.

        println!("cargo:rustc-link-lib=static=pdfium");
        println!("cargo:rustc-link-search=native={}", path);

        // Optionally instruct cargo to link to a C++ standard library during the build.
        // TODO: AJRC - 30/9/22 - for now, we dynamically link to the selected standard library,
        // but ultimately we want to use a link type of "static:-bundle" once the feature is stabilised
        // (currently it is available only in nightly builds of Rust).

        #[cfg(feature = "libstdc++")]
        println!("cargo:rustc-link-lib=dylib=stdc++");

        #[cfg(feature = "libc++")]
        println!("cargo:rustc-link-lib=dylib=c++");
    }
}
