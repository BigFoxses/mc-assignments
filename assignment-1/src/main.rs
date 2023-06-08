/// Day1 - assignment#1 
///1. Define a User struct which contains 2 fields:
///a. name (string e.g "John")
///b. balance (tuple e.g (100.00, "SGD"))
///2. Define a User method (using impl) called print_user_detail, which simply prints the name,
///balance and currency of the user.
///3. Define an accrue_interest function, which takes in a user and interest percentage as 2 separate
///parameters. Within the function, increase the users' balance by the interest percentage, and print
///out the user details by calling the print_user_detail method.
///4. In the main function, create a user variable of type User, populating the field values of name, and
///balance and currency. Then, call the accrue_interest function.
///5. Bonus: After the call to accrue_interest, call it multiple times so that the user may benefit from
///compounding interest.

struct User {
    
    name: String,
    balance: (f32,String),
    
    }
impl User {
    
    fn print_user_detail(&self){
        println!("username:{} balance:{} currency:{}", self.name, self.balance.0, self.balance.1);
   }
}

fn accrue_interest(user: &mut User, interest_rate: f32) {  

    user.balance.0 = user.balance.0 + (user.balance.0
         * interest_rate / 100.0);
    user.print_user_detail();
}
fn main() {
   
   let mut user = User{
      name: "Robert".to_owned(),
      balance: (100.0, "SGD".to_owned()),
   };
    
    accrue_interest(&mut user, 1.0);
    accrue_interest(&mut user, 1.0);
    accrue_interest(&mut user, 1.0);

}