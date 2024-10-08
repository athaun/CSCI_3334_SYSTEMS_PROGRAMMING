#[derive(Debug)]
enum GradeLevel {
    Bachelor,
    Master,
    PhD,
}

#[derive(Debug)]
enum Major {
    ComputerScience,
    ElectricalEngineering,
}

#[derive(Debug)]
struct Student {
    name:String,
    grade:GradeLevel,
    major:Major
}

impl Student {
    fn new(name: String, grade: GradeLevel, major: Major) -> Self {
        Student {
            name:name,
            grade:grade,
            major:major,
        }
    }

    fn introduce_yourself(&self) {
        let mut introduction: String = format!("Hello, my name is {}. I am a ", self.name);

        introduction += match self.grade {
            GradeLevel::Bachelor => "Bachelor's",
            GradeLevel::Master => "Master's",
            GradeLevel::PhD => "PhD",
        };

        introduction += " student majoring in ";
        introduction += match self.major {
            Major::ComputerScience => "Computer Science",
            Major::ElectricalEngineering => "Electrical Engineering",
        };

        println!("{}.", introduction);
    }
}

fn main() {
    let asher = Student::new("Asher Haun".to_string(), GradeLevel::Bachelor, Major::ComputerScience);
    asher.introduce_yourself();
}