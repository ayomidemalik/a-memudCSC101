//Rust program to find the roots of an equation
use std::io;
fn main() {
 
let mut roota:f32 = 0.0;
let mut rootb:f32 = 0.0;

//Inputs
let mut input1 = String::new();
println!("Enter a: ");
io::stdin().read_line(&mut input1).expect("Not a valid string");
let a:f32 = input1.trim().parse().expect("Not a valid number");

let mut input2 = String::new();
println!("Enter b: ");
io::stdin().read_line(&mut input2).expect("Not a valid string");
let b:f32 = input2.trim().parse().expect("Not a valid number");

let mut input3 = String::new();
println!("Enter c: ");
io::stdin().read_line(&mut input3).expect("Not a valid string");
let c:f32 = input3.trim().parse().expect("Not a valid number");

//Calculations
if a == 0.0 || b == 0.0 || c == 0.0
{
    println!("Error, Cannot determine roots");
    return;
}
else
{
   let mut num:f32 = b * b - 4.0 * a * c;
    if num < 0.0
    {
        println!("Imaginary Roots");
        let realrt:f32 = -b / (2.0 * a);
        num = num.abs();
        let imagert:f32 = num.sqrt() / (2.0 * a);
        println!("Root1 = {} +i {}",realrt,imagert);
        println!("Root2 = {} -i {}",realrt,imagert);
    }
    else if num > 0.0
    {
        println!("Roots are real and distinct");
        roota = (-b + num.sqrt()) / (2.0 * a);
        rootb = (-b - num.sqrt()) / (2.0 * a);
        println!("Root1 = {}", roota);
        println!("Root2 = {}", rootb);
    }
    else if num == 0.0
    {
        println!("Roots are real and equal");
        roota = -b / (2.0 * a);
        rootb = roota;
        println!("Root1 = {}",roota);
        println!("Root2 = {}",rootb);
    }
}
 

}
