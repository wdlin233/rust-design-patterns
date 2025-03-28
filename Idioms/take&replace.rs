use std::mem;

enum MyEnum {
    A { name: String, x: u8 },
    B { name: String },
}

fn a_to_b(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        // This takes out our `name` and puts in an empty String instead
        // (note that empty strings don't allocate).
        // Then, construct the new enum variant (which will
        // be assigned to `*e`).
        *e = MyEnum::B {
            /// without cloning the `name`
            name: mem::take(name),
        }
    }
}

enum MultiVariateEnum {
    A { name: String },
    B { name: String },
    C,
    D,
}

/// We could of course `.clone()` name and put the clone into our `MyEnum::B`, 
/// but that would be an instance of the "Clone to satisfy the borrow checker"(4.1) anti-pattern.  
/// 
/// `mem::take` lets us swap out the value, replacing it with its default value, 
/// and returning the previous value. 
/// 
/// As a result, we get the original name as an owned value. We can then wrap this in another enum.
/// 
/// `mem::replace` is very similar. An equivalent to our `mem::take` line would be `mem::replace(name, String::new())`.
/// 
/// The type you are taking needs to implement the `Default` trait. 
/// However, if the type you’re working with doesn’t implement this, you can instead use `mem::replace`.
fn swizzle(e: &mut MultiVariateEnum) {
    use MultiVariateEnum::*;
    *e = match e {
        // Ownership rules do not allow taking `name` by value, but we cannot
        // take the value out of a mutable reference, unless we replace it:
        A { name } => B {
            name: mem::take(name),
        },
        B { name } => A {
            name: mem::take(name),
        },
        C => D,
        D => C,
    }
}

