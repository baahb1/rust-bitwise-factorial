use::std::io;



fn main() {


    


    println!("Enter the first number");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
    .ok();
    let input_str: &str = &input[..]; 

    
    let x : u64 = input_str.trim().parse().unwrap();

    println!("enter the second number");

    //let x : u64 = 4;


    let mut input = String::new();

    io::stdin().read_line(&mut input)
    .ok();
    let input_str: &str = &input[..]; 

    
    let y : u64 = input_str.trim().parse().unwrap();


    let mut result :u64;

    result = x | y;



    println!("(x | y ) => {} ",result); 

    let factorial : u64 = factorial(y);
    println!("the factorial of the second number is : {factorial}");

    result = bitwise_multiplication(x, y);
    println!("bitwise multiplication : {result}");
}


fn factorial(x: u64) -> u64{
    let mut result :u64 = 1;
    let mut input = x;
    while input != 0 {
        result = result * input;
        input = input-1;
    }

    result

    
}



fn even(x: u64) -> u64{
    let result: u64 = x & 1;
    result

}

fn bitwise_multiplication(mut a: u64, mut b: u64) -> u64{
    let mut result :u64 = 0;

    while b > 1 {

        if even(b) == 1  {
            
            result = result + a; 

        }
        a = a << 1;
        b = b >> 1;
    }
    result = result + a;
    result
}