// expose our modules publicly
mod variable;
mod constraint;

pub mod csp {
    pub use variable::Variable;
    pub use constraint::Constraint;
}
