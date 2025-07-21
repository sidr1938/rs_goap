use std::collections::HashMap;
use crate::action_builders::*;
use crate::type_conversions::*;
use dyn_clone::DynClone;
use dyn_clone::clone_trait_object;

pub trait GlobalCloneableFnF32: Fn(&Agent, &World) -> f32 + DynClone {}
impl<T> GlobalCloneableFnF32 for T where T: Fn(&Agent, &World) -> f32 + Clone {}
pub trait GlobalCloneableFnTyp: Fn(&Agent, &World) -> Typ + DynClone {}
impl<T> GlobalCloneableFnTyp for T where T: Fn(&Agent, &World) -> Typ + Clone {}

pub trait CloneableFnF32: Fn(&Agent) -> f32 + DynClone {}
impl<T> CloneableFnF32 for T where T: Fn(&Agent) -> f32 + Clone {}
pub trait CloneableFnTyp: Fn(&Agent) -> Typ + DynClone {}
impl<T> CloneableFnTyp for T where T: Fn(&Agent) -> Typ + Clone {}

// NEVER OPEN THIS DAMN MODULE
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

    use crate::{Agent, World};
    use std::collections::HashMap;

    pub trait TypToPrim<'a> {
        fn states(&self) -> &HashMap<&'a str, Typ>;

        fn f64(&self, tag: &str) -> f64 {
            match self.states().get(tag) {
                Some(Typ::f64(x))  => *x,
                Some(Typ::f32(x))  => *x as f64,
                Some(Typ::i64(x))  => *x as f64,
                Some(Typ::u64(x))  => *x as f64,
                Some(Typ::i32(x))  => *x as f64,
                Some(Typ::u32(x))  => *x as f64,
                Some(Typ::i16(x))  => *x as f64,
                Some(Typ::u16(x))  => *x as f64,
                Some(Typ::i8(x))   => *x as f64,
                Some(Typ::u8(x))   => *x as f64,
                Some(Typ::bool(x)) => (*x as u8) as f64,
                Some(_) => panic!("Type mismatch for f64"),
                None => panic!("State does not exist: {}", tag),
            }
        }

        fn i64(&self, tag: &str) -> i64 {
            match self.states().get(tag) {
                Some(Typ::i64(x))  => *x,
                Some(Typ::u64(x))  => *x as i64,
                Some(Typ::f64(x))  => *x as i64,
                Some(Typ::f32(x))  => *x as i64,
                Some(Typ::i32(x))  => *x as i64,
                Some(Typ::u32(x))  => *x as i64,
                Some(Typ::i16(x))  => *x as i64,
                Some(Typ::u16(x))  => *x as i64,
                Some(Typ::i8(x))   => *x as i64,
                Some(Typ::u8(x))   => *x as i64,
                Some(Typ::bool(x)) => (*x as u8) as i64,
                Some(_) => panic!("Type mismatch for i64"),
                None => panic!("State does not exist: {}", tag),
            }
        }

        fn u64(&self, tag: &str) -> u64 {
            match self.states().get(tag) {
                Some(Typ::u64(x))  => *x,
                Some(Typ::i64(x))  => *x as u64,
                Some(Typ::f64(x))  => *x as u64,
                Some(Typ::f32(x))  => *x as u64,
                Some(Typ::i32(x))  => *x as u64,
                Some(Typ::u32(x))  => *x as u64,
                Some(Typ::i16(x))  => *x as u64,
                Some(Typ::u16(x))  => *x as u64,
                Some(Typ::i8(x))   => *x as u64,
                Some(Typ::u8(x))   => *x as u64,
                Some(Typ::bool(x)) => (*x as u8) as u64,
                Some(_) => panic!("Type mismatch for u64"),
                None => panic!("State does not exist: {}", tag),
            }
        }

        fn f32(&self, tag: &str) -> f32 {
            match self.states().get(tag) {
                Some(Typ::f32(x))  => *x,
                Some(Typ::f64(x))  => *x as f32,
                Some(Typ::i64(x))  => *x as f32,
                Some(Typ::u64(x))  => *x as f32,
                Some(Typ::i32(x))  => *x as f32,
                Some(Typ::u32(x))  => *x as f32,
                Some(Typ::i16(x))  => *x as f32,
                Some(Typ::u16(x))  => *x as f32,
                Some(Typ::i8(x))   => *x as f32,
                Some(Typ::u8(x))   => *x as f32,
                Some(Typ::bool(x)) => (*x as u8) as f32,
                Some(_) => panic!("Type mismatch for f32"),
                None => panic!("State does not exist: {}", tag),
            }
        }

        fn i32(&self, tag: &str) -> i32 {
            match self.states().get(tag) {
                Some(Typ::i32(x))  => *x,
                Some(Typ::u32(x))  => *x as i32,
                Some(Typ::f64(x))  => *x as i32,
                Some(Typ::f32(x))  => *x as i32,
                Some(Typ::i64(x))  => *x as i32,
                Some(Typ::u64(x))  => *x as i32,
                Some(Typ::i16(x))  => *x as i32,
                Some(Typ::u16(x))  => *x as i32,
                Some(Typ::i8(x))   => *x as i32,
                Some(Typ::u8(x))   => *x as i32,
                Some(Typ::bool(x)) => (*x as u8) as i32,
                Some(_) => panic!("Type mismatch for i32"),
                None => panic!("State does not exist: {}", tag),
            }
        }

        fn u32(&self, tag: &str) -> u32 {
            match self.states().get(tag) {
                Some(Typ::u32(x))  => *x,
                Some(Typ::i32(x))  => *x as u32,
                Some(Typ::f64(x))  => *x as u32,
                Some(Typ::f32(x))  => *x as u32,
                Some(Typ::i64(x))  => *x as u32,
                Some(Typ::u64(x))  => *x as u32,
                Some(Typ::i16(x))  => *x as u32,
                Some(Typ::u16(x))  => *x as u32,
                Some(Typ::i8(x))   => *x as u32,
                Some(Typ::u8(x))   => *x as u32,
                Some(Typ::bool(x)) => (*x as u8) as u32,
                Some(_) => panic!("Type mismatch for u32"),
                None => panic!("State does not exist: {}", tag),
            }
        }

        fn i16(&self, tag: &str) -> i16 {
            match self.states().get(tag) {
                Some(Typ::i16(x))  => *x,
                Some(Typ::u16(x))  => *x as i16,
                Some(Typ::i8(x))   => *x as i16,
                Some(Typ::u8(x))   => *x as i16,
                Some(Typ::f64(x))  => *x as i16,
                Some(Typ::f32(x))  => *x as i16,
                Some(Typ::i32(x))  => *x as i16,
                Some(Typ::u32(x))  => *x as i16,
                Some(Typ::i64(x))  => *x as i16,
                Some(Typ::u64(x))  => *x as i16,
                Some(Typ::bool(x)) => (*x as u8) as i16,
                Some(_) => panic!("Type mismatch for i16"),
                None  => panic!("State does not exist: {}", tag),
            }
        }

        fn u16(&self, tag: &str) -> u16 {
            match self.states().get(tag) {
                Some(Typ::u16(x))  => *x,
                Some(Typ::i16(x))  => *x as u16,
                Some(Typ::u8(x))   => *x as u16,
                Some(Typ::i8(x))   => *x as u16,
                Some(Typ::f64(x))  => *x as u16,
                Some(Typ::f32(x))  => *x as u16,
                Some(Typ::i32(x))  => *x as u16,
                Some(Typ::u32(x))  => *x as u16,
                Some(Typ::i64(x))  => *x as u16,
                Some(Typ::u64(x))  => *x as u16,
                Some(Typ::bool(x)) => (*x as u8) as u16,
                Some(_) => panic!("Type mismatch for u16"),
                None => panic!("State does not exist: {}", tag),
            }
        }

        fn i8(&self, tag: &str) -> i8 {
            match self.states().get(tag) {
                Some(Typ::i8(x))   => *x,
                Some(Typ::u8(x))   => *x as i8,
                Some(Typ::i16(x))  => *x as i8,
                Some(Typ::u16(x))  => *x as i8,
                Some(Typ::f64(x))  => *x as i8,
                Some(Typ::f32(x))  => *x as i8,
                Some(Typ::i32(x))  => *x as i8,
                Some(Typ::u32(x))  => *x as i8,
                Some(Typ::i64(x))  => *x as i8,
                Some(Typ::u64(x))  => *x as i8,
                Some(Typ::bool(x)) => (*x as u8) as i8,
                Some(_) => panic!("Type mismatch for i8"),
                None => panic!("State does not exist: {}", tag),
            }
        }

        fn u8(&self, tag: &str) -> u8 {
            match self.states().get(tag) {
                Some(Typ::u8(x))   => *x,
                Some(Typ::i8(x))   => *x as u8,
                Some(Typ::u16(x))  => *x as u8,
                Some(Typ::i16(x))  => *x as u8,
                Some(Typ::f64(x))  => *x as u8,
                Some(Typ::f32(x))  => *x as u8,
                Some(Typ::i32(x))  => *x as u8,
                Some(Typ::u32(x))  => *x as u8,
                Some(Typ::i64(x))  => *x as u8,
                Some(Typ::u64(x))  => *x as u8,
                Some(Typ::bool(x)) => *x as u8,
                Some(_) => panic!("Type mismatch for u8"),
                None => panic!("State does not exist: {}", tag),
            }
        }


        fn bool(&self, tag: &str) -> bool {
            match self.states().get(tag) {
                Some(Typ::bool(x)) => *x,
                Some(Typ::u8(x))   => *x != 0,
                Some(Typ::i8(x))   => *x != 0,
                Some(Typ::u16(x))  => *x != 0,
                Some(Typ::i16(x))  => *x != 0,
                Some(Typ::f64(x))  => *x as u8 != 0,
                Some(Typ::f32(x))  => *x as u8 != 0,
                Some(Typ::i32(x))  => *x != 0,
                Some(Typ::u32(x))  => *x != 0,
                Some(Typ::i64(x))  => *x != 0,
                Some(Typ::u64(x))  => *x != 0,
                Some(_) => panic!("Type mismatch for bool"),
                None => panic!("State does not exist: {}", tag),
            }
        }
    }

    impl<'a> TypToPrim<'a> for Agent<'a> { fn states(&self) -> &HashMap<&'a str, Typ> { &self.states } }
    impl<'a> TypToPrim<'a> for World<'a> { fn states(&self) -> &HashMap<&'a str, Typ> { &self.states } }


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

