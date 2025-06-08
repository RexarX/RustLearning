use std::io;

#[derive(Debug, Clone, Copy)]
enum TempUnit {
    Celsius,
    Fahrenheit,
}

impl TempUnit {
    fn symbol(&self) -> char {
        match self {
            TempUnit::Celsius => 'C',
            TempUnit::Fahrenheit => 'F',
        }
    }

    fn name(&self) -> &'static str {
        match self {
            TempUnit::Celsius => "Celsius",
            TempUnit::Fahrenheit => "Fahrenheit",
        }
    }

    fn opposite(&self) -> TempUnit {
        match self {
            TempUnit::Celsius => TempUnit::Fahrenheit,
            TempUnit::Fahrenheit => TempUnit::Celsius,
        }
    }
}

fn convert_temperature(temp: f32, from_unit: TempUnit) -> f32 {
    match from_unit {
        TempUnit::Celsius => temp * 9.0 / 5.0 + 32.0,
        TempUnit::Fahrenheit => (temp - 32.0) * 5.0 / 9.0,
    }
}

fn parse_temperature(input: &str) -> Result<(f32, TempUnit), String> {
    let input = input.trim().to_uppercase();

    if input.len() < 2 {
        return Err("Input too short".to_string());
    }

    let unit = match input.chars().last().unwrap() {
        'C' => TempUnit::Celsius,
        'F' => TempUnit::Fahrenheit,
        _ => return Err("Unit must be 'C' or 'F'".to_string()),
    };

    let temp_str = &input[..input.len() - 1];
    let temperature = temp_str
        .parse::<f32>()
        .map_err(|_| "Invalid temperature value".to_string())?;

    Ok((temperature, unit))
}

fn main() {
    println!("Enter temperature with unit (e.g., 100F, 37.5C) or 'quit' to exit.");

    let mut input = String::new();
    loop {
        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input");
            continue;
        }

        let input = input.trim();
        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("exit") {
            break;
        }

        match parse_temperature(input) {
            Ok((temperature, from_unit)) => {
                let converted = convert_temperature(temperature, from_unit);
                let to_unit = from_unit.opposite();

                println!(
                    "{:.1}°{} = {:.1}°{}",
                    temperature,
                    from_unit.symbol(),
                    converted,
                    to_unit.symbol()
                );
                break;
            }
            Err(error) => {
                println!("Error: {}", error);
                println!("Please enter temperature like: 100F or 37.5C");
            }
        }
    }
}
