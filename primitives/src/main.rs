/**  INFO:
* SCALAR TYPES
* Signed integers - all i8-16-32-64-128 and isize (pointer size)
* Unsigned integers - all u8-16-32-64-128 and usize (pointer size)
* Floating points
* char Unicode scalar values
* bool
* Unit type () always an empty tuple
*
*/

/**  INFO:
* COMPOUND TYPES
* Array like [1,2,3]
* Tuple like (1, true)
*
*/
fn main() {
    // Vaiables can be type annotated
    let logical: bool = true;

    let a: f64 = 1.0; // Regular annotation
    let an_integer = 5i32; // suffix annotation

    // Or a default will be used - use inlay hints
    let default_float = 3.0;
    let default_integer = 7;

    // A type can also be inferred from context
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;

    // A mutable variable's value can be changed
    let mut mutable = true; // Mutable 'i32'
    mutable = 21; // mismatched type: expected bool

    // Error! The type of variable can't be changed
    mutable = true;

    // Variables can be overwritten with shadowing
    let mutable = true; // this is fine

    /*  HEADER: Compound types - Array and Tuples */
    
    // Array signature consists of of Type T and lenght as [T; length]
    let my_array: [i32, 5] = [1,2,3,4,5];

    // Tuple is a collection of values of different types
    // and is constructed using parentheses ().
    let my_tuple = (5u32, 1u8, true, -5.04f32);

}
