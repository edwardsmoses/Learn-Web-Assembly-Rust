fn add (num_one: i32, num_two: i32) -> i32 {
return num_one + num_two;
}


fn main() {
    let total = add(10, 25);
     if total > 50 {
         println!("You qualify for free shipping");
    }
    else if total > 20 {
        println!("If you add more items, you can qualify for free shipping");
    }
    else {
        println!("No free shipping");
    }
    println!("{:?} is {0}", total);
}
