
pub mod type_conversions {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    #[derive(Clone)]
    pub enum Typ {
        // this was the only other option damnit
        f64(f64),
        i64(i64),
        u64(u64),
        f32(f32),
        i32(i32),
        u32(u32),
        i16(i16),
        u16(u16),
        u8(u8),
        i8(i8),
        bool(bool),
        None
    }

    impl Typ {
        pub fn into_f64(&self) -> f64 {
            match self {
                Typ::f64(x) => *x,
                Typ::i64(x) => *x as f64,
                Typ::u64(x) => *x as f64,
                Typ::f32(x) => *x as f64,
                Typ::i32(x) => *x as f64,
                Typ::u32(x) => *x as f64,
                Typ::i16(x) => *x as f64,
                Typ::u16(x) => *x as f64,
                Typ::i8(x) => *x as f64,
                Typ::u8(x) => *x as f64,
                Typ::bool(x) => (*x as u8) as f64,
                _ => panic!("Unsupported type for f64 conversion"),
            }
        }

        pub fn into_i64(&self) -> i64 {
            match self {
                Typ::i64(x) => *x,
                Typ::i32(x) => *x as i64,
                Typ::i16(x) => *x as i64,
                Typ::i8(x) => *x as i64,
                Typ::u64(x) => *x as i64,
                Typ::u32(x) => *x as i64,
                Typ::u16(x) => *x as i64,
                Typ::u8(x) => *x as i64,
                Typ::f64(x) => *x as i64,
                Typ::f32(x) => *x as i64,
                Typ::bool(x) => (*x as u8) as i64,
                _ => panic!("Unsupported type for i64 conversion"),
            }
        }

        pub fn into_u64(&self) -> u64 {
            match self {
                Typ::u64(x) => *x,
                Typ::u32(x) => *x as u64,
                Typ::u16(x) => *x as u64,
                Typ::u8(x) => *x as u64,
                Typ::i64(x) => *x as u64,
                Typ::i32(x) => *x as u64,
                Typ::i16(x) => *x as u64,
                Typ::i8(x) => *x as u64,
                Typ::f64(x) => *x as u64,
                Typ::f32(x) => *x as u64,
                Typ::bool(x) => (*x as u8) as u64,
                _ => panic!("Unsupported type for u64 conversion"),
            }
        }

        pub fn into_f32(&self) -> f32 {
            match self {
                Typ::f32(x) => *x,
                Typ::f64(x) => *x as f32,
                Typ::i32(x) => *x as f32,
                Typ::u32(x) => *x as f32,
                Typ::i64(x) => *x as f32,
                Typ::u64(x) => *x as f32,
                Typ::i16(x) => *x as f32,
                Typ::u16(x) => *x as f32,
                Typ::i8(x) => *x as f32,
                Typ::u8(x) => *x as f32,
                Typ::bool(x) => (*x as u8) as f32,
                _ => panic!("Unsupported type for f32 conversion"),
            }
        }

        pub fn into_i32(&self) -> i32 {
            match self {
                Typ::i32(x) => *x,
                Typ::i64(x) => *x as i32,
                Typ::i16(x) => *x as i32,
                Typ::i8(x) => *x as i32,
                Typ::u32(x) => *x as i32,
                Typ::u64(x) => *x as i32,
                Typ::u16(x) => *x as i32,
                Typ::u8(x) => *x as i32,
                Typ::f32(x) => *x as i32,
                Typ::f64(x) => *x as i32,
                Typ::bool(x) => (*x as u8) as i32,
                _ => panic!("Unsupported type for i32 conversion"),
            }
        }

        pub fn into_u32(&self) -> u32 {
            match self {
                Typ::u32(x) => *x,
                Typ::u64(x) => *x as u32,
                Typ::u16(x) => *x as u32,
                Typ::u8(x) => *x as u32,
                Typ::i32(x) => *x as u32,
                Typ::i64(x) => *x as u32,
                Typ::i16(x) => *x as u32,
                Typ::i8(x) => *x as u32,
                Typ::f32(x) => *x as u32,
                Typ::f64(x) => *x as u32,
                Typ::bool(x) => (*x as u8) as u32,
                _ => panic!("Unsupported type for u32 conversion"),
            }
        }

