// 
// assingment 3
// 
struct UserAccount{
    name:String,
    age:Option<u32>,
}
trait Balance{
  fn get_balance(&self) -> u32{
    10
  }

}

impl Balance for UserAccount {
    
}

fn increase_balance<T: Balance>(balance: &T, increase_amount: u32) -> Result<u32, String> {
    if increase_amount <= 10 {
        Ok(balance.get_balance() + increase_amount)
    }else{
        Err("Increase must be less than 10 !".to_owned())
    }
}

fn main() {
    // create user_account, and set his age as Option::None
    let user_account = UserAccount { name: "John".to_owned(), age:None};
    match increase_balance(&user_account, 11) {
        Ok(x) => println!("UserAccount balance increased to {}", x),
        Err(e) => println!("{}", e),
    }
    if let Some(age) = user_account.age {
        println!("UserAccount age is {}",age);
    }


}

