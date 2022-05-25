fn main() {
    // let (x,y,z,w,k) = (3,4,5,6,7);
    // println!("the value of x is {}", x); // warnings because of unused values other than x

    // -----------------

    // let (x,y,z,w,k) = (3,4,5,6,7);
    // println!("the value of x is {}", x);
    // println!("the value of y is {}", y);
    // println!("the value of z is {}", z);
    // println!("the value of w is {}", w);
    // println!("the value of k is {}", k); // this works

    // ----------------

    // let (x,..) = (3,4,5,6,7); // store the first thing in x and then ignore the rest (destructuring)
    // println!("the value of x is {}", x); // this works

    // ----------------

    // let (..,y) = (3,4,5,6,7); // store the last item in y and then ignore the rest (destructuring)
    // println!("the value of y is {}", y); // this works

    // ----------------

    // let [..,z] = [3,4,5,6,7];
    // println!("the value of z is {}", z); // works with lists too

    // -----------------

    let (x,..) = (3,4,5,6,7);
    let (..,y) = (4,5,6,7,8,9);
    let [..,z] = [3,4,5,6,7];
    println!("the value of x is {}", x);
    println!("the value of y is {}", y);
    println!("the value of z is {}", z);

    assert_eq!([x,y,z], [3,9,7]);
    println!("success, assertion correct");
}
