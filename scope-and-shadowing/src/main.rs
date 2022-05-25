fn main() {
    // let x: i32 = 10;
    // {  // codeblock with y defined inside
    //     let y : i32 = 15;
    //     println!("the value of x is {} and the value of y is {}", x, y);
    // }
    // println!("the value of x is {} and the value of y is {}", x, y);  // y undefined outside the code block; compile error

    // ------------------

    // let x: i32 = 10;
    // let y : i32 = 15;
    // {
    //     println!("the value of x is {} and the value of y is {}", x, y);
    // }
    // println!("the value of x is {} and the value of y is {}", x, y);  // y is undefined here; compile error

    // ------------------

    // define_x();  // calling funcrtions

    // ------------------

    // let x:i32 = 4;
    // let x = "some text";
    // println!("{} is the value of x.", x); 
    // pick the video back up at 9:10 
    // https://www.youtube.com/watch?v=ad57wnn25iQ&list=PL5dTjWUk_cPaPhW2SQ1OCNwu3h8D9dYHh&index=4&t=10s

    // -----------------

    // let x: i32 = 5;
    // {
    //     let x=12;
    //     assert_eq!(x,12);
    // }
    // assert_eq!(x,5);

    // let x= 42;
    // println!("{}", x)

    // -----------------

    let mut x: i32 =1;
    x =7;

    let x =4; // this shadows x and its defined immutable
    x +=3; //cant change value of x

    println!("success!")
}

// fn define_x() {
//     let x = "Hello";
//     println!("{}, world!", x);
// }
