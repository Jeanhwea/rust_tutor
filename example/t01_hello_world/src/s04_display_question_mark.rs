use std::fmt; // Import the `fmt` module.

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Extract the value using tuple indexing,
        // and create a reference to `vec`.
        let vec = &self.0;

        // 注意 ? 符号, Try `write!`. If it errors, return if error
        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        for (i, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator to return on errors.
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
