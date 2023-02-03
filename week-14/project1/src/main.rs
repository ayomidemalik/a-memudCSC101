use std::io;
use std::io::Read;

fn main(){
	
	println!("Welcome to Globacom!");
	println!("\nTo access the database or its contents, kindly follow the following procedures mentioned below!");
	println!("\nIf you are an administrator, enter 1!");
	println!("\nIf you are a project manager, enter 2!");
	println!("\nIf you are an employee, enter 3!");
	println!("\nIf you are a customer, enter 4!");
	println!("\nIf you are a vendor, enter 5!");
	let mut input1 = String::new();
	
	io::stdin().read_line(&mut input1).expect("Not a valid string");
	let thevalue:i32 = input1.trim().parse().expect("Not a valid number");
	
	if thevalue == 1{

		let mut file = std::fs::File::open("globacom_db.sql").unwrap();
		let mut contents = String::new();
		file.read_to_string(&mut contents).unwrap();
		print!("{}", contents);
	} 
	else if thevalue == 2{
		let mut file = std::fs::File::open("project_tb.sql").unwrap();
		let mut contents = String::new();
		file.read_to_string(&mut contents).unwrap();
		print!("{}", contents);
	}
	else if thevalue == 3{
		let mut file = std::fs::File::open("staff_tb.sql").unwrap();
		let mut contents = String::new();
		file.read_to_string(&mut contents).unwrap();
		print!("{}", contents);
	}
	else if thevalue == 4{
		let mut file = std::fs::File::open("customer_tb.sql").unwrap();
		let mut contents = String::new();
		file.read_to_string(&mut contents).unwrap();
		print!("{}", contents);
	}
	else if thevalue == 5{
		let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
		let mut contents = String::new();
		file.read_to_string(&mut contents).unwrap();
		print!("{}", contents);
	}
	else{
	println!("A non identified value was entered. Try again. Thank you!");
	}
}