#[derive(Debug)]
pub struct World<'a> {
    // needs a lifetime
    pub states: HashMap<&'a str,Typ>
}

impl<'a> World<'a> {
    pub fn initialize() -> World<'a> {
        World {
            // just use a hashmap
            states: HashMap::new()
        }
    }
}

// ---------------------

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
// Activates the goal if the condition is false
pub enum GoalConditions {
    ne,
    e,
    ng,
    nl,
    g,
    l,
    nge,
    nle,
    ge,
    le,
}
impl GoalConditions {
    pub fn compare<T: PartialOrd>(cmp: &GoalConditions, a: T, b: T) -> bool {
        match cmp {
            GoalConditions::ge => if a >= b {return true},
            GoalConditions::le => if a <= b {return true},
            GoalConditions::g => if a > b {return true},
            GoalConditions::l => if a < b {return true},
            GoalConditions::ne => if a != b {return true},
            GoalConditions::e => if a == b {return true},
            GoalConditions::ng => if a <= b {return true},
            GoalConditions::nl => if a >= b {return true},
            GoalConditions::nge => if a < b {return true},
            GoalConditions::nle => if a > b {return true},
        }
        false
    }
}

pub enum Eval<T: Copy + ToTyp> {
    Dynamic(Box<dyn Fn(&Agent) -> T>),
    Pointer(fn(&Agent) -> T),
    Fixed(T)
}
pub trait Extract<T: Copy + ToTyp> {
    fn extract(&self, agent: &Agent, global: &World) -> T;
}


