/* INFO:
* Integers, floats, characters, strings, booleans and the unit type ()
* can be expressed using literals.
*
* We need to tell the compilers of the literals we use.
*
*
*
*
* */
fn main() {
    println!("== Literals & Operators ==");

    // Integer addition
    println!("1 - 2 = {}", 1u32 + 2); // 3

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2); // 3
                                      // using the wrong type will make the operation overflow
                                      // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("True AND false is {}", true && false);
    println!("True OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 AND 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 2 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability
    println!("One million is written as {}", 1_000_000u32);
}
