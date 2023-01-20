mod student;
use student::Student;
use std::collections::LinkedList;

fn test_basics(){
    let s = Student::create(1234567, String::from("Smith"), String::from("Malory Logan")); 
    println!("{}\n", s.getPerm()); 
    println!("{}\n", s.getLastName()); 
    println!("{}\n", s.getFirstAndMiddleName()); 
    println!("{}\n", s.toString()); 


}



fn main() {
    let s = Student::create(1234567, String::from("Smith"), String::from("Malory Logan")); 
    let mut list:LinkedList<Student> = LinkedList::new();
    list.push_back(s);
    list.push_back(Student::create(2222222, String::from("john"), String::from("Bob Paul")));
    student::test_list(list);
}
