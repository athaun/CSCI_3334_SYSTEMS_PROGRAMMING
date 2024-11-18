use crate::major::Major;

#[derive(Debug)]
pub struct Student {
    name: String,
    major: Major,
}

impl Student {
    pub fn new(name: &str, major: &str) -> Student {
        Student {
            name: name.to_string(),
            major: Major::classify(major)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_undefined_creation() {
        let s = Student::new("A", "Chemistry");
        assert_eq!(s.major, Major::Unassigned);
    }

    #[test]
    fn test_defined_creation() {
        let s = Student::new("B", "BIO");
        assert_eq!(s.major, Major::Biology);
    }
}