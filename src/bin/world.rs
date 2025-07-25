use goap::{Agent, HasStates, Shared, targ, cond};
use goap::action_handler::action_builders::*;
use goap::GoalConditions::*;
use goap::response_curves::{ResponseCurve};
use goap::poly_type::type_conversions::{ToTyp, TypToPrim};


fn main() {

    // States contain information about the world or the agent, like custom variables
    let mut world: Shared =  Shared::initialize();
    world.state("fruit", 5i32);
    let mut agent: Agent = Agent::initialize();
    // Initialize some states (kinda like mini variables)
    agent.state("hunger", 20i32);
    agent.state("food", 10i32);
    agent.state("tired", 20i32);


    // First goal will be the default goal
    // Part of the preset functions
    // The min value is the x value where the function reaches its minimum-
    // it is used to help reduce load in the planning phase to figure out what minimizes the function
    // however for simple functions it should be determined automatically (by you in the response curve script)


    fn linear_a<T, U>(lo: T, hi: U) -> (ResponseCurve, T, U) {
        (ResponseCurve::linear(1.0,1.0,1.0), lo, hi)
    }
    fn linear_b<T, U>(lo: T, hi: U) -> (ResponseCurve, T, U) {
        (ResponseCurve::linear(0.5,1.0,1.0), lo, hi)
    }
    fn poly_a<T, U>(lo: T, hi: U) -> (ResponseCurve, T, U) {
        (ResponseCurve::polynomial(0.0,-1.8,4.0, 0.8, 1.0), lo, hi)
    }

    fn poly_b<T, U>(hi: T, lo: U) -> (ResponseCurve, T, U) {
        (ResponseCurve::polynomial(1.0,-1.8,5.0, 1.0, 0.0), hi, lo)
    }

    // condition directives are not minimized, only used for planning the goal
    // (matters least eg: dont care, matters most eg: aim for) for the linear function

    agent.goal("FARM", 1.0, vec![
        // if fruit is alot, then we want to replenish
        targ("food", poly_b(5,30),1.0),
        targ("fruit", linear_a(50, 0), 1.0)
    ]);
    // hi: really need to do / must do / good idea
    // lo: dont need to do / should not do / bad idea

    // Actions

    // _dyn _eval fields will be updated during planing or if the state is changed, but you can also
    // use a static value with _val, it also helps with performance [ best (_val > _eval > _dyn) worst ]
    agent.action(
        "eat",
        cost_dyn(|agent| { if agent.f64("food") < 10.0 { 20.0 } else { 10.0 } }),
        vec![
            req_val("food", g, 0),
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
            req_val("tired", l, 50),
            req_val("fruit", g, 0)
        ],
        vec![
            ef_eval("tired", |agent| (agent.f64("tired") + 20.0).typ()),
            ef_eval("food", |agent| (agent.f64("food") + 10.0).typ()),
            ef_eval("hunger", |agent| (agent.f64("hunger") + 5.0).typ()),
            g_ef_eval("fruit", |_agent, world| (world.f64("fruit") - 5.0).typ())
        ]
    );

    agent.action(
        "plant",
        cost_val(10.0),
        vec![
            req_val("tired", l, 50i32)
        ],
        vec![
            ef_eval("tired", |agent| (agent.f64("tired") + 20.0).typ()),
            ef_eval("hunger", |agent| (agent.f64("hunger") + 5.0).typ()),
            g_ef_eval("fruit", |_agent, world| (world.f64("fruit") + 5.0).typ())
        ]
    );

    agent.action(
        "sleep",
        cost_val(10.0),
        vec![
            req_val("tired", g, 20.0),
        ],
        vec![
            ef_eval("tired", |agent| (agent.f64("tired") - 30.0).typ()),
        ]
    );
    // Automatically sets global and local fields
    agent.build_fields(&world);
    agent.simulate_priority_goal(&world);


    let thing = agent.clone();
}

