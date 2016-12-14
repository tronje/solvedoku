use std::fmt;

#[derive(Debug)]
pub struct Variable {
    pub x: u8,
    pub y: u8,
    pub domain: Vec<u8>,
}

/// Display trait implementation for Variable struct.
/// 
/// Simply prints all the information stored by the variable
/// in a very concise format.
/// 
/// # Examples
/// 
/// ```
/// let var = Variable {x: 4, y: 2, domain: vec![1, 3, 3, 7]};
/// println!("{}", var);
/// ```
/// This will print:
/// ```
/// Variable (4, 2): [1, 3, 3, 7]
/// ```
impl fmt::Display for Variable {
    // since we already derive the Debug trait, this makes
    // two ways to print a Variable. This one's a little more concise.
    // the debug format also prints the names of all fields.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Variable ({}, {}): {:?}", self.x, self.y, self.domain)
    }
}
