use std::io;
fn main() 
{

let mut input1 = String::new();
println!("How old are you? ");
io::stdin().read_line(&mut input1).expect("Not a valid string");
let a:f32 = input1.trim().parse().expect("Not a valid number");

let mut input2 = String::new();
println!("How many years of work experience do you have?");
io::stdin().read_line(&mut input2).expect("Not a valid string");
let b:f32 = input2.trim().parse().expect("Not a valid number");

if a == 0.0 || b == 0.0
{
    println!("Error: cannot determine incentive");
    return;
}
else
{
    if a >= 40.0 && b >= 8.0
    {
        println!("Your incentive is N1,560,000.00");
    }
    else if a >= 30.0 && a < 40.0 && b >= 8.0
    {
        println!("Your incentive is N1,480,000.00");
    }  
    else if a < 28.0 && b >= 8.0
    {
        println!("Your incentive is N1,300,000.00");
    }
    else if b <= 8.0
    {
        println!("Your incentive is N100,000.00");
    }
}
}
