use cucumber::{cucumber, steps, before, after};

pub struct MyWorld {
    // use this for mutable context in scenarios?
    foo: String
}

impl cucumber::World for MyWorld {}
impl std::default::Default for MyWorld {
    fn default() -> MyWorld {
        // this function is called every time a new scenario is started
        MyWorld {
            foo: "a default string".to_string()
        }
    }
}

mod example_steps {
    use cucumber::steps;

    // Any type that implements cucumber::World + Default can be the world
    steps!(crate::MyWorld => {
        given "I am trying out Cucumber" |world, step| {
            world.foo = "Some string".to_string();
            // set up context here
        };
    });
}

// Declares a before handler function named `a_before_fn`
before!(a_before_fn => |scenario| {

});

// Declares an after handler function named `an_after_fn`
after!(an_after_fn => |scenario| {

});

// A setup function to be called before everything else
fn setup() {
    
}

cucumber! {
    features: "./features", // path to feature files
    world: crate::MyWorld, // the world obj, must be the same in steps
    steps: &[
        example_steps::steps // was created by steps! above
    ],
    setup: setup, // optional, called once before everything starts
    before: &[
        a_before_fn // optional, called before each scenario
    ],
    after: &[
        an_after_fn // optional, called after each scenario
    ]
}
