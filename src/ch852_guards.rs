enum Temperature {
    Celsius(i32),
    FreedomUnits(i32),
}

// additional guards to a match arm

fn main() {
    let temp = Temperature::Celsius(35);

    match temp {
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),
        Temperature::FreedomUnits(t) if t > 86 => println!("F is above 86 fahrenheit"),
        _ => (),
    }
}

// !!! compiler doesn't take guard conditions into account when doing
// exhaustiveness checking

fn exhaust() {
    let number: u8 = 4;

    match number {
        i if i == 0 => println!("zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => unreachable!(
            "this will never happen, but the compiler doesn't know, since that's a runtime check"
        ),
    }
}
