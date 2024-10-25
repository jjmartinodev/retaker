use retaker::{
    aoc::AOCStorage,
    scheduler::{Scheduler, System},
    world::{EntityStorage, World},
};

#[derive(Debug)]
pub struct Name(String);
pub struct Person;
pub struct Dog;

fn add_dogs(world: &mut World) {
    let snowflake = world.create_entity();
    world.insert_component(&snowflake, Name(String::from("Snowflake")));
    world.insert_component(&snowflake, Dog);

    let canela = world.create_entity();
    world.insert_component(&canela, Name(String::from("Canela")));
    world.insert_component(&canela, Dog);
}

fn add_people(world: &mut World) {
    let francis = world.create_entity();
    world.insert_component(&francis, Name(String::from("Francis")));
    world.insert_component(&francis, Person);

    let camila = world.create_entity();
    world.insert_component(&camila, Name(String::from("Camila")));
    world.insert_component(&camila, Person);
}

fn greet_people(world: &mut World) {
    for person in world.query::<Person>() {
        let name = world
            .get_component::<Name>(&person)
            .expect("this person doesn't have a name!");
        println!("good morning {}!", name.0)
    }
}

fn greet_dogs(world: &mut World) {
    for dog in world.query::<Person>() {
        let name = world
            .get_component::<Name>(&dog)
            .expect("this dog doesn't have a name!");
        println!("awww, hi {}!", name.0)
    }
}

fn main() {
    let mut world = World::new(EntityStorage::AOC(AOCStorage::new()));
    let mut scheduler = Scheduler::new();

    scheduler.add_system(System::Start(add_dogs));
    scheduler.add_system(System::Start(add_people));
    scheduler.add_system(System::Uptade(greet_people));
    scheduler.add_system(System::Uptade(greet_dogs));

    scheduler.start(&mut world);
    scheduler.uptade(&mut world);
}
