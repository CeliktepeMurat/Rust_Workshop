use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    const THREE_OUR_IN_SECONDS: u32 = 3_000_000_000;
    println!("The value of THREE_OUR_IN_SECONDS is: {}", THREE_OUR_IN_SECONDS);

    let x = 2.5; // f64
    let y: f32 = 3.0; // f32

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    let t = true;
    let f: bool = false;    
    println!("The value of t is: {}", t);
    println!("The value of f is: {}", f);

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("The value of c is: {}", c);
    println!("The value of z is: {}", z);
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    let tup = (5,10,15);
    println!("The value of tup is: {}", tup.0);

    let a = [1,2,3,4,5];
    for x in a {
        println!("The value of x is: {}", x);    
    }

    let a = [3; 5];
    for x in a {
        println!("The value of x is: {}", x);    
    }

    let x = another_function(5);
    println!("The value of x is: {}", x);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    
    println!("Please input number: ");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Please type a number!");
    
    if index > 5 {
        println!("The value of index is greater than 5");
    } else {
        println!("The value of index is less than 5");
    }

    let mut counter = 0;
    
    let res = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The value of res is: {}", res);

    let mut count = 0;

    'counting_loop: loop {
        println!("count: {count}");
        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");
            if remaining == 9 { break; }
            if count == 2 {break 'counting_loop; }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count: {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
    
}

fn another_function(x: i32) -> i32 {
    x+1
}