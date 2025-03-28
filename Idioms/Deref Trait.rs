use std::ops::Deref;

struct Vec<T> {
    data: RawVec<T>,
    //..
}

/// Implementing Deref for Vec allows implicit dereferencing from &Vec<T> to &[T] 
/// and includes the relationship in auto-dereferencing searches. 
/// pros: Most methods can be implemented only for the borrowed view, they are then 
/// implicitly available for the owning view.
/// cons: Methods and traits only available via dereferencing are 
/// not taken into account when bounds checking
impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        //..
    }
}