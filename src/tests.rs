
use crate::{state::State, App};

struct Health {
    val: u32
}

struct Name {
    val: String
}

struct Dead;

fn start(state: &mut State) {
    let player = state.create_entity();
    state.insert_component(Health {val: 5}, &player);
    state.insert_component(Name {val: "adventurer".to_owned()}, &player);

    let enemy = state.create_entity();
    state.insert_component(Health {val: 0}, &enemy);
    state.insert_component(Name {val: "goblin".to_owned()}, &enemy);
}

fn die_if_health_is_zero(state: &mut State) {
    let health_owners = state.get_owners::<Health>().to_owned();
    let name_owners = state.get_owners::<Name>().to_owned();

    for i in &health_owners {
        for j in &name_owners {
            if j != i {
                continue;
            }
            let entity = i;
            let health = state.get_component::<Health>(&entity).unwrap();
            let name = state.get_component::<Name>(&entity).unwrap();
            println!("{} has {} of health", name.val, health.val);

            if health.val <= 0 {
                println!("{} died", name.val);
                state.delete_entity(&entity);
            }
        }
    }

    state.exit()
}

#[test]
fn general() {
    App::new(&start)
    .add_system(&die_if_health_is_zero)
    .run();
}