//Mutabale Reference
// Create Reference by add "&"
// -I- Immutable Reference
fn main() {
    let x: i32 = 5;
    // let x become mutable that way we reference and x will not be dropped
    // & sign added to type and variable x
    let r: &i32 = &x;
    println!("X is equal to {}", x);
    println!("r is equal to {}", r);
    
}
