use std::fmt;

pub struct Variable {
    pub x: u8,
    pub y: u8,
    pub domain: Vec<u8>,
}

/// Display trait implementation for Variable struct.
/// 
/// Simply prints all the information stored by the variable
/// in an easily readable format.
/// 
/// # Examples
/// 
/// ```
/// let var = Variable {x: 4, y: 2, domain: vec![1, 3, 3, 7]};
/// println!("{}", var);
/// // will print "Variable at (4, 2) with domain [1, 3, 3, 7];"
impl fmt::Display for Variable {
    // human-readably print a Variable.
    // might change this; maybe move this particulay representation
    // to the Debug trait, and make this more concise?
    // TODO
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Variable at ({}, {}) with domain {:?};", self.x, self.y, self.domain)
    }
}
