//bmi calculator
use std::io;
fn main() {
    println!("Enter your weight in kg");
    let mut weight = String::new();
    io::stdin().read_line(&mut weight).expect("Failed to read line");
    let weight: f32 = weight.trim().parse().expect("Please type a number!");
    println!("Enter your height in feet");
    let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed to read line");
    let height: f32 = height.trim().parse().expect("Please type a number!");
    let a = height*0.3048;
    let bmi = weight / (a * a);
    println!("Your BMI is {}", bmi);
    if bmi < 18.5 {
        println!("You are underweight");
    } else if bmi >= 18.5 && bmi < 25.0 {
        println!("You are normal");
    } else if bmi >= 25.0 && bmi < 30.0 {
        println!("You are overweight");
    } else {
        println!("You are obese");
    }
}





// //converting meter to feet
// use std::io;
// fn main()
// {
//     println!("Enter your height in meter");
//     let mut height = String::new();
//     io::stdin().read_line(&mut height).expect("Failed to read line");
//     let height: f32 = height.trim().parse().expect("Please type a number!");
//     let a = height*3.28084;
//     println!("Your height in feet is {}", a);
// }