/// Since Option implements IntoIterator, it can be used as an argument to `.extend()`
let turing = Some("Turing");
let mut logicians = vec!["Curry", "Kleene", "Markov"];

logicians.extend(turing);

// equivalent to
if let Some(turing_inner) = turing {
    logicians.push(turing_inner);
}

/// `.chain()`
let turing = Some("Turing");
let logicians = vec!["Curry", "Kleene", "Markov"];

for logician in logicians.iter().chain(turing.iter()) {
    println!("{logician} is a logician");
}

/// Also, since `Option` implements `IntoIterator`, 
/// it’s possible to iterate over it using a for loop. 
/// This is equivalent to matching it with `if let Some(..)`.
