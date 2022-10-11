fn main() {
    println!("Hello, world! to the Data Types in Rust");

    /* Example */
    let guess: u32 = "42".parse().expect("Not a number!"); // u32 is unsigned 32-bit integer, without it we get an error: expected `u32`, found `&str`
    println!("The value of guess is: {}", guess);

    /* Scalar Types */

    // Integer Types

    /*
        Length	Signed	Unsigned
        8-bit	i8	    u8
        16-bit	i16	    u16
        32-bit	i32	    u32
        64-bit	i64	    u64
        128-bit	i128	u128
        arch	isize	usize
    */

    // Integer Literals

    /*
        Number literals	Example
        Decimal	        98_222
        Hex	            0xff
        Octal	        0o77
        Binary	        0b1111_0000
        Byte (u8 only)	b'A'
    */

    let a: u8 = 255;
    let b: u8 = 0;
    let c: u8 = 0xff;
    let d: u8 = 0o77;
    let e: u32 = 98_222;
    let f: u8 = b'A';
    let g: u8 = 0b1111_0000;
    println!(
        "a: {}, b: {}, c: {}, d: {}, e: {}, f: {}, g: {}",
        a, b, c, d, e, f, g
    );

    //Floating Point Types

    /*
        Length	Signed	Unsigned
        32-bit	f32     // All are signed
        64-bit	f64
    */

    let x: f32 = 2.1; // f32
    let y: f64 = 3.015; // f64
    println!("x: {}, y: {}", x, y);

    // Numeric Operations

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    println!(
        "sum: {}, difference: {}, product: {}, quotient: {}, floored: {}, remainder: {}",
        sum, difference, product, quotient, floored, remainder
    );

    // The Boolean Type

    let t = true;

    let f: bool = false; // with explicit type annotation
    println!("t: {}, f: {}", t, f);

    // The Character Type

    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("c: {}, z: {}, heart_eyed_cat: {}", c, z, heart_eyed_cat);

    /* Compound Types */

    // Tuple Type

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!(
        "five_hundred: {}, six_point_four: {}, one: {}",
        five_hundred, six_point_four, one
    );

    // Array Type

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let jan = months[0];
    let feb = months[1];
    println!("The value of jan is: {}", jan);
    println!("The value of feb is: {}", feb);

    // Array Type with type annotation

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a[0] is: {}", a[0]);

    // Array Type with type annotation and default value

    let a = [3; 5];
    println!(
        "The value of a is: {} {} {} {} {}",
        a[0], a[1], a[2], a[3], a[4]
    );
}
