mod math_functions; 
use math_functions:: {add, subtract, multiply,divide}; 

use std:: io; 

fn input_numbers() -> (i32, i32) {

    let mut input = String::new(); 
    println!("Enter a number: ");
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let num1: i32 = input.trim().parse()
        .expect("Please enter a valid number"); 
    
    input.clear(); 

    println!("Enter other number: "); 
    io::stdin().read_line(&mut input)
        .expect("Failed to read line"); 
    let num2:i32 = input.trim().parse()
        .expect("Please enter a valid number"); 

    return (num1, num2); 
}

fn main() { 
    println!("Calculator v1.0.0"); 

    loop { 
        println!("1: add, 2: subtract, 3: multiply, 4: divide, 0: exit");
        println!("Type : "); 
        let mut input_num = String::new(); 
        io::stdin().read_line(&mut input_num)
            .expect("Failed to read line");
        let input_num: i32 = input_num.trim().parse()
            .expect("Please enter a valid number"); 
        
        match input_num {
            1=> {
                let a:i32;
                let b:i32; 
                let (a,b) = input_numbers(); 
                let result:i32 = add(a,b); 
                println!("Result : {}", result); 
            }, 
            2=> {
                let a:i32;
                let b:i32; 
                let (a,b) = input_numbers(); 
                let result:i32 = subtract(a,b); 
                println!("Result : {}", result); 
            }, 
            3=> {
                let a:i32;
                let b:i32; 
                let (a,b) = input_numbers(); 
                let result:i32 = multiply(a,b); 
                println!("Result : {}", result); 
            },
            4=> {
                let a:i32;
                let b:i32; 
                let (a,b) = input_numbers(); 
                let result:i32 = divide(a,b); 
                println!("Result : {}", result); 
            },
            0=>{break;},
            _=>{println!("Please choose the number from 0 to 4"); }
        }
                
    }

    println!("Bye Bye"); 

}
