fn baz() -> Result<(), ()> {
    // some code, run successfully or not
}

/// an object’s destructor can be used to run code that must be run before exit.
/// Rust warns about unused variables. Prefixing with _ signals to the compiler that 
/// the variable is intentionally unused (solely for its destructor), suppressing warnings.
/// The lifetime of _exit **lasts until bar() exits**, regardless of how the function terminates.

fn bar() -> Result<(), ()> {
    // These don't need to be defined inside the function.
    struct Foo;

    // Implement a destructor for Foo.
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("exit");
        }
    }

    // The variable name must start with _ if the variable is only used as a finalizer
    // The dtor of _exit will run however the function `bar` is exited.
    let _exit = Foo;
    /// If baz() returns Err, ? triggers early return, but _exit's destructor still executes before exiting.
    /// If baz() succeeds, _exit leaves scope at Ok(()), invoking its destructor.
    // Implicit return with `?` operator.
    baz()?;
    // Normal return.
    Ok(())
}