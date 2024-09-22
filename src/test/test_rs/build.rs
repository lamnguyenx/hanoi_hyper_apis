fn main() {
    // Compile the C code
    cc::Build::new()
        .file("c_src/program.c")
        .compile("libprogram.a");
}