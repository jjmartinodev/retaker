

use retaker::{entity::DefaultEntityIdGenerator, system::Scheduler, world::World};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SystemGroups {
    SomeSystemGroup,
}

pub struct State {
    world: World,
    generator: DefaultEntityIdGenerator,
}

unsafe impl Send for State {}
unsafe impl Sync for State {}

pub struct MyComponent(String);
pub struct Exclude;

fn system(state: &mut State) {
    let entity1 = state.generator.generate();
    state.world.insert_component(&entity1, MyComponent(String::from("hi!!!")));

    let entity2 = state.generator.generate();
    state.world.insert_component(&entity2, MyComponent(String::from("bye!!!")));

    let entity3 = state.generator.generate();
    state.world.insert_component2(
        &entity3,
        (MyComponent(String::from("excluded!!!")), Exclude),
    );

    {
        let mut query = state.world.query::<MyComponent>();
        query.filter_without::<Exclude>(&state.world);
        for entity in query {
            let mut string = state.world.mut_component::<MyComponent>(&entity).unwrap();

            string.0 = string.0.to_uppercase();

            println!("{}", string.0);
        }
    }
}

fn main() {
    let mut state = State {
        world: World::new().into(),
        generator: DefaultEntityIdGenerator::new().into(),
    };
    let mut scheduler: Scheduler<SystemGroups, State> = Scheduler::new();

    scheduler.add_system(system, SystemGroups::SomeSystemGroup);

    scheduler.run_group(&mut state, SystemGroups::SomeSystemGroup);
}
