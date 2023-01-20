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
fn test_list(){
    let s = Student::create(1234567, String::from("Smith"), String::from("Malory Logan")); 
    let mut list:LinkedList<Student> = LinkedList::new();
    let a = s.clone();
    let b = Student::copy(a);
    list.push_back(s);
    list.push_back(b);
    list.push_back(Student::create(2222222, String::from("john"), String::from("Bob Paul")));
    student::print_list(list);
}



fn main() {
    test_basics(); 
    test_list();
}