        pub fn into_i16(&self) -> i16 {
            match self {
                Typ::i16(x) => *x,
                Typ::i32(x) => *x as i16,
                Typ::i64(x) => *x as i16,
                Typ::i8(x) => *x as i16,
                Typ::u16(x) => *x as i16,
                Typ::u32(x) => *x as i16,
                Typ::u64(x) => *x as i16,
                Typ::u8(x) => *x as i16,
                Typ::f32(x) => *x as i16,
                Typ::f64(x) => *x as i16,
                Typ::bool(x) => (*x as u8) as i16,
                _ => panic!("Unsupported type for i16 conversion"),
            }
        }

        pub fn into_u16(&self) -> u16 {
            match self {
                Typ::u16(x) => *x,
                Typ::u32(x) => *x as u16,
                Typ::u64(x) => *x as u16,
                Typ::u8(x) => *x as u16,
                Typ::i16(x) => *x as u16,
                Typ::i32(x) => *x as u16,
                Typ::i64(x) => *x as u16,
                Typ::i8(x) => *x as u16,
                Typ::f32(x) => *x as u16,
                Typ::f64(x) => *x as u16,
                Typ::bool(x) => (*x as u8) as u16,
                _ => panic!("Unsupported type for u16 conversion"),
            }
        }

        pub fn into_u8(&self) -> u8 {
            match self {
                Typ::u8(x) => *x,
                Typ::u16(x) => *x as u8,
                Typ::u32(x) => *x as u8,
                Typ::u64(x) => *x as u8,
                Typ::i8(x) => *x as u8,
                Typ::i16(x) => *x as u8,
                Typ::i32(x) => *x as u8,
                Typ::i64(x) => *x as u8,
                Typ::f32(x) => *x as u8,
                Typ::f64(x) => *x as u8,
                Typ::bool(x) => *x as u8,
                _ => panic!("Unsupported type for u8 conversion"),
            }
        }

        pub fn into_i8(&self) -> i8 {
            match self {
                Typ::i8(x) => *x,
                Typ::i16(x) => *x as i8,
                Typ::i32(x) => *x as i8,
                Typ::i64(x) => *x as i8,
                Typ::u8(x) => *x as i8,
                Typ::u16(x) => *x as i8,
                Typ::u32(x) => *x as i8,
                Typ::u64(x) => *x as i8,
                Typ::f32(x) => *x as i8,
                Typ::f64(x) => *x as i8,
                Typ::bool(x) => (*x as u8) as i8,
                _ => panic!("Unsupported type for i8 conversion"),
            }
        }

        pub fn into_bool(&self) -> bool {
            match self {
                Typ::bool(x) => *x,
                Typ::f64(x) => *x != 0.0,
                Typ::i64(x) => *x != 0,
                Typ::u64(x) => *x != 0,
                Typ::f32(x) => *x != 0.0,
                Typ::i32(x) => *x != 0,
                Typ::u32(x) => *x != 0,
                Typ::i16(x) => *x != 0,
                Typ::u16(x) => *x != 0,
                Typ::u8(x)  => *x != 0,
                Typ::i8(x)  => *x != 0,
                _ => panic!("Unsupported type for bool conversion"),
            }
        }
    }

    use crate::{Agent, Data, Shared};
    use std::collections::HashMap;

    pub trait TypToPrim<'a> {
        fn states(&self) -> &HashMap<&'a str, Data>;

        fn f64(&self, tag: &str) -> f64 {
            self.states().get(tag).map_or_else(
                || panic!("State does not exist: {}", tag),
                |data| match data.val {
                    Typ::f64(x) => x,
                    Typ::f32(x) => x as f64,
                    Typ::i64(x) => x as f64,
                    Typ::u64(x) => x as f64,
                    Typ::i32(x) => x as f64,
                    Typ::u32(x) => x as f64,
                    Typ::i16(x) => x as f64,
                    Typ::u16(x) => x as f64,
                    Typ::i8(x)  => x as f64,
                    Typ::u8(x)  => x as f64,
                    Typ::bool(x) => (x as u8) as f64,
                    _ => panic!("Type mismatch for f64"),
                }
            )
        }

        fn i64(&self, tag: &str) -> i64 {
            self.states().get(tag).map_or_else(
                || panic!("State does not exist: {}", tag),
                |data| match data.val {
                    Typ::i64(x) => x,
                    Typ::f64(x) => x as i64,
                    Typ::f32(x) => x as i64,
                    Typ::u64(x) => x as i64,
                    Typ::i32(x) => x as i64,
                    Typ::u32(x) => x as i64,
                    Typ::i16(x) => x as i64,
                    Typ::u16(x) => x as i64,
                    Typ::i8(x)  => x as i64,
                    Typ::u8(x)  => x as i64,
                    Typ::bool(x) => (x as u8) as i64,
                    _ => panic!("Type mismatch for i64"),
                }
            )
        }

