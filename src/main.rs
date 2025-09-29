use rand::Rng;

fn main() {

    println!("input your guess:");
    let secret_number = rand::rng().random_range(1..=100);
    
    loop {
        let mut guess = String::new();
        /*
        std::io::stdin().read_line(&guess)
        specify that the parameter expects a mut borrow in the function declaration
        and then actually take a mutable borrow of the value you want to pass as argument when you call the function.
        You took an immutable borrow of a mutable variable, which won't work.
        */
        std::io::stdin()
            .read_line(&mut guess)
            .expect("failed to read_line();");
        println!("{guess}");

        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Dumb fuck !");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small !"),
            std::cmp::Ordering::Equal => {
                println!("You win !");
                break;
            }
            std::cmp::Ordering::Greater => println!("To big !")
        }
    }

}
