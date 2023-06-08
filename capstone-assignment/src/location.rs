
//!  Location Module
//! 
//! `location` module defines [Country] and [Continent] enum 
//! and its associated functions and methods  
//! 

#[derive(Debug,Clone,Copy)]
/// The Country list used
pub enum Country {

    UnitedStates, Canada,
UnitedKingdom, Germany, France, Japan, Australia, China, Brazil, SouthKorea,
Ireland, Spain, India, Switzerland,
}

#[derive(Debug,PartialEq,Clone,Copy)]
/// The Continents of the world
pub enum Continent {
    NorthAmerica, Europe,
Asia, Oceania, SouthAmerica,
}

impl Continent {

/// 
/// 
/// Return Continent str from the Continent enum 
/// 
/// # Arguments
///  
/// * `self` - Continent enum
/// 

     pub fn convert_to_string(&self) -> Option<&str> {
         match &self {
            Continent::NorthAmerica => Some("NorthAmerica"),
            Continent::Europe => Some("Europe"),
            Continent::Asia => Some("Asia"),
            Continent::Oceania => Some("Oceania"),
            Continent::SouthAmerica => Some("SouthAmerica"),
            _ => None,
            
         }


     }
}
impl Country {
    
/// 
/// 
/// Return [Continent] where the [Country] is located
/// 
/// # Arguments
///  
/// * `self` - [Country]
/// 
    pub fn country_to_continent(&self) -> Continent {
        // match and then return Continent variant
        match &self {
            Country::UnitedStates | Country::Canada => Continent::NorthAmerica,
            Country::UnitedKingdom |Country::France | Country::Germany | Country::Ireland | Country::Spain | Country::Switzerland => Continent::Europe,
            Country::India | Country::China | Country::Japan | Country::SouthKorea => Continent::Asia,
            Country::Brazil =>Continent::SouthAmerica,
            Country::Australia => Continent::Oceania,
        }
    }
/// 
/// 
/// Return Country str from the Country enum 
/// 
/// # Arguments
///  
/// * `self` - [Country] enum
/// 


    pub fn convert_to_string(&self) -> Option<&str> {

        match &self{
            Country::UnitedStates =>  Some("USA"),
            Country::Canada  => Some("Canada") , 
            Country::UnitedKingdom => Some("UK") , 
            Country::Germany =>Some("Germany") ,
            Country::France => Some("France") , 
            Country::Japan => Some("Japan"),  
            Country::Australia => Some("Australia"), 
            Country::China => Some("China") , 
            Country::Brazil => Some("Brazil") , 
            Country::SouthKorea => Some("South Korea") ,
            Country::Ireland => Some("Ireland"), 
            Country::Spain => Some("Spain") ,
            Country::India => Some("India") , 
            Country::Switzerland => Some("Switzerland") ,
            _ => None,

        }
    }

}


/// 
/// 
/// Return `Option<Country>` from the Country str 
/// 
/// # Arguments
///  
/// * `self` - [Country] str
///
    impl std::str::FromStr for Country {
        type Err=&'static str;
        fn from_str(s: &str) -> Result<Self, Self::Err>{
            match s {
                "USA" => Ok(Country::UnitedStates),
                "Canada" => Ok(Country::Canada), 
                "UK" =>Ok(Country::UnitedKingdom), 
                "Germany" => Ok(Country::Germany),
                "France" => Ok(Country::France), 
                "Japan" => Ok(Country::Japan), 
                "Australia" => Ok(Country::Australia), 
                "China" => Ok(Country::China), 
                "Brazil" => Ok(Country::Brazil), 
                "South Korea" => Ok(Country::SouthKorea),
                "Ireland" => Ok(Country::Ireland), 
                "Spain" => Ok(Country::Spain),
                "India" => Ok(Country::India), 
                "Switzerland" => Ok(Country::Switzerland),
                _=> Err("Error in finding country"),

            }
        }
    }

