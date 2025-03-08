use retaker::world::World;

fn main() {
    let mut world = World::new();

    #[derive(Debug)]
    struct Person {
        name: String,
    }
    #[derive(Debug)]
    struct Dead;

    let mathias = world.create_entity();
    world.insert(
        &mathias,
        Person {
            name: "Mathias".to_string(),
        },
    );
    world.insert(&mathias, Dead);

    let thomas = world.create_entity();
    world.insert(
        &thomas,
        Person {
            name: "Thomas".to_string(),
        },
    );

    let dead_components = world.components::<Dead>().unwrap();
    let person_components = world.components::<Person>().unwrap();

    for dead_person_id in person_components.with(dead_components.query()) {
        let person = person_components.get(&dead_person_id).unwrap();
        println!("{} is dead", person.name);
    }

    for alive_person_id in dead_components.without(person_components.query()) {
        let person = person_components.get(&alive_person_id).unwrap();
        println!("{} is alive", person.name);
    }
}
