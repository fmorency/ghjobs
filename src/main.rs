use minicbor::{Decode, Encode};

#[derive(Debug, Encode, Decode, PartialEq)]
struct Foo {
    #[n(0)]
    x: u32,

    #[n(1)]
    y: u32,
}

#[derive(Debug, Encode, Decode, PartialEq)]
struct Bar {
    #[n(0)]
    z: u32,
}

struct Unit;

fn main() {
    let a = minicbor::to_vec(Foo { x: 50, y: 100 }).unwrap();
    let b: Foo = minicbor::decode(&a).unwrap();

    println!("{:?}", b);
}

#[test]
fn foo() {
    let a = minicbor::to_vec(Foo { x: 51, y: 101 }).unwrap();
    let b: Foo = minicbor::decode(&a).unwrap();

    assert_eq!(b, Foo { x: 51, y: 101 });
}

#[test]
fn unit() {
    let _ = Unit;
}
