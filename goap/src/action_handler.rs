use crate::action_handler::action_builders::{ActionMeta, Effect, Requirement};
use crate::{Agent, Shared};

use dyn_clone::DynClone;
use dyn_clone::clone_trait_object;
use crate::poly_type::type_conversions::{ToTyp, Typ};

pub trait GlobalCloneableFnF32: Fn(&Agent, &Shared) -> f32 + DynClone {}
impl<T> GlobalCloneableFnF32 for T where T: Fn(&Agent, &Shared) -> f32 + Clone {}
pub trait GlobalCloneableFnTyp: Fn(&Agent, &Shared) -> Typ + DynClone {}
impl<T> GlobalCloneableFnTyp for T where T: Fn(&Agent, &Shared) -> Typ + Clone {}

pub trait CloneableFnF32: Fn(&Agent) -> f32 + DynClone {}
impl<T> CloneableFnF32 for T where T: Fn(&Agent) -> f32 + Clone {}
pub trait CloneableFnTyp: Fn(&Agent) -> Typ + DynClone {}
impl<T> CloneableFnTyp for T where T: Fn(&Agent) -> Typ + Clone {}

pub trait Extract<T: Copy + ToTyp> {
    fn extract(&self, agent: &Agent, global: &Shared) -> T;
}

pub enum Eval<T: Copy + ToTyp> {
    Dynamic(Box<dyn Fn(&Agent) -> T>),
    Pointer(fn(&Agent) -> T),
    Fixed(T)
}

#[derive(Clone)]
pub enum CostType {
    GDynamic(Box<dyn GlobalCloneableFnF32>),
    GPointer(fn(&Agent, &Shared) -> f32),
    Dynamic(Box<dyn CloneableFnF32>),
    Pointer(fn(&Agent) -> f32),
    Fixed(f32)
}
#[derive(Clone)]
pub enum DeltaType {
    GDynamic(Box<dyn GlobalCloneableFnTyp>),
    GPointer(fn(&Agent, &Shared) -> Typ),
    Dynamic(Box<dyn CloneableFnTyp>),
    Pointer(fn(&Agent) -> Typ),
    Fixed(Typ)
}

impl DeltaType {
    pub(crate) fn extract_raw(&self, agent: &Agent, global: &Shared) -> Typ {
        match self {
            DeltaType::Fixed(x) => x.clone(),
            DeltaType::Pointer(x) => x(agent),
            DeltaType::Dynamic(x) => x(agent),
            DeltaType::GPointer(x) => x(agent, global),
            DeltaType::GDynamic(x) => x(agent, global)
        }
    }
}

impl Extract<f64> for DeltaType {
    fn extract(&self, agent: &Agent, global: &Shared) -> f64 {
        self.extract_raw(agent, global).into_f64()
    }
}


impl Extract<f32> for CostType {
    fn extract(&self, agent: &Agent, global: &Shared) -> f32 {
        match self {
            CostType::Fixed(x) => x.clone(),
            CostType::Pointer(x) => x(agent),
            CostType::Dynamic(x) => x(agent),
            CostType::GPointer(x) => x(agent, global),
            CostType::GDynamic(x) => x(agent, global)
        }
    }
}


clone_trait_object!(GlobalCloneableFnF32);
clone_trait_object!(CloneableFnF32);

clone_trait_object!(GlobalCloneableFnTyp);
clone_trait_object!(CloneableFnTyp);

