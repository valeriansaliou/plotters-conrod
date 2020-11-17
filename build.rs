// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

fn main() {
    build_poly2tri();
}

fn build_poly2tri() {
    cc::Build::new()
        .cpp(true)
        .flag("-Wno-c++11-extensions")
        .include("vendor/poly2tri")
        .file("src/triangulate/binding.cpp")
        .compile("libpoly2tri.a");
}
