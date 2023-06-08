//!  Utility Module
//! 
//! `utils` is a collection of utilities to make certain business
//! calculation more convenient

use crate::transaction::Transaction;
use crate::location::Continent;

/// 
/// 
/// Return [Transaction]s done in a specific [Continent] 
/// 
/// # Arguments
///  
/// * `txns` - reference to [Transaction]s 
/// 
/// * `c` - reference to [Continent] 
/// 
/// 
pub fn transactions_per_continent<'a>(txns: &'a Vec<Transaction>, c: &Continent) -> Vec<&'a Transaction> {
        
    txns.into_iter().filter(|x| *x.get_continent() == *c).collect::<Vec<_>>()
} 