# Retaker
 Retaker is an ECS(very wip) that aims for simplicity and performance.

# Example :
```
use crate::{App, state::State};

struct Health { pub val: i32 }
struct Name { pub val: String }

fn start_fn(state: &mut State) {
    let player = state.create_entity();
    state.add_component::<Health>(player, Health {val: 3});
    state.add_component::<Name>(player, Name {val: "adventurer".to_owned()});
}

fn uptade_fn(state: &mut State) {
    let health_owners = state.get_owners::<Health>();
    let name_owners = state.get_owners::<Name>();
    println!("{:?}",health_owners);
    println!("{:?}",name_owners);
    for health_owner in health_owners {
        for name_owner in name_owners {
            if health_owner != name_owner {
                continue;
            }
            let health = state.get_component::<Health>(*health_owner);
            let name = state.get_component::<Name>(*name_owner);

            println!("{} has {} health",name.val,health.val);
        }
    }
    state.exit();
}

fn main() {
    App::new(&start_fn)
    .add_system(&uptade_fn)
    .run();
}
```
