extern crate rand;
use std::io;
use std::cmp::Ordering;
use rand::{random_range};

fn main(){

    println!("Welcome To Our Game");

    //Creating a Secret Number

    let secret_number = random_range(1..=101);
    // println!("The Seceret Number is {}",secret_number);

    //Run A loop So The user Play Till he Not Win

    loop {
    

    //Declare The Variable    
    let mut guess = String::new();


    //Enter Input from user in String Format
    io::stdin().read_line(&mut guess)
        .expect("Enter Value is not valid");


    //Convert String into Number    
    let guess:u32 = match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue,
    };

    println!("You Guessed: {}",guess);    


    //Comparing Secret Number With Guessed Number
    match guess.cmp(&secret_number){
        Ordering::Less => println!("You Number is Less "),
        Ordering::Equal =>{
        println!("You Win");
        break;
        }
        Ordering::Greater => println!("You Number is Big "),
    }
    }
}