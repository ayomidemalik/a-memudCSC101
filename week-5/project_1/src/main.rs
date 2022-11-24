//Rust program to find the roots of a quadratic equation
use std::io;
fn main() {
	
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();

	println!("Quadratic Equation/Formula : ax^2 + bx + c = 0");

	println!("Enter the value of the Coefficient of x^2, 'a'");
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let a:f32 = input1.trim().parse().expect("Not a valid number");

	println!("Enter the value of the Coefficient of x, 'b'");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let b:f32 = input2.trim().parse().expect("Not a valid number");

	println!("Enter the value of c");
	io::stdin().read_line(&mut input3).expect("Failed to read input");
	let c:f32 = input3.trim().parse().expect("Not a valid number");

	let s:f32 = (b * b) - (4.0 * a * c);
	
	let discriminant:f32 = s.sqrt();

	if discriminant > 0.0 {
		let root:f32 = (-b + discriminant)/ 2.0 * a;
		let root2:f32 = (-b - discriminant)/ 2.0 * a;
		println!("The roots of the quadratic equation are {} and {}.", root,root2);
	} else if discriminant == 0.0 { 
		let root1:f32 = (-b)/2.0 * a;
		println!("The root of the quadratic equation is {}",root1);
	} else {
		println!("There are no real roots of the quadratic equation");
	}



}