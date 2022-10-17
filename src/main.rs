pub trait FooTrait {
    fn foo(&mut self, x: u64) -> u64;
}

pub struct FooStruct {
    pub val: u64,
}

impl FooTrait for FooStruct {
    fn foo(&mut self, x: u64) -> u64 {
        x + 1
    }
}

fn main() {
    peg::parser!{
        grammar list_parser() for str {
            rule number() -> u64
                = n:$(['0'..='9']+) {? n.parse().or(Err("u64")) }

            pub rule list() -> Box<impl FooTrait>
                = n:(number()) { Box::new(FooStruct { val: n }) }
        }
    }
}