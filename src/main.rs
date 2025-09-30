use rand::Rng;

fn main() {

    const _MUST_UPPERCASE:i8 = 1;
    let _a = 5;
    let _a = 'a';
    let _a:i8 = 9;
    let a:&u8 = &mut 0u8;
    println!("{a}");
    let mut _a = "aa";
    let mut _a:i8 = 9;
    let mut _a:&u8 = &0u8;
    let _tuple = ();
    let _tuple: ();
    let _tuple: () = ();
    let _tuple: (i32, f64, char) = (5, 15., 'c');
    let tuple : &(i32, f64, char) = & (5i32, 15., 'c');
    println!("{tuple:?}");
    let tuple : &(i32, f64, char) = &mut (5i32, 15., 'c');
    println!("{tuple:?}");
    let tuple : &mut (i32, f64, char) = & mut (5i32, 15., 'c');
    println!("{tuple:?}");
    let _tuple = (1, 15., 'c');
    let (_x, _u, _z) = _tuple;
    let (mut _x, _u, mut _z) = _tuple;
    let _fifteen_not_allow_camel_case:f32 = _tuple.1;
    let _array_must_be_same_type = [1., 2.];
    /*let _array = []; error*/
    let _empty_array:[i8;0];
    let empty_array:[i8;00] = [];
    /*println!("{empty_array}"); error*/
    println!("{empty_array:?}");
    let mut _empty_array:[i8;00] = [];
    let _array:[u8;3];
    let _array:[u8;3] = [0, 1, 2];
    let _array:&[u8;3] = &[0, 1, 2u8];
    let _zer0:&u8 = &_array[0];
    let secret_number = rand::rng().random_range(1..=100);

    println!("{_z} input your guess:");
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
