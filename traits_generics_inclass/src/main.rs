// Asignment Instructions:
// define 2 struct undegrad and grad student

// define trait show info

// grad student should have a thesis compnent
// gpa and major will be shared

// create another struct called Enrollment
// inside enrollment store undegrad and grads together
// implement show_info  for all enrolled student

// everywhere use generics and traits, no if or match statement
// program to behavior only

pub trait Info{
    fn show_info(&self) -> String;
}

pub struct GradStudent{
    pub thesis:String,
    pub gpa:f64,
    pub major:String,
}
impl Info for GradStudent{
    fn show_info(&self) -> String {
        format!("Graduate Student: Thesis= {}, GPA= {} , Major= {}", self.thesis, self.gpa, self.major)
    }
}

pub struct UndergradStudent {
    pub gpa:f64,
    pub major:String,
}
impl Info for UndergradStudent{
    fn show_info(&self) -> String {
        format!("Undergraduate Student: GPA= {} , Major= {}", self.gpa, self.major)
    }
}

pub struct Enrollment<'a>{
    pub enrolled_students: Vec<&'a dyn Info>,
    // note needed to include 'a lifetime annotation because compiler was complaining
    // with it we can now inside enrollment store undergrad and grads together
    // like was done in 02rust-traits-guide.md section 2. Same Method, Same Logical Entity
}

fn traits_generics_inclass(){
    let gs = GradStudent {
    thesis: String::from("ML"),
    gpa: 3.9,
    major: String::from("CompSci"),
    };
    println!("{}",gs.show_info()); // individual grad check

    let us = UndergradStudent {
        gpa: 2.7,
        major: String::from("CompSci"),
    };
    println!("{}",us.show_info()); // induvidual undergrad check

    let enrollment = Enrollment{
        enrolled_students:vec![&gs,&us],
    };

    for student in enrollment.enrolled_students.iter(){
        println!("Student: {}", student.show_info());
    }
}

fn main() {
    traits_generics_inclass();
}