pub mod action_builders {
    use crate::{Agent, CostType, DeltaType, Shared, GoalConditions};
    use crate::poly_type::type_conversions::{ToTyp, Typ};
    #[derive(Clone)]
    pub struct ActionMeta<'a> {
        pub cost: CostType,
        pub require: Vec<Requirement<'a>>,
        pub effects: Vec<Effect<'a>>,
    }
    #[derive(Clone)]
    pub struct Requirement<'a> {
        pub tag: &'a str,
        pub cmp: GoalConditions,
        pub val: DeltaType,
        pub local: bool

    }
    #[derive(Clone)]
    pub struct Effect<'a> {
        pub(crate) tag: &'a str,
        pub val: DeltaType,
        pub local: bool
    }



    pub fn g_req_dyn<T: Copy + ToTyp + 'static>(tag: &str, cmp: GoalConditions, eval: fn(&Agent, &Shared) -> T, ) -> Requirement {
        let wrapped = move |agent: &Agent, global: &Shared| { eval(agent, global).typ() };
        Requirement {
            tag,
            cmp,
            val: DeltaType::GDynamic(Box::new(wrapped)),
            local: false
        }
    }

    pub fn g_ef_dyn<T: Copy + ToTyp + 'static>(tag: &str, eval: fn(&Agent, &Shared) -> T, ) -> Effect {
        let wrapped = move |agent: &Agent, global: &Shared| { eval(agent, global).typ() };
        Effect {
            tag,
            val: DeltaType::GDynamic(Box::new(wrapped)),
            local: false
        }
    }

    pub fn g_cost_dyn(eval: fn(&Agent, &Shared) -> f32, ) -> CostType {
        CostType::GDynamic(Box::new(eval))
    }

    pub fn req_dyn<T: Copy + ToTyp + 'static>(tag: &str, cmp: GoalConditions, eval: fn(&Agent) -> T, ) -> Requirement {
        let wrapped = move |agent: &Agent| { eval(agent).typ() };
        Requirement {
            tag,
            cmp,
            val: DeltaType::Dynamic(Box::new(wrapped)),
            local: false
        }
    }

    pub fn ef_dyn<T: Copy + ToTyp + 'static>(tag: &str, eval: fn(&Agent) -> T, ) -> Effect {
        let wrapped = move |agent: &Agent| { eval(agent).typ() };
        Effect {
            tag,
            val: DeltaType::Dynamic(Box::new(wrapped)),
            local: false
        }
    }

    pub fn cost_dyn(eval: fn(&Agent) -> f32, ) -> CostType {
        CostType::Dynamic(Box::new(eval))
    }


    pub fn g_req_eval(tag: &str, cmp: GoalConditions, eval: fn(&Agent, &Shared) -> Typ, ) -> Requirement {
        Requirement {
            tag,
            cmp,
            val: DeltaType::GPointer(eval),
            local: false
        }
    }

    pub fn g_ef_eval(tag: &str, eval: fn(&Agent, &Shared) -> Typ, ) -> Effect {
        Effect {
            tag,
            val: DeltaType::GPointer(eval),
            local: false
        }
    }

    pub fn g_cost_eval(eval: fn(&Agent, &Shared) -> f32, ) -> CostType {
        CostType::GPointer(eval)
    }


    pub fn req_eval(tag: &str, cmp: GoalConditions, eval: fn(&Agent) -> Typ, ) -> Requirement {
        Requirement {
            tag,
            cmp,
            val: DeltaType::Pointer(eval),
            local: false
        }
    }

    pub fn ef_eval(tag: &str, eval: fn(&Agent) -> Typ, ) -> Effect {
        Effect {
            tag,
            val: DeltaType::Pointer(eval),
            local: false
        }
    }

    pub fn cost_eval(eval: fn(&Agent) -> f32, ) -> CostType {
        CostType::Pointer(eval)
    }

    pub fn req_val<T: Copy + ToTyp>(tag: &str, cmp: GoalConditions, eval: T, ) -> Requirement {
        Requirement {
            tag,
            cmp,
            val: DeltaType::Fixed(eval.typ()),
            local: false
        }
    }

    pub fn ef_val<T: Copy + ToTyp>(tag: &str, eval: T, ) -> Effect {
        Effect {
            tag,
            val: DeltaType::Fixed(eval.typ()),
            local: false
        }
    }


    pub fn cost_val(eval: f32) -> CostType {
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