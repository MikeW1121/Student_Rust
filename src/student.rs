pub struct Student{
    pub perm:i32, 
    pub name:String,

}

impl Student{
    pub fn new() -> Self{
        Student{
            perm: 0,
            name: String::from(""),
        }
    }
    pub fn create(name:String, perm:i32) -> Self{
        Self{
            name: name, 
            perm: perm,
        }
    }
    pub fn getName(&self) -> &str{
        &self.name
    }
    pub fn getPerm(&self) -> i32{
        self.perm
    }
    pub fn setName(&mut self, x: String){
        self.name = x;
    }
    pub fn setPerm(&mut self, x: i32){
        self.perm = x; 
    }
    pub fn copy(x:Student)-> Student{
        Student{
            name: x.name, 
            perm: x.perm,
            
        }
    }
    pub fn toString(&self) -> String{
        let mut s:String = String::from(""); 
        s = s + self.getName() + ", " + self.getPerm().to_string().as_str(); 
        s
    }
}