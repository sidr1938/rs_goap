pub mod response_curves;
pub mod poly_type;
pub mod action_handler;

use response_curves::ResponseCurve;
// implement multithreading in the future
use rayon::prelude::*;
use std::collections::HashMap;
use crate::action_handler::action_builders::*;
use crate::action_handler::*;
use crate::poly_type::type_conversions::*;



#[derive(Debug)]
pub struct Shared<'a> {
    // needs a lifetime
    pub states: HashMap<&'a str, Data>
}

impl<'a> Shared<'a> {
    pub fn initialize() -> Shared<'a> {
        Shared {
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





// ---------------------
pub trait HasStates<'a> {
    // function is used as a substitute to a struct field
    fn states_mut(&mut self) -> &mut HashMap<&'a str, Data>;

    fn state<T: Copy + ToTyp>(&mut self, name: &'a str, data: T) {
        self.states_mut().insert(name, data_build(data.typ()));
        ;
    }
}


impl<'a> HasStates<'a> for Shared<'a> {
    // change it to return the struct field
    fn states_mut(&mut self) -> &mut HashMap<&'a str, Data> {
        &mut self.states
    }
}
impl<'a> HasStates<'a> for Agent<'a> {
    // change it to return the struct field
    fn states_mut(&mut self) -> &mut HashMap<&'a str, Data> {
        &mut self.states
    }
}

pub fn new_targ_node(tag: &str, bias: f32, lower: Typ, upper: Typ, varient: DirectiveType, minimize: bool, curve: ResponseCurve) -> TargetNode {
    let l = lower.into_f64();
    if minimize {
        println!("tag: {} min: {}", tag, l + (upper.into_f64() - l) * curve.min);
    }
    TargetNode {
        tag,
        bias,
        upper: upper.clone(),
        lower,
        varient,
        minimize,
        local: true,
        curve,
        min: l + (upper.into_f64() - l) * curve.min,
    }
}

#[derive(Clone, Debug)]
enum DirectiveType {
    Equality,
    Normal,
}

pub fn targ<T: Copy + ToTyp, U: Copy + ToTyp>(tag: &str, curve: (ResponseCurve, T, U), bias: f32) -> TargetNode {
    if let Typ::bool(_) = curve.2.typ() {
        // val equals / not equals
        new_targ_node(tag, bias, curve.1.typ(), curve.2.typ(), DirectiveType::Equality,true, curve.0)
    } else {
        new_targ_node(tag, bias, curve.1.typ(), curve.2.typ(), DirectiveType::Normal,true, curve.0)
    }
}
pub fn cond<T: Copy + ToTyp, U: Copy + ToTyp>(tag: &str, curve: (ResponseCurve, T, U), bias: f32) -> TargetNode {
    if let Typ::bool(_) = curve.2.typ() {
        // val equals / not equals
        new_targ_node(tag, bias, curve.1.typ(), curve.2.typ(), DirectiveType::Equality,false, curve.0)
    } else {
        new_targ_node(tag, bias, curve.1.typ(), curve.2.typ(), DirectiveType::Normal,false, curve.0)
    }
}


// ---------------------

#[derive(Clone)]
pub struct Agent<'a> {
    // more hashmaps
    pub goals: HashMap<&'a str,Directives<'a>>,
    pub states: HashMap<&'a str, Data>,
    pub actions: HashMap<&'a str, ActionMeta<'a>>
}


#[derive(Debug)]
#[derive(Clone)]
pub struct Data {
    val: Typ,
    global: bool
}



#[derive(Debug, Clone)]
pub struct Directives<'a> {
    primary: Vec<TargetNode<'a>>,
    secondary: Vec<TargetNode<'a>>,
    total_weight: f32
}
#[derive(Debug, Clone)]
pub struct TargetNode<'a> {
    tag: &'a str,
    lower: Typ,
    upper: Typ,
    bias: f32,
    varient: DirectiveType,
    minimize: bool,
    pub local: bool,
    curve: ResponseCurve,
    min: f64
}


pub fn data_build(val: Typ) -> Data {
    Data {
        val,
        global: false
    }
}


