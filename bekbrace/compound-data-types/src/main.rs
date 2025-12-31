fn main() {
    let numbers: [i32; 4]= [1,2,3,4];
    println!("Array of numbers {:?}", numbers);
    // let mix = [1, 2, "apple", true];
    // println!("Mixed Array {:?}", mix);
    let fruits: [&str; 3] = ["apple", "banana", "orange"];
    println!("Fruits Array: {}", fruits[0]);
    println!("Fruits Array: {}", fruits[1]);
    println!("Fruits Array: {}", fruits[2]);

    //SLices; stored on the stack
    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", number_slices);

     let animal_slices:&[&str; 4] = &["Tiger", "Lion", "Bear", "Rat"];
    println!("Animal Slice: {:?}", animal_slices);

    let book_slice:&[&String] = &[&"IT".to_string(), &"Harry Potter".to_string(), &"Zen".to_string()];
    println!("Book Slice: {:?}", book_slice);


    //Strings vs String Slices (&str)
    //Strings [growable, mutable, owned string type]
    //String Allocated on the heap
    // Declare String with String::from() or .to_string()
    // let stone_cold: String = String::from("Hell,"); make mut in order to push
    let mut stone_cold: String = String::from("Hell,");
    stone_cold.push_str("Yeah");
    println!("Stone cold says: {}", stone_cold);  

    let string: String = String::from("Hello, World!");
    // let slice: &str = &string; //This would return the whole message
    let slice: &str = &string[0..=4];
    println!("Slice Value: {}", slice);  
    
}
