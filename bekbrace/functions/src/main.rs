fn main() {
    hello_world();
    tell_height(182);
}

//hoisting allows us to call functions from anywhere
fn hello_world(){
println!("Hello");
}

//You can insert input type values
fn tell_height(height: u32){
    println!("My height is {} cm.", height);
}
