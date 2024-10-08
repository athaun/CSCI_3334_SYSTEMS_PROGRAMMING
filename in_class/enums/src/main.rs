#[derive(Debug)]
enum GradeLevel {
    Freshman,
    Sophomore,
    Junior,
    Senior
}

#[derive(Debug)]
enum Major {
    ComputerScience,
    ComputerEngineering
}

#[derive(Debug)]
struct Student {
    grade: GradeLevel,
    sid: u32,
    name: String,
    major: Major
}

impl Student {
    fn new(grade: GradeLevel, sid: u32, name: String, major: Major) -> Self {
        Student {
            grade: grade,
            sid: sid,
            name: name,
            major: major
        }
    }

    fn introduce_yourself(&self) {
        let mut str: String = "Hello, my name is ".to_string();
        str += &self.name;
        str += ". I am a ";

        str += match self.grade {
            GradeLevel::Freshman => "freshman",
            GradeLevel::Sophomore => "sophomore",
            GradeLevel::Junior => "junior",
            GradeLevel::Senior => "senior",
        };

        str += " majoring in ";
        str += match self.major {
            Major::ComputerScience => "Computer Science",
            Major::ComputerEngineering => "Computer Engineering",
        };
        str += ". My Student id is ";
        str += &self.sid.to_string();

        println!("{}.", str);
    }
}

fn main() {
    let student = Student::new(GradeLevel::Senior, 123456789, String::from("Asher Haun"), Major::ComputerScience);
    student.introduce_yourself();
}
