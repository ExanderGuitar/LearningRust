use std::io;

fn main() {
    let no_variable = 10;
    println!("My variable no variable contians: {}", no_variable);

    let mut variable = 5;
    println!("My variable stores: {}", variable);
    variable = 30;
    println!("My variable now stores: {}", variable);

    let known_variable: f64 = 2.12;
    println!("My variable for sure is a 64 bits float and stores: {}", known_variable);

    let (tuplin1, tuplin2, tuplin3) = ("Es", "una", "tupla");
    println!("Values of my tuple are: {}, {}, {}", tuplin1, tuplin2, tuplin3);

    const PERMANENCE: i32 = 2;
    println!("For sure this value never changes: {}", PERMANENCE);

    println!("Can you comment your opinion about variables?");
    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Error reading the line");
    
    println!("Your opinion is {}", answer);

    fn waving (name: String) {
        println!("Hello {}", name)
    }

    waving("Alejandro".to_string());

    fn construct_waving (name: &mut String) {
        name.insert_str(0, "Hello ");
    }

    let mut name = String::from("Alejandro");

    construct_waving(&mut name);

    println!("{}", name);

    let condition = false;
    let my_number_guay = if condition {4} else {9};
    println!("Look at this number {}", my_number_guay);
    
    let noseque = [5; 10];
    for n in noseque {
        println!("{}", n);
    }
}
    
