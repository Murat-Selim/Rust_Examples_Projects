fn main() {
    let result = increment(5);
    println!("The increment of value: {}", result);
    
    let result2 = decrement(5);
    println!("The decrement of value: {}", result2);

    println!("------------------------------------");

    let approved_number = approved_number(9);
    
    if approved_number {
        println!("this is number approved: {}", approved_number);
    }
    else {
        println!("this is number disapproved: {}", approved_number);
    }
}

fn increment(mut x: i32) -> i32 {
    x = x+1;
    x
}

fn decrement(mut x: i32) -> i32 {
    x = x-1;
    x
}


fn approved_number(x: u32) -> bool {
    match x {
        10 => true,
        _  => false,
    }
}
