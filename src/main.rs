#[allow(unused_variables)]
fn main() {
    println!("hello, world");

    println!("It's OK to use 🥰 Unicode¡");

    // Variables
    // Note the type inference for foo!
    let foo = 1;
    println!("foo is {}", foo);
    println!("foo is {foo}");

    // Types
    // https://google.github.io/comprehensive-rust/basic-syntax/scalar-types.html
    let i = 1;
    let j = 1u8;

    // Strings
    let string : &str = "Hi 🥰";
    let byte_string : &[u8] = b"ASCII only!"; // cannot contain 🥰

    // Arrays (fixed size; use Vector for auto-growing)
    let two_fixed_ints = [42; 10];

    // Mutable Variables
    // (All variables above are implicitly immutable, like C++ "const" or Java "final".)
    let mut two_ints =  [43; 17];
    two_ints[1] = 9;

    // Tuples
    let t: (i8, bool, usize) = (7, true, 9);
    println!("{:?}", t);
    println!("Tuple #1st element is {}", t.0);
}
