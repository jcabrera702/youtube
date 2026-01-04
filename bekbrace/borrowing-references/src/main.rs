//Mutabale Reference
// Create Reference by add "&"
// -I- Immutable Reference
fn main() {
    
    // let x become mutable that way we reference and x will not be dropped
    // & sign added to type and variable x
    
    //made x mutable
    //changes in line 11 mut, line 12 &mut, &mut 
    // Notice how x value has changed when cargo run
    //Only 1 mutable references and many immutable references can be done
    let mut x: i32 = 5;
    let r: &mut i32 = &mut x;
    *r +=1;
    println!("X is equal to {}", x);
    
    
}
