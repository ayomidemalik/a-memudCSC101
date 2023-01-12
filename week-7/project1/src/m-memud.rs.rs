use std::io;

fn main()
{
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();
	let mut input4 = String::new();
	let mut input5 = String::new();
	let mut input6 = String::new();
	let mut input7 = String::new();

	println!("Enter your name:{}",input1);
	io::stdin().read_line(&mut input1).expect("Not a valid string");
	println!("Enter your email:{}",input2);
	io::stdin().read_line(&mut input2).expect("Not a valid string");

	println!("Enter your department:{}",input3);
		io::stdin().read_line(&mut input3).expect("Not a valid string");
	println!("Enter your state of origin:{}",input4);
	io::stdin().read_line(&mut input4).expect("Not a valid string");

	println!("Are you a current class rep? If yes, enter one . If no, enter zero");
	io::stdin().read_line(&mut input3).expect("Not a valid string");
	let ccr:i32 = input5.trim().parse().expect("Not a valid string");

	println!("Are you in 100 level  If yes, enter one. If no, enter zero");
	io::stdin().read_line(&mut input6).expect("Not a valid String");
	let lvl:i32 = input6.trim().parse().expect("Not a valid String");

	println!("Is your CGPA greater than 4.0? If yes, enter one. If not, enter zero");
	io::stdin().read_line(&mut input7).expect("Not a valid string");
	let cgpa:i32 = input7.trim().parse().expect("Not a valid string");


	if ccr > 0  && lvl < 1 && cgpa > 0
	{
	
		println!("{}",input1);
		println!("{}",input2);
		println!("{}",input3);
		println!("{}",input4);
		println!("You can vote")
	}
	else
	{
		println!("Sorry you are not eligible to vote")
	}





}