#[derive(Clone)]
pub enum CostType {
    GDynamic(Box<dyn GlobalCloneableFnF32>),
    GPointer(fn(&Agent, &World) -> f32),
    Dynamic(Box<dyn CloneableFnF32>),
    Pointer(fn(&Agent) -> f32),
    Fixed(f32)
}
clone_trait_object!(GlobalCloneableFnF32);
clone_trait_object!(CloneableFnF32);
impl Extract<f32> for CostType {
    fn extract(&self, agent: &Agent, global: &World) -> f32 {
        match self {
            CostType::Fixed(x) => x.clone(),
            CostType::Pointer(x) => x(agent),
            CostType::Dynamic(x) => x(agent),
            CostType::GPointer(x) => x(agent, global),
            CostType::GDynamic(x) => x(agent, global)
        }
    }
}

#[derive(Clone)]
pub enum DeltaType {
    GDynamic(Box<dyn GlobalCloneableFnTyp>),
    GPointer(fn(&Agent, &World) -> Typ),
    Dynamic(Box<dyn CloneableFnTyp>),
    Pointer(fn(&Agent) -> Typ),
    Fixed(Typ)
}
clone_trait_object!(GlobalCloneableFnTyp);
clone_trait_object!(CloneableFnTyp);
impl Extract<f64> for DeltaType {
    fn extract(&self, agent: &Agent, global: &World) -> f64 {
        self.extract_raw(agent, global).into_f64()
    }
}

