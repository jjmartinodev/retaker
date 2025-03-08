# Retaker
 Retaker is an ECS with a lot of work to be done.

 The World struct can create entities then insert components in them, it
 can also create resources.

 To query components, first a component list is locked, then you can query
 the entities of that list from the list.

 Components can be read/written when locking a component list and getting
 them from the list with an entity id.

 Because the World has to be locked by every system that creates an entity,
 it is a bit inneficient. The optimal solution would be a queue that can
 delete and create entities, then push to that queue instead of blocking
 a whole World.

 Queuing is a bit verbose as well as making a system and locking a World. Macros
 or a better structural design could alleviate this.

 Having the world locked dynamically for each system is inneficient, if i
 could get metadata of a function like in Zig, a scheduler that permits
 a lot more systems run parallel and reduce systems locking the world to
 almost none.

 All this said, this ecs is good enough for me and not very much complex.

# World Example :
```
    let mut world = World::new();

    struct Person {
        name: String,
    }
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
```

Three more examples that also feature systems are found in the examples folder