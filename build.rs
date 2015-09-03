extern crate gcc;

fn main() {
    gcc::compile_library("libhello.a", &["src/hello.c"]);
}
