//Mutabale Reference
// Create Reference by add "&"
// -I- Immutable Reference
fn main() {
    
    // let x become a reference and x will not be dropped
    // & sign added to type and variable x
    
    //made x mutable
    //changes in line 11 mut, line 12 &mut, &mut 
    // Notice how x value has changed when cargo run
    //Only 1 mutable references and many immutable references can be done
    let mut x: i32 = 5;
    let r: &mut i32 = &mut x;
    *r +=1;
    println!("X is equal to {}", x);

    let mut account: BankAccount = BankAccount {
        owner: "Alice".to_string(),
        balance: 100.00
    };
    //Immutable borrow to check the balance
    account.check_balance();
    //Mutable borrow to withdraw money
    account.withdraw(25.00);
    account.check_balance();
      
}
 //struct: A data structure that allows you to group
//multiple fields together under one name
    struct BankAccount {
        owner: String,
        balance: f64,
    }

    impl BankAccount {
        fn withdraw(&mut self, amount: f64){
            println!("Withdrawing {} from account owned by {}", amount, self.owner);
            self.balance -= amount
        }
        fn check_balance(&self) {
            println!("Account owned by {} has a balance of {}", self.owner, self.balance);

        }
    }