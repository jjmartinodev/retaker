use retaker::{entity::EntityIdGenerator, system::Scheduler, world::World};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SystemGroups {
    SomeSystemGroup,
}

pub struct State {
    world: World,
    generator: EntityIdGenerator,
}

#[derive(Debug)]
pub struct P {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

fn system(state: &mut State) {
    let entity1 = state.generator.generate();
    state.world.insert_component(
        &entity1,
        P {
            x: 0.0,
            y: 0.0,
            vx: 0.0,
            vy: 0.0,
        },
    );

    let entity2 = state.generator.generate();
    state.world.insert_component(
        &entity2,
        P {
            x: 100.0,
            y: 100.0,
            vx: 0.0,
            vy: 0.0,
        },
    );

    let query = state.world.query::<P>();
    for a in query.clone() {
        for b in query.clone() {
            if a == b {
                continue;
            }
            let mut comps = state.world.many_mut_component::<P, 2>([a, b]).unwrap();
            let [a, b] = comps.get();

            let dx = a.x - b.x;
            let dy = a.y - b.y;

            b.vx += dx;
            b.vy += dy;

            a.vx -= dx;
            a.vy -= dy;
        }
    }

    for a in query.clone() {
        println!("{:?}", *state.world.component::<P>(&a).unwrap())
    }
}

fn main() {
    let mut state = State {
        world: World::new(),
        generator: EntityIdGenerator::new(),
    };
    let mut scheduler: Scheduler<SystemGroups, State> = Scheduler::new();

    scheduler.add_system(system, SystemGroups::SomeSystemGroup);

    scheduler.run_group(&mut state, SystemGroups::SomeSystemGroup);
}
