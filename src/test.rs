use super::*;

#[derive(PartialEq, Debug)]
struct Test {
    key: i32,
    value: i32,
}

impl ToPartial<i32> for Test {
    fn to_partial(&self) -> &i32 {
        &self.key
    }
}

#[test]
fn test_equality() {
    assert_eq!(*Test { key: 1, value: 2 }.to_partial(), 1i32);
}
