use git2::Repository;

fn main() {
    Repository::open("wren").unwrap_or_else(|_| {
        Repository::clone("https://github.com/wren-lang/wren", "wren").expect("failed to clone")
    });

    cc::Build::new()
        .warnings(false)
        .define("WREN_OPT_META", "0")
        .define("WREN_OPT_RANDOM", "0")
        .includes(&["wren/src/include", "wren/src/vm"])
        .files(&[
            "wren/src/vm/wren_compiler.c",
            "wren/src/vm/wren_core.c",
            "wren/src/vm/wren_debug.c",
            "wren/src/vm/wren_primitive.c",
            "wren/src/vm/wren_utils.c",
            "wren/src/vm/wren_value.c",
            "wren/src/vm/wren_vm.c",
        ])
        .compile("wren");
}
