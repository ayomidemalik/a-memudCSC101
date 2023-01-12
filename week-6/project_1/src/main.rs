use std::io;

//Functions for the formulas
fn area_of_trapezium(a:f64, b:f64, h:f64) {
    let area = h/2.0*(a+b);
    println!("The area of the Trapezium = {}",area );

}

fn area_of_rhombus(a:f64, b:f64) {
    let area = (1.0/2.0)*a*b;
    println!("The area of the rhombus = {}",area );
}

fn area_of_parallelogram(a:f64, b:f64) {
    let area = b * a;
    println!("The area of the parallelogram = {}",area );
}

fn area_of_cube(a:f64) {
    let area = 6.0 * (a*a);
    println!("The area of the cube = {}",area );
}

fn volume_of_cylinder(a:f64, h:f64) {
    let volume = (22.0/7.0) * (a*a)*h;
    println!("The volume of cylinder = {}",volume );
}



fn main()
{
     let mut input1 = String::new();
     let mut input2 = String::new();
     let mut input3 = String::new();


    let arr1:[&str;5] = ["Trapezium", "rhombus", "parallelogram", "cube", "cylinder"];
    println!("\nformulas are :{:?}", arr1);

    for index in 0..5 {
        println!("Formula number {} solves area/volume for :{}",index, arr1[index]);
    }
    let mut input4 = String::new();
    println!("Select a number for the formula you want to solve");
    io::stdin().read_line(&mut input4).expect("Not a valid input");
    let i:i32 = input4.trim().parse().expect("Not a valid number");

    if i == 0
    {
        println!("Enter your parameter for a: ");
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let a:f64 = input1.trim().parse().expect("Not a valid number");

        println!("Enter your parameter for b: ");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let b:f64 = input2.trim().parse().expect("Invalid number");

        println!("Enter your value for h: ");
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let h:f64 = input3.trim().parse().expect("Invalid number");

        area_of_trapezium(a,b,h);
    }
    else if i == 1
    {
        println!("Enter your parameter for a: ");
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let a:f64 = input1.trim().parse().expect("Not a valid number");

        println!("Enter your parameter for b: ");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let b:f64 = input2.trim().parse().expect("Invalid number");

        area_of_rhombus(a,b);
    }
    else if i == 2
    {
        println!("Enter your parameter for a: ");
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let a:f64 = input1.trim().parse().expect("Not a valid number");

        println!("Enter your parameter for b: ");
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let b:f64 = input2.trim().parse().expect("Invalid number");

        area_of_parallelogram(a,b);
    }
    else if i == 3
    {
        println!("Enter your parameter for a: ");
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let a:f64 = input1.trim().parse().expect("Not a valid number");

        area_of_cube(a);
    }
    else if i ==4
    {
        println!("Enter your parameter for a: ");
        io::stdin().read_line(&mut input1).expect("Failed to read input");
        let a:f64 = input1.trim().parse().expect("Not a valid number");

        println!("Enter your value for h: ");
        io::stdin().read_line(&mut input3).expect("Failed to read input");
        let h:f64 = input3.trim().parse().expect("Invalid number");

        volume_of_cylinder(a,h);
    }
}

