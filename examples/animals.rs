use retaker::{work::Work, world::LockedWorld};

#[derive(Debug)]
pub struct Legged;

#[derive(Debug)]
pub struct Armed;

#[derive(Debug)]
pub struct Named(String);

fn main() {
    let world = LockedWorld::new();

    let start_work = Work::new().add_system(|world: &LockedWorld| {
        let mut world = world.lock_exclusive();

        let armed_entity = world.create_entity();
        world.insert(&armed_entity, Armed);
        world.insert(&armed_entity, Named(String::from("the armed one")));

        let legged_entity = world.create_entity();
        world.insert(&legged_entity, Legged);
        world.insert(&legged_entity, Named(String::from("the legged one")));

        let armed_and_legged_entity = world.create_entity();
        world.insert(&armed_and_legged_entity, Legged);
        world.insert(&armed_and_legged_entity, Armed);
        world.insert(
            &armed_and_legged_entity,
            Named(String::from("the legged and armed one")),
        );
    });

    let print_work = Work::new().add_system(|world: &LockedWorld| {
        let world = world.lock_shared();

        let legged_entities = world.components::<Legged>().unwrap();
        let armed_entities = world.components::<Armed>().unwrap();
        let named_entities = world.components::<Named>().unwrap();

        for legged_id in named_entities.with(legged_entities.query()) {
            let named = named_entities.get(&legged_id).unwrap();
            println!("i am legged and i can run! i am {}", named.0);
        }

        for armed_id in named_entities.with(armed_entities.query()) {
            let named = named_entities.get(&armed_id).unwrap();
            println!("i am armed and i can grab! i am {}", named.0);
        }

        for only_legged_id in named_entities.with(legged_entities.without(armed_entities.query())) {
            let named = named_entities.get(&only_legged_id).unwrap();
            println!("i am only legged so i can only run! i am {}", named.0);
        }

        for only_armed_id in named_entities.with(armed_entities.without(legged_entities.query())) {
            let named = named_entities.get(&only_armed_id).unwrap();
            println!("i am only armed so i can only grab! i am {}", named.0);
        }

        for armed_id in named_entities.with(armed_entities.query()) {
            let named = named_entities.get(&armed_id).unwrap();
            println!("i am armed and i can grab! i am {}", named.0);
        }

        for armed_and_legged_id in named_entities.with(legged_entities.with(armed_entities.query()))
        {
            let named = named_entities.get(&armed_and_legged_id).unwrap();
            println!(
                "i am armed and i can grab! but im also legged so i can also run! i am {}",
                named.0
            );
        }
    });

    start_work.run(&world);
    print_work.run(&world);
}
