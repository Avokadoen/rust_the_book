fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    for b in a.iter() {
        let b = b + 1;
        println!("b: {}", b);
    }

    for b in &a {
        let b = b + 1;
        println!("b: {}", b);
    }
    
    for b in &a {
        println!("b: {}", b);
    }

    another_function();
    another_function_param(a[0]);

    // statements don't return. you can use blocks to create expressions
    //let x = (let y = 6);

    // statement
    let x = 5;

    // statement
    let y = {
        // expression
        let x = 3;
        x + 1
    };

  println!("The value of y is: {}", y);

  println!("fn five: {}", five());
}


fn another_function() {
    println!("Another function.");
}

fn another_function_param(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

// valid implicit return as body is an expression
fn plus_one(x: i32) -> i32 {
    x + 1
}


/* invalid implicit return as semicolon turns it into a statement
fn plus_one(x: i32) -> i32 {
    x + 1;
}*/
