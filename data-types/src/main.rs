fn main() {
    // let x: f64 = 6.0; // explicit definition of float
    // println!("value of x is {}", x);

    // -----------------------

    // let x = 6.0; // infered definition of float
    // println!("value of x is {}", x);

    // -----------------------

    // let logical:bool = false; // bool
    // println!("value of logical is {}", logical);

    // -----------------------

    // let a:f64 = 6.0;
    // let b = 5i32;
    // println!("value of a is {}, b is {}", a, b);

    // -----------------------

    // let mut x = 12;
    // x = 6;
    // println!("value of x is {}", x); // gives warning that the original x was never read before reassignment

    // -----------------------

    // let mut x = 12;
    // x = false; // if we were to use 'let x = false;' here, then the code would work using 'shadowing'
    // println!("value of x is {}", x); // give an error... cannot change type of a mutable var

    // ----------------------

    // let x = 5.0_f64;
    // println!("success");

    // ----------------------

    // let x = 5.0_f64;
    // println!(x);  // does not work, needs string and curly braces

    // ----------------------

    // let x = 5.0_f64;
    // println!("{}", x);  // works

    // ---------------------

    let x:i32 = 5;
    let mut y:u32 = 5;
    println!("{}", y);
    y = x; // will not allow assigning a u32 into a i32 // wont build
    println!("{}", y); 
    println!("success!")
}
