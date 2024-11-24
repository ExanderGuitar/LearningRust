use std::io;

fn main() {
    let mut list = Vec::new();
    println!("{}", list.len());
    list.push("Hello");
    println!("{}", list[0]);

    loop {
        println!("Shopping list management");
        println!("------------------------");
        println!("1.Show the shopping list.");
        println!("2.Add a new element.");
        println!("3.Modify an element.");
        println!("4.Delete an element.");
        println!("5.Exit from the application.");

        let mut answer_string = String::new();

        io::stdin()
            .read_line(&mut answer_string)
            .expect("Error reading the input.");

        let answer_clean = answer_string.trim_end();
        let answer: u32 = answer_clean.parse().expect("Error al convertir a u entero.");

        match answer {
            1 => println!("Showing shopping list."),
            2 => println!("Adding a new element."),
            3 => println!("Modifying an existing element."),
            4 => println!("Delete an element."),
            5 => {
                println!("Leaving the application.");
                break;
            },
            _ => println!("Error in your selection."),
        }
    }
}
