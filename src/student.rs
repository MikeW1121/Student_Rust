use std::collections::LinkedList;
#[derive(Clone)]
pub struct Student{
    pub perm:i32, 
    pub lastName:String,
    pub firstAndMiddleName:String,
}

impl Student{
    pub fn new() -> Self{
        Student{
            perm: 0,
            lastName: String::from(""),
            firstAndMiddleName: String::from(""),
        }
    }
    pub fn create(perm:i32, lastName:String, firstAndMiddleName:String) -> Self{
        Self{
            perm: perm,
            lastName:lastName, 
            firstAndMiddleName:firstAndMiddleName,
        }
    }
    pub fn getLastName(&self) -> &str{
        &self.lastName
    }
    pub fn getFirstAndMiddleName(&self) -> &str{
        &self.firstAndMiddleName
    }
    pub fn getFullName(&self) -> String{
        let mut name = String::from(""); 
        name = name + &self.getLastName() + ", " + &self.getFirstAndMiddleName();
        name
    }
    pub fn getPerm(&self) -> i32{
        self.perm
    }

    pub fn copy(x:Student)-> Student{
        Student::create(x.perm, x.lastName, x.firstAndMiddleName)
        // Student{
        //     lastName: (x).lastName, 
        //     firstAndMiddleName:x.firstAndMiddleName,
        //     perm: x.perm,
            
        // }

    }
    pub fn toString(&self) -> String{
        let mut s:String = String::from(""); 
        s = s + self.getPerm().to_string().as_str() + ", " + self.getFullName().as_str(); 
        s
    }
}
pub fn print_list(list:LinkedList<Student>){

    for element in list.iter(){
        println!("{}\n", element.toString());
    }
}