impl DeltaType {
    fn extract_raw(&self, agent: &Agent, global: &World) -> Typ {
        match self {
            DeltaType::Fixed(x) => x.clone(),
            DeltaType::Pointer(x) => x(agent),
            DeltaType::Dynamic(x) => x(agent),
            DeltaType::GPointer(x) => x(agent, global),
            DeltaType::GDynamic(x) => x(agent, global)
        }
    }
}

// ---------------------
pub trait HasStates<'a> {
    // function is used as a substitute to a struct field
    fn states_mut(&mut self) -> &mut HashMap<&'a str, Typ>;

    fn state<T: Copy + ToTyp>(&mut self, name: &'a str, data: T) {
        self.states_mut().insert(name, data.typ());
    }
}

impl<'a> HasStates<'a> for World<'a> {
    // change it to return the struct field
    fn states_mut(&mut self) -> &mut HashMap<&'a str, Typ> {
        &mut self.states
    }
}
impl<'a> HasStates<'a> for Agent<'a> {
    // change it to return the struct field
    fn states_mut(&mut self) -> &mut HashMap<&'a str, Typ> {
        &mut self.states
    }
}

pub fn targ<T: Copy + ToTyp>(tag: &str, cmp: GoalConditions, aim: T, bias: f32) -> TargetNode {
    if let Typ::bool(_) = aim.typ() {
        TargetNode {
            tag,
            cmp,
            bias,
            val: aim.typ(),
            bool_cmp: true,
            minimize: true
        }
    } else {
        TargetNode {
            tag,
            cmp,
            bias,
            val: aim.typ(),
            bool_cmp: false,
            minimize: true
        }
    }
}
pub fn cond<T: Copy + ToTyp>(tag: &str, cmp: GoalConditions, val: T, bias: f32) -> TargetNode {
    if let Typ::bool(_) = val.typ() {
        TargetNode {
            tag,
            cmp,
            bias,
            val: val.typ(),
            bool_cmp: true,
            minimize: false
        }
    } else {
        TargetNode {
            tag,
            cmp,
            bias,
            val: val.typ(),
            bool_cmp: false,
            minimize: false
        }
    }
}

// ---------------------

#[derive(Clone)]
pub struct Agent<'a> {
    // more hashmaps
    pub goals: HashMap<&'a str,Directives<'a>>,
    pub states: HashMap<&'a str, Typ>,
    pub actions: HashMap<&'a str, ActionMeta<'a>>
}



#[derive(Debug, Clone)]
pub struct Directives<'a> {
    directives: Vec<TargetNode<'a>>,
    total_weight: f32
}
#[derive(Debug, Clone)]
pub struct TargetNode<'a> {
    tag: &'a str,
    cmp: GoalConditions,
    val: Typ,
    bias: f32,
    bool_cmp: bool,
    minimize: bool
}

