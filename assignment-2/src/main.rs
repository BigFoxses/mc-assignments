/// Day1 -Assignment 2
///
///
///2023-05-14
///
///1. Define an Enum PaymentType with variants:
///a. DigitalToken
///b. Cash
///2. Define a Seller struct which contains 3 fields:
///a. payment_type (PaymentType)
///b. price (f32)
///c. balance (f32)
///3. Define a Buyer struct which contains 3 fields:
///a. name (String)
///b. payment_type (PaymentType)
///c. balance (f32)
///4. Define a BuyerGroup struct that contains a vector of members (a vector of Buyer struct).
///
///5. Implement method add_member on BuyerGroup which adds a Buyer into members vector
///6. Implement method find_buyer on BuyerGroup that finds returns index of Buyer with matching
///payment_type, otherwise return -1
///7. Implement method buy on BuyerGroup which accepts a buyer index, reference to a seller, and
///keeps transferring value of seller.price from buyer to seller, until the buyer's balance is
///insufficient.
///8. In the main function:
///a. Create 2 buyers with names John, Sally, with payment_type of DigitalToken, Cash, and
///balance of 100.00 and 100.00 respectively
///b. Create an empty BuyerGroup
///c. Add 2 buyers (John and Sally) into buyer_group sequentially
///d. Create 1 seller with payment_type of Cash, price of 10, balance of 0
///e. Call find_buyer method on the buyer group to get index of Sally
///f. Call buy method on the buyer group passing the index of Sally, and the seller
///


 #[derive(PartialEq, Debug,Clone, Copy)]
 enum PaymentType  {
 DigitalToken,
 Cash,
    
}




#[derive(Debug)]
struct Seller {
 payment_type: PaymentType,
 price: f32,
 balance:f32,    
}

#[derive(Debug)]
struct Buyer{
    name:String,
    payment_type:PaymentType,
    balance:f32,
}

#[derive(Debug)]
struct BuyerGroup{
    buyers: Vec<Buyer>,
}
impl BuyerGroup {
    fn add_member(&mut self, buyer: Buyer ){
        self.buyers.push(buyer);
    }
    fn find_buyer(&self, payment_type: &PaymentType) -> i32 {
        let len= self.buyers.len();
        let mut result = -1;
        for i in 0..len{
            if self.buyers[i].payment_type == *payment_type {
               result= i as i32;
            }
        }
        result
    }
    

    fn buy(&mut self, index: usize, seller: &mut Seller){

          if let  Some(_) = self.buyers.get(index){
            let mut buyer = &mut self.buyers[index];
            loop {
                if  buyer.balance >= seller.price {
                    seller.balance += seller.price;
                    buyer.balance -= seller.price;
                    println!("{} Balance {}. Seller Balance {}", buyer.name, buyer.balance, seller.balance);
                } else{
                    println!("{} Balance {} insuffient fund. Seller Balance {}", buyer.name, buyer.balance, seller.balance);
                    break;
                }
            }   
          }
        
    }
 }



fn main() {
    let john = Buyer {
        name: "John".to_owned(),
        payment_type: PaymentType::DigitalToken,
        balance:100.00,

    } ;
    let sally = Buyer {
        name: "Sally".to_owned(),
        payment_type: PaymentType::Cash,
        balance:100.00,
    };

    let mut buyer_group = BuyerGroup  { 
        buyers: Vec::new(),
    };
    buyer_group.add_member(john);
    buyer_group.add_member(sally);

    let mut seller = Seller {
        payment_type: PaymentType::Cash,
        price:10.0,
        balance:0.0,
    };

    println!("BuyerGroup before selling - {:?}\n", buyer_group); 
    println!("Seller before selling ={:?}\n", seller);


        let index = buyer_group.find_buyer(&seller.payment_type);
        if  index != -1 {
            let u_index= index as usize;
            buyer_group.buy(u_index, &mut seller);
        }
       
    println!("Seller after selling={:?}\n", seller);
    println!("BuyerGroup - after selling {:?}\n", buyer_group);


    
    


}
