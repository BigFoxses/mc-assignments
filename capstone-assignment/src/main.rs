//! Main Module
//! 
//! `main` is to to process this raw data so that it can be
//! used stored in an analytics database and used productively by the Data Analyst team to 
//! forecast trades for the following financial year.
//! 



mod location;
mod utils;
use utils::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod transaction;
use location::Continent;
use transaction::Transaction;
use std::collections::HashMap;






/// 'Main' is to to process this raw data so that it can be 
///  used stored in an analytics database and used productively by the Data Analyst team to 
///  forecast trades for the following financial year.
///
/// * Read the transaction file
/// 
/// * skip the first line which is header
/// 
/// * parse line by line
/// 
/// * push the proper formatted transaction into transactions vector
/// 
/// * push the mal-formatted line into skipped_line
/// 
/// * print all well-formatted transactions 
/// 
/// * print the mal-formatted skipped_line
/// 
/// * print investment amount per Continent
/// 
/// * print Transactions with European Countries
/// 
fn main() {

    let file:File = File::open("./transactions.csv").unwrap();
    let reader  = BufReader::new(file);

    let mut transactions:Vec<Transaction> =Vec::new();
    let mut skipped_lines: Vec<_> = Vec::new();
  
    for (idx, line) in reader.lines().enumerate() {
        if idx ==0 {
            continue; // skip the header 
        }
        let line_str = line.unwrap();
        let parsed_transaction = Transaction::from_csv_line(&line_str);
        match parsed_transaction {
            Ok(transaction) => transactions.push(transaction), 
            Err(error_string) => skipped_lines.push((idx,error_string,line_str)),
        }

    }
    
    
//
// print all well-formatted transactions 
//  

    for transaction in transactions.iter() {
        println!("{:?}", transaction);
    }
    
// 
// print the mal-formatted skipped_line
// 
    for skipped_line in skipped_lines.iter() {
       println!("{:?}", skipped_line); 
    }

//
// Collect investment per Continent
// 
    let mut investment_per_continent:HashMap<String,f64>= HashMap::new();

    for transaction in transactions.iter(){
        if let Some(c) = transaction.get_continent().convert_to_string() {
            let continent=c.to_owned();
            let mut curr_amount = transaction.get_amount();
            if investment_per_continent.contains_key(&continent) {  // if key exist, add amount first
                curr_amount += investment_per_continent.get(&continent).unwrap();
                
            }
            investment_per_continent.insert(c.to_owned(), curr_amount); // if the key does not exist, a key/value is inserted or else the value is updated

            }
            
        } 

        println!("***************************");
        println!("investment per Continent is {:?}", investment_per_continent);

 //
 // Transactions with European Countries
 // 
        let continent = Continent::Europe;
        //let txn:Vec<Transaction> = transactions.into_iter().filter(|x| *x.get_continent() == c).collect();
        let txnlist= transactions_per_continent(&transactions, &continent);

        println!("***************************");
        println!("{:?} transactions list ",continent );
        println!("Transactions list is {:?}", txnlist);


        //println!("investment spec Continent is {:?}", txn);
        
    }
    



