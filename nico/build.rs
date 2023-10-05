fn main() {
    // Build the `spellfix` extension because it's usually not distributed with sqlite packages.
    cc::Build::new()
        .file("sqlite/ext/misc/spellfix.c")
        .flag_if_supported("-pipe")
        .pic(true)
        .shared_flag(true)
        .compile("spellfix");
    println!("cargo:rerun-if-changed=sqlite/ext/misc/spellfix.c");
}
