fn main() {
    hello_world();
    tell_height(182);
    human_id("Justin", 37, 172.0);
    //If declaring _X outside of main function, Rust does not like the let keyword; use const
    let _X: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty 
    };
    println!("Result is: {}", _X);
    //calling the function add in a variable and in itself
    let y: i32 = add(4, 6);
    println!("The value of Y is {}", y);
    println!("The value of function add() is: {}", add(3,2));
}

//hoisting allows us to call functions from anywhere
fn hello_world(){
println!("Hello");
}

//You can insert input type values
fn tell_height(height: u32){
    println!("My height is {} cm.", height);
}

//You can insert more than one value
fn human_id(name: &str, age: u32, height: f32){
    println!("Hello my name is {}, I am {} years old and my height is {} cm.", name, age, height);
}

//functions returning values
fn add(a: i32, b: i32) -> i32{
    a + b
}

//Expressions and Statements
//Expression: Anything that returns a value
//Statement: Anything that does not return a value
//Expression examples: 5, true & false, add(3,4), if/else condition, ({code})
