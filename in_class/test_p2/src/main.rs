mod student;
mod major;
use student::Student;

fn main() {
    let s1 = Student::new("Bob", "BIO");

    println!("{:?}", s1);
}
