// a..b yields values from a incluse to b _exclusive_
// a..=b is inclusive to b

// for and iterators:
// for loop will aply the into_iter function to the collection.
// iter - borrows each element through each iteration, leaving the collection untouched
// and available for reause after the loop

fn iter() {
    let names = vec!["Box", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}

// into_iter - consumes the collection, on each iteration the exact data is provided.
// no longer available after because it has been moved within the loop
