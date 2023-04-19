use serde::{Serialize, Deserialize};


//type of cgroup used throughout the program 
//for bookkeeping 
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Cgroup {
    pub name: String,
    pub delete: u8,
}

impl Cgroup {

    pub fn new(name: String) -> Self {
        Cgroup{name: name, delete: 0}       
    }


}