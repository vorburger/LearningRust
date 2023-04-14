mod library;
// use library::print_books;

mod health_statistics;
mod health_statistics_pub;

mod geometry;

#[allow(unused_variables)]
fn main() {
    println!("hello, world");

    println!("It's OK to use 🥰 Unicode¡");

    // Variables
    // Note the type inference for foo!
    let first_var = 1;
    println!("first_var is {}", first_var);
    println!("first_var is {first_var}");

    // Types
    // https://google.github.io/comprehensive-rust/basic-syntax/scalar-types.html
    let i = 1;
    let j = 1u8;

    // Strings
    let string: &str = "Hi 🥰";
    let byte_string: &[u8] = b"ASCII only!"; // cannot contain 🥰

    // Arrays (fixed size; use Vector for auto-growing)
    let three_fixed_ints = [42; 3];
    // NOK: println!("{}", three_fixed_ints);
    println!(":? {three_fixed_ints:?}");
    println!(":#? {three_fixed_ints:#?}");

    // Array Type includes size!
    let three_moar_fixed_ints: [i32; 3] = three_fixed_ints;
    // NOK! let four_fixed_ints: [i32; 4] = three_fixed_ints;

    // Mutable Variables
    // (All variables above are implicitly immutable, like C++ "const" or Java "final".)
    let mut two_ints = [43; 17];
    two_ints[1] = 9;

    // Tuples
    let t: (i8, bool, usize) = (7, true, 9);
    println!("t: {t:?}");
    println!("Tuple #1st element is {}", t.0);

    // Structs, incl. methods on structs (OOP?)
    // see https://google.github.io/comprehensive-rust/basic-syntax/methods.html
    #[derive(Copy, Clone, Debug)]
    struct Point(i32, i32);
    let p1 = Point(3, 4);
    println!("p1: {p1:?}"); // :? requires Debug
    let p2 = p1; // This requires Copy
    let x = p2.0;

    // Call function
    println!("{}", greet("world"));

    // https://google.github.io/comprehensive-rust/exercises/day-1/implicit-conversions.html
    let x: i8 = 15;
    let y: i16 = 1000;
    let m = multiply(x.into(), y); // into() for i8 to i16 type conversion
    println!("{x} * {y} = {m}");

    // String to Integer Type Conversion with cool fancy pattern matching!
    let s = "3";
    if let Ok(i) = s.parse() {
        println!("{x} * {s} = {}", multiply(x.into(), i)); // into() for i8 to i16 type conversion
    }
    println!("{}", multiply_number_and_string(15, "3").unwrap());

    // library::print_books(); or, with the use library::print_books; on top, just:
    // TODO print_books();

    health_statistics::main();
    health_statistics_pub::main();
}

// Everything is private by default, `pub` makes it public.
// The following is Rustdoc; note the triple slash:
/// Return "Hello " + name argument.
// Examples in Rustdoc are run as tests - that's cool!
pub fn greet(name: &str) -> String {
    format!("Hello {name}")
}

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

fn multiply_number_and_string(x: i16, s: &str) -> Result<i16, std::num::ParseIntError> {
    // Check this out... IFF s can be parsed to an i16, then y will have the value;
    // BUT if parse() fails then this '?' will implicitly return from this function!
    // This saves a shitload of Go-like if err return crap non-sense; I 💝 this!
    let y: i16 = s.parse()?;
    Ok(x * y)
}
