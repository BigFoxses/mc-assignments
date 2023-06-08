//!  Transaction Module
//! 
//! `transaction` defines the structure of a business [Transaction] record, associated function 
//! and methods


use crate::location::{Country, Continent};
use chrono::NaiveDate;

#[derive(Debug)]
/// Transaction Struct - business transaction id, etc
pub struct Transaction {
    transaction_id: u32,
    client_id: u32, 
    asset_name: String,
    country: Country,
    continent: Continent,
    amount: f64,
    days_under_management: i64,
}

impl Transaction{


/// 
/// 
/// Return [Transaction] if OK or ErrorString If Error 
/// 
/// # Arguments
///  
/// * `line` - line to be parsed based on "," delimiter. every line should have 7 fields
/// 
    pub fn from_csv_line(line: &str) -> Result<Transaction, String>{
        let fields:Vec<&str> = line.split(',').collect();
        if fields.len() !=7 {
            return Err("incorrect number of column".to_owned());
        } 
        // building the transaction struct
        let transaction_id = fields[0].parse::<u32>().unwrap(); // try to use get ??
        let client_id = fields[1].parse::<u32>().unwrap(); // try to use get ??
        let asset_name =fields[2].to_uppercase();
        let transaction_start_date= NaiveDate::parse_from_str(fields[3],"%Y-%m-%d").unwrap();
        let transaction_end_date= NaiveDate::parse_from_str(fields[4],"%Y-%m-%d").unwrap();
        
        let country =fields[5].parse::<Country>()?;
        let amount = fields[6].parse::<f64>().unwrap();
        let days_under_management= (transaction_end_date - transaction_start_date).num_days();

        let continent= country.country_to_continent();

        let transaction =Transaction {
            transaction_id: transaction_id,
            client_id:client_id,
            asset_name:asset_name,
            country:country,
            continent: continent,
            amount:amount,
            days_under_management:days_under_management,
        };

        return Ok(transaction);


    }

///
/// 
/// Return a reference to [Continent] from the [Transaction] record
/// 
/// * `self` - reference to a [Transaction] record
/// 
///  
    pub fn get_continent(&self) ->&Continent {
        &self.continent
    }
///
/// 
/// Return the investment amount from the [Transaction] record
/// 
/// * `self` - reference to [Transaction] record
/// 
///  
    
    
    pub fn get_amount(&self) -> f64 {
        self.amount
    }
}