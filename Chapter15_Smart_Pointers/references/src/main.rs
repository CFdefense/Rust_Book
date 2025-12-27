/// Simple Box<T> Clone struct
/// 
/// Holds Generic type T
/// 
struct MyBox<T>(T);

/// MyBox<T> Box<T> clone Constructor
/// 
/// Holds Generic type T
/// 
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    // x contains the i32 value 5
    let x = 5;

    // y is a Box containing the copied x
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // assert_eq!(5, *y); - Error here cannot use dereference operator need to implement Deref

    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_reference() {
        // x contains the i32 value 5
        let x = 5;

        // y is a reference to x
        let y = &x;

        // We can simply assert 5 is x
        assert_eq!(5, x);

        // However for y we must deference it first to assert its value hiding behind the reference
        assert_eq!(5, *y);
    }

    #[test]
    fn test_box_reference() {
        // x contains the i32 value 5
        let x = 5;

        // y is a Box containing the copied x
        let y = Box::new(x);

        // We can simply assert 5 is x
        assert_eq!(5, x);

        // However for y we must still deference it first to assert its value hiding behind the box
        assert_eq!(5, *y);
    }
}