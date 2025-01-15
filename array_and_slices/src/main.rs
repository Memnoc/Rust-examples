use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    println!("== Array and Slices ==");
    // Fixed-size array (type signature is superfluous).
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // All elements can be initialised to the same value
    let ys: [i32; 500] = [0; 500];

    // Indexing starts at 0
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);


    // `len` return the counts of elements in the array
    println!("Number of elements in the array: {}", xs.len())

    // Arrays are stock allocated
    println!("Array occupies {} bytes", mem::size_of_val(&xs))

    // Array can be automatically borrowed as slices
    println!("Borrow the whole array as a slice.")
    analyze_slice(&xs);
}
