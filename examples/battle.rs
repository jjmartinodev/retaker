use retaker::{work::Work, world::LockedWorld};

pub struct Health(i32);
pub struct Attaker {
    damage: u32,
}
#[derive(PartialEq, Eq, Debug)]
pub enum Faction {
    Allies,
    Enemies,
}
pub struct Dead;
pub struct Battle {
    on_going: bool,
}

fn create_battle(world: &LockedWorld) {
    let mut world = world.lock_exclusive();
    _ = world.create_resource(Battle { on_going: true });
}

fn create_player(world: &LockedWorld) {
    let mut world = world.lock_exclusive();
    let player = world.create_entity();
    world.insert(&player, Health(3));
    world.insert(&player, Attaker { damage: 1 });
    world.insert(&player, Faction::Allies);
}

fn create_enemy(world: &LockedWorld) {
    let mut world = world.lock_exclusive();
    let enemy = world.create_entity();
    world.insert(&enemy, Health(2));
    world.insert(&enemy, Attaker { damage: 1 });
    world.insert(&enemy, Faction::Enemies);
}

fn tick_attacks(world: &LockedWorld) {
    let world = world.lock_shared();

    let battle_state = world.resource::<Battle>().unwrap();
    if !battle_state.on_going {
        return;
    }

    let attaker_comps = world.components::<Attaker>().unwrap();
    let mut health_comps = world.components_mut::<Health>().unwrap();
    let faction_comps = world.components::<Faction>().unwrap();

    let attakers = attaker_comps.with(faction_comps.query());
    let attackable = health_comps.with(faction_comps.query());

    for attacker_id in attakers {
        let attacker_faction = faction_comps.get(&attacker_id).unwrap();
        let attacker_comp = attaker_comps.get(&attacker_id).unwrap();
        for attackable_id in attackable.clone() {
            let attackable_health = health_comps.get_mut(&attackable_id).unwrap();
            let attackable_faction = faction_comps.get(&attackable_id).unwrap();
            if attackable_id == attacker_id {
                continue;
            }

            if *attackable_faction != *attacker_faction {
                attackable_health.0 -= attacker_comp.damage as i32;
                println!(
                    "{:?} attacked {:?} and inflicted {}, {} health remaining on {:?} entity",
                    *attacker_faction,
                    *attackable_faction,
                    attacker_comp.damage,
                    attackable_health.0,
                    *attackable_faction,
                );
                continue;
            }
        }
    }
}

fn check_battle(world: &LockedWorld) {
    let world = world.lock_shared();
    let mut battle = world.resource_mut::<Battle>().unwrap();
    if battle.on_going {
        let factions = world.components::<Faction>().unwrap();
        let reference_point = factions.query().into_iter().last();
        if let Some(reference_point) = reference_point {
            for faction in factions.query() {
                if reference_point != faction {
                    return;
                }
            }
            battle.on_going = false;
            println!("battle finished");
        }
    }
}

fn print_dead(world: &LockedWorld) {
    let world = world.lock_shared();
    let healths = world.components::<Health>().unwrap();
    let factions = world.components::<Faction>().unwrap();
    for printable_id in factions.with(healths.query()) {
        let faction = factions.get(&printable_id).unwrap();
        let health = healths.get(&printable_id).unwrap();
        if health.0 <= 0 {
            println!("{:?} died", *faction);
        }
    }
}

fn clear_dead(world: &LockedWorld) {
    let mut world = world.lock_exclusive();
    let health_comps = world.components_mut::<Health>().unwrap();
    let mut queued_deletes = vec![];
    for health_id in health_comps.query() {
        let health = health_comps.get(&health_id).unwrap();
        if health.0 <= 0 {
            queued_deletes.push(health_id.clone());
        }
    }
    drop(health_comps);
    for queued in queued_deletes {
        world.delete_entity(&queued);
    }
}

fn main() {
    let world = LockedWorld::new();

    let start_work = Work::new()
        .add_system(create_battle)
        .add_system(create_player)
        .add_system(create_enemy);

    let update_work = Work::new()
        .add_system(check_battle)
        .add_system(tick_attacks)
        .add_system(print_dead)
        .add_system(clear_dead);

    start_work.run(&world);

    loop {
        update_work.run(&world);
        let world = world.lock_shared();
        let battle = world.resource::<Battle>().unwrap();
        if !battle.on_going {
            break;
        }
    }
}
