use std::fmt;

#[derive(Debug)]
pub struct Variable {
    /// X-position in the sudoku grid.
    pub x: u8,

    /// Y-position in the sudoku grid.
    pub y: u8,

    /// Value held by this variable.
    /// The intention here is that it will be set to 0 by default,
    /// and will be given a valid value once definitely determined.
    pub value: u8,

    /// This variable's domain, i.e. its *possible* values.
    pub domain: Vec<u8>,
}

impl Variable {
    /// Variable's constructor, for convenience.
    /// You can do
    /// 
    /// ```
    /// let var = Variable::new(4, 2, 0, vec![1, 3, 3, 7]);
    /// ```
    /// 
    /// instead of
    /// 
    /// ```
    /// let var = Variable {x: 4, y: 2, value: 0, domain: vec![1, 3, 3, 7]};
    /// ```
    /// 
    /// to save some keystrokes.
    pub fn new(x: u8, y: u8, value: u8, domain: Vec<u8>) -> Variable {
        Variable {
            x: x,
            y: y,
            value: value,
            domain: domain,
        }
    }
}
impl fmt::Display for Variable {
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
    /// 
    /// This will print:
    /// 
    /// ```
    /// Variable (4, 2): [1, 3, 3, 7]
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Variable[{}] ({}, {}): {:?}", self.value, self.x, self.y, self.domain)
    }
}
