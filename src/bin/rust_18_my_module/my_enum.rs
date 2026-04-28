#[derive(PartialEq)]
pub enum MyEnum {
    FIRST,
    SECOND,
}

impl MyEnum {
    pub fn is_first(&self) -> bool {
        // matches!(self, MyEnum::FIRST)
        *self == MyEnum::FIRST
    }
    pub fn is_second(self) -> bool {
        matches!(self, MyEnum::SECOND)
    }
}