        fn u64(&self, tag: &str) -> u64 {
            self.states().get(tag).map_or_else(
                || panic!("State does not exist: {}", tag),
                |data| match data.val {
                    Typ::u64(x) => x,
                    Typ::f64(x) => x as u64,
                    Typ::f32(x) => x as u64,
                    Typ::i64(x) => x as u64,
                    Typ::i32(x) => x as u64,
                    Typ::u32(x) => x as u64,
                    Typ::i16(x) => x as u64,
                    Typ::u16(x) => x as u64,
                    Typ::i8(x)  => x as u64,
                    Typ::u8(x)  => x as u64,
                    Typ::bool(x) => (x as u8) as u64,
                    _ => panic!("Type mismatch for u64"),
                }
            )
        }

        fn f32(&self, tag: &str) -> f32 {
            self.states().get(tag).map_or_else(
                || panic!("State does not exist: {}", tag),
                |data| match data.val {
                    Typ::f32(x) => x,
                    Typ::f64(x) => x as f32,
                    Typ::i64(x) => x as f32,
                    Typ::u64(x) => x as f32,
                    Typ::i32(x) => x as f32,
                    Typ::u32(x) => x as f32,
                    Typ::i16(x) => x as f32,
                    Typ::u16(x) => x as f32,
                    Typ::i8(x)  => x as f32,
                    Typ::u8(x)  => x as f32,
                    Typ::bool(x) => (x as u8) as f32,
                    _ => panic!("Type mismatch for f32"),
                }
            )
        }

        fn i32(&self, tag: &str) -> i32 {
            self.states().get(tag).map_or_else(
                || panic!("State does not exist: {}", tag),
                |data| match data.val {
                    Typ::i32(x) => x,
                    Typ::f64(x) => x as i32,
                    Typ::f32(x) => x as i32,
                    Typ::i64(x) => x as i32,
                    Typ::u64(x) => x as i32,
                    Typ::u32(x) => x as i32,
                    Typ::i16(x) => x as i32,
                    Typ::u16(x) => x as i32,
                    Typ::i8(x)  => x as i32,
                    Typ::u8(x)  => x as i32,
                    Typ::bool(x) => (x as u8) as i32,
                    _ => panic!("Type mismatch for i32"),
                }
            )
        }

        fn u32(&self, tag: &str) -> u32 {
            self.states().get(tag).map_or_else(
                || panic!("State does not exist: {}", tag),
                |data| match data.val {
                    Typ::u32(x) => x,
                    Typ::f64(x) => x as u32,
                    Typ::f32(x) => x as u32,
                    Typ::i64(x) => x as u32,
                    Typ::u64(x) => x as u32,
                    Typ::i32(x) => x as u32,
                    Typ::i16(x) => x as u32,
                    Typ::u16(x) => x as u32,
                    Typ::i8(x)  => x as u32,
                    Typ::u8(x)  => x as u32,
                    Typ::bool(x) => (x as u8) as u32,
                    _ => panic!("Type mismatch for u32"),
                }
            )
        }

        fn i16(&self, tag: &str) -> i16 {
            self.states().get(tag).map_or_else(
                || panic!("State does not exist: {}", tag),
                |data| match data.val {
                    Typ::i16(x) => x,
                    Typ::i32(x) => x as i16,
                    Typ::u32(x) => x as i16,
                    Typ::i64(x) => x as i16,
                    Typ::u64(x) => x as i16,
                    Typ::f32(x) => x as i16,
                    Typ::f64(x) => x as i16,
                    Typ::u16(x) => x as i16,
                    Typ::u8(x)  => x as i16,
                    Typ::i8(x)  => x as i16,
                    Typ::bool(x) => (x as u8) as i16,
                    _ => panic!("Type mismatch for i16"),
                }
            )
        }

