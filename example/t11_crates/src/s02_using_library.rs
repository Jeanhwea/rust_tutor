// extern crate rary; // May be required for Rust 2015 edition or earlier

// Where library.rlib is the path to the compiled library, assumed that it's in
// the same directory here:

// rustc s02_using_library.rs --extern rary=library.rlib

fn main() {
    rary::public_function();

    // Error! `private_function` is private
    //rary::private_function();

    rary::indirect_access();
}
