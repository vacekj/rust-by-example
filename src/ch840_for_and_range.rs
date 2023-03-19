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
            /* the ampersand here signifies a borrow. we can't mutate and we are not moving the value
            so it can be reused after returning from the loop */
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}

// into_iter - consumes the collection, on each iteration the exact data is provided.
// no longer available after because it has been moved within the loop

fn into_iter() {
    let names = vec!["Box", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            /* the ampersand here signifies a borrow. we can't mutate and we are not moving the value
            so it can be reused after returning from the loop */
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}

// iter_mut() - mutably borrows, allows collection to be modified in place

fn iter_mut() {
    let names = vec!["Box", "Frank", "Ferris"];

    for name in names.iter_mut() {
        match name {
            /* the ampersand here signifies a borrow. we can't mutate and we are not moving the value
            so it can be reused after returning from the loop */
            &mut "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}
