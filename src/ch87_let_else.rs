fn process_value(optional_value: Option<i32>) -> Result<(), &'static str> {
    let Some(value) = optional_value else {
        return Err("The optional value is None");
    };
    println!("The optional value is: {:?}", value);
    Ok(())
}

fn main() {
    process_value(None);
}
