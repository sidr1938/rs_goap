use goap::{Agent, HasStates, World, targ, cond, CostType, Eval, action_meta, Extract};
use goap::action_builders::*;
use goap::GoalConditions::*;
use goap::type_conversions::{ToTyp, TypToPrim};

fn main() {

    // States contain information about the world or the agent, like custom variables
    let mut world: World =  World::initialize();
    world.state("GLOBAL: fruit", 100i64);

    let mut agent: Agent = Agent::initialize();

    // Initialize some states (kinda like mini variables)
    agent.state("hunger", 80i32);
    agent.state("food", 0i32);
    agent.state("tired", 0i32);


    // First goal will be the default goal

    // cond goals happen when the condition is true, they help to make targ states more appealing or not appealing to the agent
    // they are only active when calculating the priority goal, not when minimizing
    // targ goals activate when the condition is false
    agent.goal("REPLENISH", 1.0, vec![
        cond("food", l, 20i32, 2.0),
        targ("food", g, 80i32, 1.0),
        targ("hunger", l, 50i32, 0.5),
        targ("tired", l, 60i32, 1.0)
    ]);

    // Actions

    //_dyn _eval fields will be updated during planing or if the state is changed, but you can also
    // use a static value with _val, it also helps with performance [ best (_val > _eval > _dyn) worst ]
    agent.action(
        "eat",
        cost_dyn(|agent| { if agent.f64("food") < 10.0 { 20.0 } else { 10.0 } }),
        vec![
            req_val("food", g, 0i32),
            req_val("food", nle, 0i32)
        ],
        vec![
            // dynamic stuff that updates while the planner is simulating routes
            ef_eval("food", |agent| (agent.f32("food") - 5.0).typ()),
            ef_eval("hunger", |agent| (agent.f64("hunger") - 50.0).typ())
        ]);

    agent.action(
        "forage",
        cost_val(10.0),
        vec![
            req_val("tired", l, 50i32)
        ],
        vec![
            ef_eval("tired", |agent| (agent.f64("tired") + 20.0).typ()),
            ef_eval("food", |agent| (agent.f64("food") + 10.0).typ()),
            ef_eval("hunger", |agent| (agent.f64("hunger") + 5.0).typ()),
            glo_ef_eval("GLOBAL: fruit", |_agent, world| (world.f64("GLOBAL: fruit") - 5.0).typ())
        ]
    );

    agent.action(
        "sleep",
        cost_val(10.0),
        vec![
            req_val("tired", g, 30i32)
        ],
        vec![
            ef_eval("tired", |agent| (agent.f64("tired") - 30.0).typ()),
        ]
    );

    println!("{world:?}");
    println!("\nGOALS:{:?}", agent.goals);
    println!("STATES:{:?}", agent.states);
    println!("current goal: {}", agent.priority_goal());

    // agent.simulate("eat");
    // agent.simulate("sleep");
    agent.simulate(&world,"forage");

    // for i in 0..10 {
    //     println!("\ndiscontentment with current state: {}", agent.h_score("REPLENISH"));
    //     agent.simulate(&world, "forage");
    //     agent.execute(&world, "forage");
    // }

    println!("discontentment with current state: {}", agent.h_score("REPLENISH"));
}