pub mod action_builders {
    use crate::{Agent, CostType, DeltaType, World, GoalConditions};
    use crate::type_conversions::{ToTyp, Typ};
    #[derive(Clone)]
    pub struct ActionMeta<'a> {
        pub cost: CostType,
        pub require: Vec<Requirement<'a>>,
        pub effects: Vec<Effect<'a>>,
    }
    #[derive(Clone)]
    pub struct Requirement<'a> {
        tag: &'a str,
        cmp: GoalConditions,
        val: DeltaType,

    }
    #[derive(Clone)]
    pub struct Effect<'a> {
        pub(crate) tag: &'a str,
        pub val: DeltaType
    }



    pub fn glo_req_dyn<T: Copy + ToTyp + 'static>(tag: &str, cmp: GoalConditions, eval: fn(&Agent, &World) -> T, ) -> Requirement {
        let wrapped = move |agent: &Agent, global: &World| { eval(agent, global).typ() };
        Requirement {
            tag,
            cmp,
            val: DeltaType::GDynamic(Box::new(wrapped))
        }
    }

    pub fn glo_ef_dyn<T: Copy + ToTyp + 'static>(tag: &str, eval: fn(&Agent, &World) -> T, ) -> Effect {
        let wrapped = move |agent: &Agent, global: &World| { eval(agent, global).typ() };
        Effect {
            tag,
            val: DeltaType::GDynamic(Box::new(wrapped)),
        }
    }

    pub fn glo_cost_dyn(eval: fn(&Agent, &World) -> f32, ) -> CostType {
        CostType::GDynamic(Box::new(eval))
    }

    pub fn req_dyn<T: Copy + ToTyp + 'static>(tag: &str, cmp: GoalConditions, eval: fn(&Agent) -> T, ) -> Requirement {
        let wrapped = move |agent: &Agent| { eval(agent).typ() };
        Requirement {
            tag,
            cmp,
            val: DeltaType::Dynamic(Box::new(wrapped))
        }
    }

    pub fn ef_dyn<T: Copy + ToTyp + 'static>(tag: &str, eval: fn(&Agent) -> T, ) -> Effect {
        let wrapped = move |agent: &Agent| { eval(agent).typ() };
        Effect {
            tag,
            val: DeltaType::Dynamic(Box::new(wrapped)),
        }
    }

    pub fn cost_dyn(eval: fn(&Agent) -> f32, ) -> CostType {
        CostType::Dynamic(Box::new(eval))
    }


    pub fn glo_req_eval(tag: &str, cmp: GoalConditions, eval: fn(&Agent, &World) -> Typ, ) -> Requirement {
        Requirement {
            tag,
            cmp,
            val: DeltaType::GPointer(eval)
        }
    }

    pub fn glo_ef_eval(tag: &str, eval: fn(&Agent, &World) -> Typ, ) -> Effect {
        Effect {
            tag,
            val: DeltaType::GPointer(eval),
        }
    }

    pub fn glo_cost_eval(eval: fn(&Agent, &World) -> f32, ) -> CostType {
        CostType::GPointer(eval)
    }


    pub fn req_eval(tag: &str, cmp: GoalConditions, eval: fn(&Agent) -> Typ, ) -> Requirement {
        Requirement {
            tag,
            cmp,
            val: DeltaType::Pointer(eval)
        }
    }

    pub fn ef_eval(tag: &str, eval: fn(&Agent) -> Typ, ) -> Effect {
        Effect {
            tag,
            val: DeltaType::Pointer(eval),
        }
    }

    pub fn cost_eval(eval: fn(&Agent) -> f32, ) -> CostType {
        CostType::Pointer(eval)
    }

    pub fn req_val<T: Copy + ToTyp>(tag: &str, cmp: GoalConditions, eval: T, ) -> Requirement {
        Requirement {
            tag,
            cmp,
            val: DeltaType::Fixed(eval.typ())
        }
    }

    pub fn ef_val<T: Copy + ToTyp>(tag: &str, eval: T, ) -> Effect {
        Effect {
            tag,
            val: DeltaType::Fixed(eval.typ())
        }
    }

    pub fn cost_val(eval: f32, ) -> CostType {
        CostType::Fixed(eval)
    }

}

