fn main() {
    hello_world();
    tell_height(182);
    human_id("Justin", 37, 172.0);
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