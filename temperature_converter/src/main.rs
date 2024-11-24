use std::io;

fn main() {
    println!("Convertidor de temperaturas Fahrenheit, Celsius y Kelvin");
    println!("--------------------------------------------------------");
    println!("1. Fahrenheit a Celsius.");
    println!("2. Fahrenheit a Kelvin.");
    println!("3. Celsius a Fahrenheit.");
    println!("4. Celsius a Kelvin.");
    println!("5. Kelvin a Fahrenheit.");
    println!("6. Kelvin a Celsius.");
    println!("");
    println!("Tu opción: ");

    let mut option = String::new();

    io::stdin()
        .read_line(&mut option)
        .expect("Error al leer la línea.");

    let option_num: i32  = option.trim().parse().unwrap_or(0);

    match option_num {
        1 => fahrenheit_celsius(),
        2 => fahrenheit_kelvin(),
        3 => celsius_fahrenheit(),
        4 => celsius_kelvin(),
        5 => kelvin_fahrenheit(),
        6 => kelvin_celsius(),
        _ => println!("No hay mas opciones."),
    };

}

fn ask_value() -> f32 {
    println!("");
    println!("Your input: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la línea.");

    let transformed_input: f32  = input.trim().parse().unwrap_or(0.0);
    return transformed_input
}

fn fahrenheit_celsius() {
    let value = ask_value();
    let result = ((value - 32.0) * 5.0) / 9.0;

    println!("{}° Fahrenheit are {}° Celsius", value, result);
}

fn fahrenheit_kelvin() {
    let value = ask_value();
    let result = ((value - 32.0) / 1.8) +273.15;

    println!("{}° Fahrenheit are {}° Kelvin", value, result);
}

fn celsius_fahrenheit() {
    let value = ask_value();
    let result = ((value * 9.0) / 5.0) + 32.0;

    println!("{}° Celsius are {}° Fahrenheit", value, result);
}

fn celsius_kelvin() {
    let value = ask_value();
    let result = value + 273.15;

    println!("{}° Celsius are {}° Kelvin", value, result);
}

fn kelvin_fahrenheit() {
    let value = ask_value();
    let result = ((value - 273.15) * 1.8) + 32.0;

    println!("{}° Kelvin are {}° Fahrenheit", value, result);
}

fn kelvin_celsius() {
    let value = ask_value();
    let result = value - 273.15;

    println!("{}° Kelvin are {}° Celsius", value, result);
}

