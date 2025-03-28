use std::io;
use std::fs;

/// We do not need to allocate anything on the heap. 
/// Neither do we need to initialize something we won’t use later, nor do we need to 
/// monomorphize the whole code that follows to work with both `File` or `Stdin`.
 
// We need to describe the type to get dynamic dispatch.
let readable: &mut dyn io::Read = if arg == "-" {
    &mut io::stdin()
} else {
    &mut fs::File::open(arg)?
};

// Read from `readable` here.

