//----------------------------------- Struct Intro In Class Assignment-----------------------------------

// Struct definition
struct Student{
    name:String,
    major:String,
}

// Struct implementation 
impl Student{
    // CONSTRUCTOR
    fn new(n:String,m:String) -> Student{
        Self{
            name: n,
            major: m,            
        }
    }

    // ------------- GETTER AND SETTER FOR NAME ----------------
    // --> note: didn't specify we needed this but wanted to add it for practice

    // getter method for name
    fn get_name(&self) -> &String{ 
        return &self.name
    }

    // setter method for name
    fn set_name(&mut self, new_name:String){
        self.name = new_name
    }

    // ------------- GETTER AND SETTER FOR MAJOR  --------------

    // getter method for major
    fn get_major(&self) -> &String{ 
        return &self.major
    }

    // setter method for major
    fn set_major(&mut self, new_major:String){
        self.major = new_major
    }
}


fn main(){
    /*
    // note: this is the previous way we discussed how to create and initialize
    // a new instance of a struct.
    // its replaced by constructor and ::new  
    //     --> this replaced way is better b/c it encapsulates the
    //         logic and looks like a normal constructor call 

    let my_student = Student{
        name: "Joanna".to_string(),
        major: "CompSci".to_string(),

    };

    println!("Students name: {}", my_student.name);
    println!("Students major: {}", my_student.major);
    */

    println!("--------------------------- Testing constructor ----------------------------");
    let mut my_student = Student::new("NAME TBA".to_string(),"MAJOR TBA".to_string());
    println!("Students Name is: {}", my_student.name);
    println!("Students Major: {}", my_student.major);
    
    println!("---------------------- Testing Getters and Setters -------------------------");
    my_student.set_name("Joanna Rios".to_string());
    my_student.set_major("Computer Science".to_string());
    println!("Students Name is: {}",my_student.get_name());
    println!("Students Major is: {}",my_student.get_major());
}
