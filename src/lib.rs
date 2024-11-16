pub mod component;
pub mod entity;
pub mod system;
pub mod world;

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::component::ManyComponentMut;

    #[test]
    fn stress() {
        use crate::{entity::EntityIdGenerator, system::Scheduler, world::World};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum SystemGroups {
            Start,
            Tick,
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

        const POINT_COUNT: usize = 10;
        const ITER_COUNT: usize = 100;

        fn start(state: &mut State) {
            for x in 0..POINT_COUNT {
                for y in 0..POINT_COUNT {
                    let entity = state.generator.generate();
                    state.world.insert_component(
                        &entity,
                        P {
                            x: x as f32,
                            y: y as f32,
                            vx: 0.0,
                            vy: 0.0,
                        },
                    );
                }
            }
        }

        fn tick(state: &mut State) {
            let query = state.world.query::<P>();
            // increments speed by about 20% overall
            let mut reusable_guard = Some(state.world.mut_component_list::<P>().unwrap());
            for ae in query.clone() {
                for be in query.clone() {
                    if ae == be {
                        continue;
                    }
                    let mut comps: ManyComponentMut<'_, P, 2> =
                        ManyComponentMut::new(reusable_guard.take().unwrap(), [ae, be]);
                    let [a, b] = comps.get();

                    let dx = a.x - b.x;
                    let dy = a.y - b.y;

                    a.vx -= dx;
                    a.vy -= dy;

                    reusable_guard = Some(comps.drop());
                }
            }
        }

        fn not_ecs() {
            let start_init = Instant::now();
            let mut points = vec![];
            for x in 0..POINT_COUNT {
                for y in 0..POINT_COUNT {
                    points.push(P {
                        x: x as f32,
                        y: y as f32,
                        vx: 0.0,
                        vy: 0.0,
                    });
                }
            }
            println!("start took {:?}", start_init.elapsed().as_micros());
            let iteration_init = Instant::now();
            for _ in 0..ITER_COUNT {
                for i in 0..points.len() {
                    for j in 0..points.len() {
                        if i == j {
                            continue;
                        }

                        let a = &points[i];
                        let b = &points[j];

                        let dx = a.x - b.x;
                        let dy = a.y - b.y;

                        points[i].vx -= dx;
                        points[i].vy -= dy;
                    }
                }
            }
            println!("iteration took {:?}", iteration_init.elapsed().as_micros());
        }
        fn ecs() {
            let mut state = State {
                world: World::new(),
                generator: EntityIdGenerator::new(),
            };
            let mut scheduler: Scheduler<SystemGroups, State> = Scheduler::new();

            scheduler.add_system(start, SystemGroups::Start);
            scheduler.add_system(tick, SystemGroups::Tick);
            let start_init = Instant::now();
            scheduler.run_group(&mut state, SystemGroups::Start);
            println!("start took {:?}", start_init.elapsed().as_micros());
            let iteration_init = Instant::now();
            for _ in 0..ITER_COUNT {
                scheduler.run_group(&mut state, SystemGroups::Tick);
            }
            println!("iteration took {:?}", iteration_init.elapsed().as_micros());
        }

        let start_non_ecs = Instant::now();
        not_ecs();
        let non_ecs_time = start_non_ecs.elapsed().as_micros();
        println!("non ecs took {:?}", non_ecs_time);
        let start_ecs = Instant::now();
        ecs();
        let ecs_time = start_ecs.elapsed().as_micros();
        println!("ecs took {:?}", ecs_time);
        
        println!("diff is {:?}x", ecs_time / non_ecs_time);
    }
}