        fn u16(&self, tag: &str) -> u16 {
            self.states().get(tag).map_or_else(
                || panic!("State does not exist: {}", tag),
                |data| match data.val {
                    Typ::u16(x) => x,
                    Typ::i16(x) => x as u16,
                    Typ::i32(x) => x as u16,
                    Typ::u32(x) => x as u16,
                    Typ::i64(x) => x as u16,
                    Typ::u64(x) => x as u16,
                    Typ::f32(x) => x as u16,
                    Typ::f64(x) => x as u16,
                    Typ::u8(x)  => x as u16,
                    Typ::i8(x)  => x as u16,
                    Typ::bool(x) => (x as u8) as u16,
                    _ => panic!("Type mismatch for u16"),
                }
            )
        }

        fn u8(&self, tag: &str) -> u8 {
            self.states().get(tag).map_or_else(
                || panic!("State does not exist: {}", tag),
                |data| match data.val {
                    Typ::u8(x) => x,
                    Typ::i8(x) => x as u8,
                    Typ::i16(x) => x as u8,
                    Typ::u16(x) => x as u8,
                    Typ::i32(x) => x as u8,
                    Typ::u32(x) => x as u8,
                    Typ::i64(x) => x as u8,
                    Typ::u64(x) => x as u8,
                    Typ::f32(x) => x as u8,
                    Typ::f64(x) => x as u8,
                    Typ::bool(x) => x as u8,
                    _ => panic!("Type mismatch for u8"),
                }
            )
        }

        fn i8(&self, tag: &str) -> i8 {
            self.states().get(tag).map_or_else(
                || panic!("State does not exist: {}", tag),
                |data| match data.val {
                    Typ::i8(x) => x,
                    Typ::u8(x) => x as i8,
                    Typ::i16(x) => x as i8,
                    Typ::u16(x) => x as i8,
                    Typ::i32(x) => x as i8,
                    Typ::u32(x) => x as i8,
                    Typ::i64(x) => x as i8,
                    Typ::u64(x) => x as i8,
                    Typ::f32(x) => x as i8,
                    Typ::f64(x) => x as i8,
                    Typ::bool(x) => (x as u8) as i8,
                    _ => panic!("Type mismatch for i8"),
                }
            )
        }

        fn bool(&self, tag: &str) -> bool {
            self.states().get(tag).map_or_else(
                || panic!("State does not exist: {}", tag),
                |data| match data.val {
                    Typ::bool(x) => x,
                    Typ::f64(x) => x != 0.0,
                    Typ::f32(x) => x != 0.0,
                    Typ::i64(x) => x != 0,
                    Typ::u64(x) => x != 0,
                    Typ::i32(x) => x != 0,
                    Typ::u32(x) => x != 0,
                    Typ::i16(x) => x != 0,
                    Typ::u16(x) => x != 0,
                    Typ::i8(x)  => x != 0,
                    Typ::u8(x)  => x != 0,
                    _ => panic!("Type mismatch for bool"),
                }
            )
        }
    }

    impl<'a> TypToPrim<'a> for Agent<'a> { fn states(&self) -> &HashMap<&'a str, Data> { &self.states } }
    impl<'a> TypToPrim<'a> for Shared<'a> { fn states(&self) -> &HashMap<&'a str, Data> { &self.states } }


    pub trait ToTyp {
        fn typ(self) -> Typ;
    }

    impl ToTyp for f64 {
        fn typ(self) -> Typ {
            Typ::f64(self)
        }
    }

    impl ToTyp for i64 {
        fn typ(self) -> Typ {
            Typ::i64(self)
        }
    }

    impl ToTyp for u64 {
        fn typ(self) -> Typ {
            Typ::u64(self)
        }
    }

    impl ToTyp for f32 {
        fn typ(self) -> Typ {
            Typ::f32(self)
        }
    }

    impl ToTyp for i32 {
        fn typ(self) -> Typ {
            Typ::i32(self)
        }
    }

    impl ToTyp for u32 {
        fn typ(self) -> Typ {
            Typ::u32(self)
        }
    }

    impl ToTyp for i16 {
        fn typ(self) -> Typ {
            Typ::i16(self)
        }
    }

    impl ToTyp for u16 {
        fn typ(self) -> Typ {
            Typ::u16(self)
        }
    }

    impl ToTyp for i8 {
        fn typ(self) -> Typ {
            Typ::i8(self)
        }
    }

    impl ToTyp for u8 {
        fn typ(self) -> Typ {
            Typ::u8(self)
        }
    }

    impl ToTyp for bool {
        fn typ(self) -> Typ {
            Typ::bool(self)
        }
    }

}