fn calculate_target_discontent(agent: &Agent, target: &TargetNode, mod_factor: f64, global: &Shared) -> f64 {
    let current = agent.locality_check(target.tag, target.local, &global).unwrap().val.into_f64();
    let lower = target.lower.into_f64();
    // Check if it's a comparative goal req or not
    match target.varient {
        DirectiveType::Normal => {
            let score = ((current - lower) / (target.upper.into_f64() - lower)).clamp(0.0,1.0);
            target.curve.output(score + ((1.0 - score) * mod_factor * score)) * target.bias as f64
        },
        DirectiveType::Equality => {
            // if upper is true then we want the current state to match the lower value
            if target.upper.into_bool() {
                // provide 1.0 if it is not matching it
                target.curve.output((!(current == lower) as u8) as f64) * target.bias as f64
            } else {
                // provide 1.0 if it is matching it
                target.curve.output(((current == lower) as u8) as f64) * target.bias as f64
            }
        }
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

    // set the state to a different value
    pub fn new_value(&mut self, state: &str, local: bool, new: Typ, global: &mut Shared) {
        if local {
            *self.states.get_mut(state).unwrap() = data_build(match self.states.get(state).expect("State not found").val {
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
            });
        } else {
            *global.states.get_mut(state).unwrap() = data_build(match global.states.get(state).expect("State not found").val {
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
            });
        }


    }

    pub fn simulate(&self, action: &'a str, global: &Shared, ) {
        match self.actions.get(action){
            Some(x) => {
                println!("Result of [{action}] simulation (ignoring required fields):");
                println!("+ cost for action: {}", self.actions.get(action).unwrap().cost.extract(&self, global));
                for effects in &self.actions.get(action).unwrap().effects {
                    if effects.local {
                        println!("- [{}] | {} -> {}", effects.tag, self.f64(effects.tag), effects.val.extract(&self, global));
                    } else {
                        println!("- G [{}] | {} -> {}", effects.tag, global.f64(effects.tag), effects.val.extract(&self, global));

                    }
                };
            }
            None => println!("Action [{action}] does not exist")
        }
    }

    pub fn execute(&mut self, action: &'a str, global: &mut Shared) {
        let actions = self.actions.get(action).unwrap().effects.clone();
        for effect in actions {
            self.new_value(effect.tag, effect.local, effect.val.extract_raw(&self, global), global);
        }
    }

    pub fn check_action(&self, action: &'a str, global: &Shared) -> bool {
        for require in &self.actions.get(action).unwrap().require {
            if !compare(&require.cmp,
                    self.locality_check(require.tag, require.local, &global).unwrap().val.into_f64(),
                    require.val.extract(&self, &global)) {
                return false;
            }
        }
        true
    }

    pub fn action(&mut self, name: &'a str, cost: CostType, require: Vec<Requirement<'a>>, effects: Vec<Effect<'a>>) {
        self.actions.insert(name, ActionMeta {
            cost,
            require,
            effects
        });
    }

    pub fn goal(&mut self, name: &'a str,  weight: f32, directives: Vec<TargetNode<'a>>) {
        if let None = self.goals.get_mut(name){
            self.goals.insert(name, Directives { primary: Vec::new(), secondary: Vec::new(), total_weight: 0.0 });
        }

        let my_goal = self.goals.get_mut(name).unwrap();

        my_goal.total_weight = weight;

        for mut node in directives {
            node.local = self.states.contains_key(node.tag);
            if node.minimize {
                my_goal.primary.push(node)
            } else {
                my_goal.secondary.push(node)
            }
        }
    }

    pub fn goal_weight(&mut self, name: &'a str,  weight: f32) {
        if let Some(x) = self.goals.get_mut(name) {
            x.total_weight = weight;
        } else {
            panic!["No such goal | [{name}]"]
        }
    }

    // only return discontentment with the sub goals in the main goal, not multiplying the sum by the total weight
    pub fn h_score(&self, goal: &'a str, global: &Shared) -> f64 {
        let goal_data = self.goals.get(goal).unwrap();
        let mod_factor = 1.0 - (1.0 / goal_data.primary.len() as f64);
        let goal_score: f64 = goal_data.primary.iter()
            .map(|target| calculate_target_discontent(self, target, mod_factor, global))
            .sum();
        goal_score / goal_data.primary.len() as f64
    }

    pub fn build_fields(&mut self, global: &Shared) {
        for i in &mut self.actions {
            for require in &mut i.1.require {
                require.local = self.states.contains_key(require.tag);
            }
            for effect in &mut i.1.effects {
                effect.local = self.states.contains_key(effect.tag);
            }
        }
    }

    pub fn locality_check<'b>(&'b self, tag: &str, local: bool, global: &'b Shared<'b>) -> Option<&'b Data>{
        if local { self.states.get(tag) } else {  global.states.get(tag)}
    }

    // Returns a key to the most urgent goal
    pub fn priority_goal(&self, global: &Shared) -> &str {
        let mut priority_goal_key: (&str, f64) = ("", 0.0);
        for (goal_key, goal_data) in &self.goals {
            let mut goal_score = 1.0;
            let mod_factor = 1.0 - (1.0 / (goal_data.primary.len() + goal_data.secondary.len()) as f64);
            for target in &goal_data.primary {
                if goal_score == 0.0 { break; }
                goal_score *= calculate_target_discontent(self, target, mod_factor, global);
            }
            for target in &goal_data.secondary {
                if goal_score == 0.0 { break; }
                goal_score *= calculate_target_discontent(self, target, mod_factor, global);
            }

            let adjusted_score = goal_score * goal_data.total_weight as f64;
            if adjusted_score > priority_goal_key.1 {
                priority_goal_key = (goal_key, adjusted_score);
            }
        }
        priority_goal_key.0
    }

    pub fn simulate_priority_goal(&self, global: &Shared) {
        println!("Result of goal weighting simulation:\n");
        let mut priority_goal_key: (&str, f64) = ("", 0.0);
        let mut is_first = true;
        for (goal_key, goal_data) in &self.goals {
            println!("--- Examining: [{goal_key}] (w/ modifier) ---");
            let mut goal_score = 1.0;
            let mod_factor = 1.0 - (1.0 / (goal_data.primary.len() + goal_data.secondary.len()) as f64);
            for target in &goal_data.primary {
                let current = self.locality_check(target.tag, target.local, &global).unwrap().val.into_f64();
                let lower = target.lower.into_f64();
                // Check if it's a comparative goal req or not
                match target.varient {
                    DirectiveType::Normal => {
                        let score = ((current - lower) / (target.upper.into_f64() - lower)).clamp(0.0,1.0);
                        let k = target.curve.output(score + ((1.0 - score) * mod_factor * score)) * target.bias as f64;
                        goal_score *= k;
                        println!("- Target | [{}] scored as: {} at {}%", target.tag, k, score + ((1.0 - score) * mod_factor * score));
                    },
                    DirectiveType::Equality => {
                        // if upper is true then we want the current state to match the lower value
                        if target.upper.into_bool() {
                            // provide 1.0 if it is not matching it
                            let k = target.curve.output((!(current == lower) as u8) as f64) * target.bias as f64;
                            goal_score *= k;
                            println!("- Target (must equal) | [{}] scored as: {} at {}%", target.tag, k, !(current == lower));
                        } else {
                            // provide 1.0 if it is matching it
                            let k = target.curve.output(((current == lower) as u8) as f64) * target.bias as f64;
                            goal_score *= k;
                            println!("- Target (must not equal) | [{}] scored as: {} at {}%", target.tag, k, (current == lower));
                        }
                    }
                }
            }

            for target in &goal_data.secondary {
                let current = self.locality_check(target.tag, target.local, &global).unwrap().val.into_f64();
                let lower = target.lower.into_f64();
                // Check if it's a comparative goal req or not
                match target.varient {
                    DirectiveType::Normal => {
                        let score = ((current - lower) / (target.upper.into_f64() - lower)).clamp(0.0,1.0);
                        let k = target.curve.output(score + ((1.0 - score) * mod_factor * score)) * target.bias as f64;
                        goal_score *= k;
                        println!("- Condition | [{}] scored as: {} at {}%", target.tag, k, score + ((1.0 - score) * mod_factor * score));
                    },
                    DirectiveType::Equality => {
                        // if upper is true then we want the current state to match the lower value
                        if target.upper.into_bool() {
                            // provide 1.0 if it is not matching it
                            let k = target.curve.output((!(current == lower) as u8) as f64) * target.bias as f64;
                            goal_score *= k;
                            println!("- Condition (must equal) | [{}] scored as: {} at {}%", target.tag, k, !(current == lower));
                        } else {
                            // provide 1.0 if it is matching it
                            let k = target.curve.output(((current == lower) as u8) as f64) * target.bias as f64;
                            goal_score *= k;
                            println!("- Condition (must not equal) | [{}] scored as: {} at {}%", target.tag, k, (current == lower));
                        }
                    }
                }
            }
            let adjusted_score = goal_score * goal_data.total_weight as f64;
            println!("+ Final score for [{}] is: {}\n", goal_key, adjusted_score);
            if adjusted_score > priority_goal_key.1 || is_first {
                if !is_first {
                    println!("* Priority goal swap: [{}] -> [{goal_key}]\n", priority_goal_key.0);
                } else {
                    println!("* Default goal set: [{goal_key}] *\n");
                }

                priority_goal_key = (goal_key, adjusted_score);
                is_first = false;
            }
        }
        println!("--- END - Priority Goal: [{}] ---", priority_goal_key.0);
    }

    pub fn plan(&mut self) {
        let old_states = self.states.clone();


    }
}

