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
    world.insert_component2(&snowflake, Name(String::from("Snowflake")), Dog);

    let some_dog = world.create_entity();
    world.insert_component(&some_dog, Dog);
}

fn add_people(world: &mut World) {
    let francis = world.create_entity();
    world.insert_component3(&francis, Name(String::from("Francis")), Person, Age(18));

    let camila = world.create_entity();
    world.insert_component4(
        &camila,
        Name(String::from("Camila")),
        Person,
        Age(12),
        IsBirthday,
    );
}

fn greet_people(world: &mut World) {
    for entity in world.query::<Person>() {
        let person = world.ref_entity(&entity);
        if let Some(name) = person.component::<Name>() {
            println!("good morning {}!", name.0);
            if let Some(age) = person.component::<Age>() {
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
        if let Some(name) = dog.component::<Name>() {
            println!("awww, hi {}!", name.0)
        } else {
            println!("this dog doesn't have a name!")
        }
    }
}

fn celebrate_birthday(world: &mut World) {
    for entity in world.query::<IsBirthday>() {
        let mut someone = world.mut_entity(&entity);
        if let Some(name) = someone.component::<Name>() {
            println!("happy birthday {}!", name.0);
            if let Some(age) = someone.mut_component::<Age>() {
                age.0 += 1;
            }
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
    scheduler.add_system(System::Uptade(celebrate_birthday));
    scheduler.add_system(System::Uptade(greet_people));
    scheduler.add_system(System::Uptade(greet_dogs));

    scheduler.start(&mut world);
    scheduler.uptade(&mut world);
}
