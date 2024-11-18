#[derive(Debug, PartialEq)]
pub enum Major {
    ComputerScience,
    Biology,
    Unassigned
}

impl Major {
    pub fn classify(major: &str) -> Self {
        return match major {
            "CS" => Major::ComputerScience,
            "BIO" => Major::Biology,
            _ => Major::Unassigned
        }
    } 
}