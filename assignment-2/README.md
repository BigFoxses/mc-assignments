// Day1 -Assignment 2


1. Define an Enum PaymentType with variants:
a. DigitalToken
b. Cash
2. Define a Seller struct which contains 3 fields:
a. payment_type (PaymentType)
b. price (f32)
c. balance (f32)
3. Define a Buyer struct which contains 3 fields:
a. name (String)
b. payment_type (PaymentType)
c. balance (f32)
4. Define a BuyerGroup struct that contains a vector of members (a vector of Buyer struct).

5. Implement method add_member on BuyerGroup which adds a Buyer into members vector
6. Implement method find_buyer on BuyerGroup that finds returns index of Buyer with matching
payment_type, otherwise return -1
7. Implement method buy on BuyerGroup which accepts a buyer index, reference to a seller, and
keeps transferring value of seller.price from buyer to seller, until the buyer's balance is
insufficient.
8. In the main function:
a. Create 2 buyers with names John, Sally, with payment_type of DigitalToken, Cash, and
balance of 100.00 and 100.00 respectively
b. Create an empty BuyerGroup
c. Add 2 buyers (John and Sally) into buyer_group sequentially
d. Create 1 seller with payment_type of Cash, price of 10, balance of 0
e. Call find_buyer method on the buyer group to get index of Sally
f. Call buy method on the buyer group passing the index of Sally, and the seller
