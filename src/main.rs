mod student;
use student::Student;

fn test_basics(){
    let s = Student::create(1234567, String::from("Smith"), String::from("Malory Logan")); 
    println!("{}\n", s.getPerm()); 
    println!("{}\n", s.getLastName()); 
    println!("{}\n", s.getFirstAndMiddleName()); 
    println!("{}\n", s.toString()); 


}
fn main() {
    test_basics();
}