pub fn action_meta<'a>(cost: CostType, require: Vec<Requirement<'a>>, effects: Vec<Effect<'a>>) -> ActionMeta<'a> {
    ActionMeta {
        cost,
        require,
        effects
    }
}



impl<'a> Agent<'a> {

    pub fn initialize() -> Agent<'a> {
        Agent {
            goals: HashMap::new(),
            states: HashMap::new(),
            actions: HashMap::new()
        }
    }

    pub fn new_value(&mut self, state: &str, new: Typ) {
        *self.states.get_mut(state).unwrap() = match self.states.get(state).expect("State not found") {
            Typ::f64(_)  => Typ::f64(new.into_f64()),
            Typ::i64(_)  => Typ::i64(new.into_i64()),
            Typ::u64(_)  => Typ::u64(new.into_u64()),
            Typ::f32(_)  => Typ::f32(new.into_f32()),
            Typ::i32(_)  => Typ::i32(new.into_i32()),
            Typ::u32(_)  => Typ::u32(new.into_u32()),
            Typ::i16(_)  => Typ::i16(new.into_i16()),
            Typ::u16(_)  => Typ::u16(new.into_u16()),
            Typ::u8(_)   => Typ::u8(new.into_u8()),
            Typ::i8(_)   => Typ::i8(new.into_i8()),
            Typ::bool(_) => Typ::bool(new.into_bool()),
            _ => panic!("Unsupported target type for conversion"),
        };

    }

    pub fn simulate(&self, global: &World, action: &'a str) {
        match self.actions.get(action){
            Some(x) => {
                println!("Result of [{action}] simulation (ignoring required fields):");
                println!("+ cost for action: {}", self.actions.get(action).unwrap().cost.extract(&self, global));
                for effects in &self.actions.get(action).unwrap().effects {
                    println!("- [{}] | {} -> {}", effects.tag, self.f64(effects.tag), effects.val.extract(&self, global));
                };
            }
            None => println!("Action [{action}] does not exist")
        }
    }

    pub fn execute(&mut self, global: &World, action: &'a str) {
        let actions = &self.actions.get(action).unwrap().effects.clone();
        for effect in actions {
            self.new_value(effect.tag, effect.val.extract_raw(&self, global));
        }
    }

    pub fn action(&mut self, name: &'a str, cost: CostType, require: Vec<Requirement<'a>>, effects: Vec<Effect<'a>>) {
        self.actions.insert(name, ActionMeta {
            cost,
            require,
            effects
        });
    }

    pub fn goal(&mut self, name: &'a str,  weight: f32, directives: Vec<TargetNode<'a>>) {
        if let Some(x) = self.goals.get_mut(name) {
            x.total_weight = weight;
            x.directives.extend(directives);
        } else {
            self.goals.insert(name, Directives { directives, total_weight: weight });
        }
    }

    pub fn goal_set_weight(&mut self, name: &'a str,  weight: f32) {
        if let Some(x) = self.goals.get_mut(name) {
            x.total_weight = weight;
        } else {
            panic!["No such goal | [{name}]"]
        }
    }

    // only return discontentment with the sub goals in the main goal, not multiplying the sum by the total weight
    pub fn h_score(&self, goal: &'a str) -> f64 {
        let mut discontentment = 0.0;
        let goal = self.goals.get(goal).unwrap();
        for target in &goal.directives {
            if target.minimize {
                if !GoalConditions::compare(&target.cmp, self.f64(target.tag), target.val.into_f64()) {
                    discontentment += (self.f64(target.tag) - target.val.into_f64()).abs() * target.bias as f64;
                }
            }
        }
        discontentment
    }


    // Returns a key to the most urgent goal
    pub fn priority_goal(&self) -> &str {
        let mut priority_goal_key: (&str, f64) = ("", 0.0);
        let mut is_first = true;
        for (goal_key, goal_data) in &self.goals {
            let mut goal_score = 0.0;
            for target in &goal_data.directives {
                let target_name = target.tag;
                let (current, desired) = match self.states.get(target_name) {
                    Some(state_val) => match state_val {
                        Typ::f64(x)  => if let Typ::f64(y)  = &target.val { (*x, *y) } else { panic!("Type mismatch [{target_name}] | Goal: {goal_key}") },
                        Typ::i64(x)  => if let Typ::i64(y)  = &target.val { (*x as f64, *y as f64) } else { panic!("Type mismatch [{target_name}] | Goal: {goal_key}") },
                        Typ::u64(x)  => if let Typ::u64(y)  = &target.val { (*x as f64, *y as f64) } else { panic!("Type mismatch [{target_name}] | Goal: {goal_key}") },
                        Typ::f32(x)  => if let Typ::f32(y)  = &target.val { (*x as f64, *y as f64) } else { panic!("Type mismatch [{target_name}] | Goal: {goal_key}") },
                        Typ::i32(x)  => if let Typ::i32(y)  = &target.val { (*x as f64, *y as f64) } else { panic!("Type mismatch [{target_name}] | Goal: {goal_key}") },
                        Typ::u32(x)  => if let Typ::u32(y)  = &target.val { (*x as f64, *y as f64) } else { panic!("Type mismatch [{target_name}] | Goal: {goal_key}") },
                        Typ::i16(x)  => if let Typ::i16(y)  = &target.val { (*x as f64, *y as f64) } else { panic!("Type mismatch [{target_name}] | Goal: {goal_key}") },
                        Typ::u16(x)  => if let Typ::u16(y)  = &target.val { (*x as f64, *y as f64) } else { panic!("Type mismatch [{target_name}] | Goal: {goal_key}") },
                        Typ::i8(x)   => if let Typ::i8(y)   = &target.val { (*x as f64, *y as f64) } else { panic!("Type mismatch [{target_name}] | Goal: {goal_key}") },
                        Typ::u8(x)   => if let Typ::u8(y)   = &target.val { (*x as f64, *y as f64) } else { panic!("Type mismatch [{target_name}] | Goal: {goal_key}") },
                        Typ::bool(x) => if let Typ::bool(y) = &target.val { ((*x as i32) as f64, (*y as i32) as f64) } else { panic!("Type mismatch [{target_name}] | Goal: {goal_key}") },
                        _ => panic!("Unknown type [{target_name}] | Goal: {goal_key}"),
                    },
                    None => panic!("Missing state [{target_name}] | Goal: {goal_key}"),
                };

                // Check if it's a comparative goal req or not
                if target.minimize {
                    // if it is then we use a different scoring category
                    if target.bool_cmp {
                        match target.cmp {
                            GoalConditions::ne => if !(current != desired) { goal_score += target.bias as f64 },
                            GoalConditions::e  => if !(current == desired) { goal_score += target.bias as f64 },
                            _ => panic!("Invalid bool condition | Goal: {goal_key}"),
                        }
                    } else {
                        if !GoalConditions::compare(&target.cmp, current, desired) {
                            goal_score += (current - desired).abs() * target.bias as f64;
                        }
                    }
                } else {
                    if target.bool_cmp {
                        match target.cmp {
                            GoalConditions::ne => if current != desired { goal_score += target.bias as f64 },
                            GoalConditions::e  => if current == desired { goal_score += target.bias as f64 },
                            _ => panic!("Invalid bool condition | Goal: {goal_key}"),
                        }
                    } else {
                        if GoalConditions::compare(&target.cmp, current, desired) {
                            goal_score += target.bias as f64;
                        }
                    }
                }
            }
            let adjusted_score = goal_score / goal_data.directives.len() as f64 * goal_data.total_weight as f64;
            if adjusted_score > priority_goal_key.1 || is_first {
                priority_goal_key = (goal_key, adjusted_score);
                is_first = false;
            }
        }
        priority_goal_key.0
    }

    pub fn plan(&mut self) {
        let old_states = self.states.clone();


    }
}

