use retaker::{
    aoc::AOCStorage,
    scheduler::{Scheduler, System},
    world::{EntityStorage, World},
};

#[derive(Debug)]
pub struct Name(String);
pub struct Person;
pub struct Dog;
pub struct Age(u32);
pub struct IsBirthday;

fn add_dogs(world: &mut World) {
    let snowflake = world.create_entity();
    world.insert_component(&snowflake, Name(String::from("Snowflake")));
    world.insert_component(&snowflake, Dog);

    let some_dog = world.create_entity();
    world.insert_component(&some_dog, Dog);
}

fn add_people(world: &mut World) {
    let francis = world.create_entity();
    world.insert_component(&francis, Name(String::from("Francis")));
    world.insert_component(&francis, Person);
    world.insert_component(&francis, Age(18));

    let camila = world.create_entity();
    world.insert_component(&camila, Name(String::from("Camila")));
    world.insert_component(&camila, Person);
    world.insert_component(&camila, Age(12));
    world.insert_component(&camila, IsBirthday);
}

fn greet_people(world: &mut World) {
    for entity in world.query::<Person>() {
        let person = world.ref_entity(&entity);
        if let Some(name) = person.get_component::<Name>() {
            println!("good morning {}!", name.0);
            if let Some(age) = person.get_component::<Age>() {
                println!("{}, {} years old", name.0, age.0);
            }
        } else {
            println!("this person doesn't have a name!")
        }
    }
}

fn greet_dogs(world: &mut World) {
    for entity in world.query::<Dog>() {
        let dog = world.ref_entity(&entity);
        if let Some(name) = dog.get_component::<Name>() {
            println!("awww, hi {}!", name.0)
        } else {
            println!("this dog doesn't have a name!")
        }
    }
}

fn celebrate_birthday(world: &mut World) {
    for entity in world.query::<IsBirthday>() {
        let someone = world.ref_entity(&entity);
        if let Some(name) = someone.get_component::<Name>() {
            println!("happy birthday {}!", name.0)
        } else {
            println!("happy birthday to you!")
        }
    }
}

fn main() {
    let mut world = World::new(EntityStorage::AOC(AOCStorage::new()));
    let mut scheduler = Scheduler::new();

    scheduler.add_system(System::Start(add_dogs));
    scheduler.add_system(System::Start(add_people));
    scheduler.add_system(System::Uptade(greet_people));
    scheduler.add_system(System::Uptade(greet_dogs));
    scheduler.add_system(System::Uptade(celebrate_birthday));

    scheduler.start(&mut world);
    scheduler.uptade(&mut world);
}
