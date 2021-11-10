
struct BankAccount {
    balance: i32,
    verified: bool
}

fn add (num_one: i32, num_two: i32) -> i32 {
return num_one + num_two;
}


fn main() {
    let mut total = add(10, 5);
    let mut free_shipping = false;


     if total > 50 {
         println!("You qualify for free shipping");
         free_shipping = true;
    }
    else if total > 20 {
        println!("If you add more items, you can qualify for free shipping");
    }
    else {
        println!("No free shipping");
    }

    total = match free_shipping {
        true => total + 0,
        false => total + 5,
    };

    match total  {
        1 => println!("1"),
        2 => println!("2"),
        _ => println!("No match found"),
    }

    println!("Total: {:?}", total);

    let items: [i32;6] = [1,2,3,4,5,9];
    println!("{:?}", items);

    let vector_items = vec![1,2,3,4,5,9];
    println!("{:?}", vector_items);

    let mut second_vector_items = Vec::new();
    second_vector_items.push(1);
    second_vector_items.push(2);
    second_vector_items.push(3);
    second_vector_items.push(4);
    second_vector_items.push(5);

    println!("{:?}", second_vector_items);

    let my_account = BankAccount {
        balance: 100,
        verified: true
    };

    println!("{:?}", my_account.balance);
    println!("{:?}", my_account.verified);

}
