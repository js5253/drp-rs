fn main() {
    println!("cargo:rerun-if-changed=.env");

    if let Ok(iter) = dotenvy::from_filename_iter(".env") {
        for (key, value) in iter.flatten() {
            // pass env vars to main.rs at compile time
            println!("cargo:rustc-env={key}={value}");
        }
    }
}