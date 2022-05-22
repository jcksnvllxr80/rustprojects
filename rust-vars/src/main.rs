fn main() {
    // let x: i32;

    // assert_eq!(x, 5);
    // println!("success!");  // cant use x without initializing it

// ------------------------

    // let y: i32;
    // println!("success!");  // warn if use y without initializing it, still prints success

// ------------------------

    // let x: i32;
    // x = 1;
    // x += 2;
    // assert_eq!(x, 3);
    // println!("success!");  // cant assign to immutable vars (all vars are immutable by default in rust)

// ------------------------

    let mut x: i32;
    x = 1;
    x += 2;
    assert_eq!(x, 3);
    println!("success!");  // this works! ("mut" in rust means that value is not immutable)
}
