fn main() {
    println!("Hello, world!");
}

pub fn add(a: u32, b: u32) -> u32{
    a + b
}

#[test]
fn test_add(){
    assert_eq!(10 + 20, add(10, 20))
}

#[test]
fn test_check(){
    assert!(true)
}