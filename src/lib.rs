pub mod component;
pub mod entity;
pub mod system;
pub mod world;

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use parking_lot::{Mutex, RwLock};

    use crate::component::ManyComponentMut;

    #[test]
    fn stress() {
        use crate::{entity::DefaultEntityIdGenerator, system::PooledScheduler, world::World};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum SystemGroups {
            Start,
            Tick,
        }

        pub struct State {
            world: RwLock<World>,
            generator: Mutex<DefaultEntityIdGenerator>,
        }

        unsafe impl Send for State {}
        unsafe impl Sync for State {}

        #[derive(Debug)]
        pub struct Point {
            x: f32,
            y: f32,
            vx: f32,
            vy: f32,
        }
        #[derive(Debug)]
        pub struct Circle {
            x: f32,
            y: f32,
            r: f32,
        }

        const POINT_COUNT: usize = 10;
        const ITER_COUNT: usize = 200;

        fn start_points(state: &State) {
            let mut generator = state.generator.lock();
            let mut world = state.world.write();
            for x in 0..POINT_COUNT {
                for y in 0..POINT_COUNT {
                    let entity = generator.generate();
                    world.insert_component(
                        &entity,
                        Point {
                            x: x as f32,
                            y: y as f32,
                            vx: 0.0,
                            vy: 0.0,
                        },
                    );
                }
            }
        }

        fn start_circles(state: &State) {
            let mut generator = state.generator.lock();
            let mut world = state.world.write();
            for x in 0..POINT_COUNT {
                for y in 0..POINT_COUNT {
                    let entity = generator.generate();
                    world.insert_component(
                        &entity,
                        Circle {
                            x: x as f32,
                            y: y as f32,
                            r: (x + y) as f32,

                        },
                    );
                }
            }
        }

        fn tick_points(state: &State) {
            let world = state.world.read();
            let query = world.query::<Point>();
            // increments speed by about 20% overall
            let mut reusable_guard = Some(world.mut_component_list::<Point>().unwrap());
            for ae in query.clone() {
                for be in query.clone() {
                    if ae == be {
                        continue;
                    }
                    let mut comps: ManyComponentMut<'_, Point, 2> =
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

        fn tick_circles(state: &State) {
            let world = state.world.read();
            let query = world.query::<Circle>();
            // increments speed by about 20% overall
            let mut reusable_guard = Some(world.mut_component_list::<Circle>().unwrap());
            for ae in query.clone() {
                for be in query.clone() {
                    if ae == be {
                        continue;
                    }
                    let mut comps: ManyComponentMut<'_, Circle, 2> =
                        ManyComponentMut::new(reusable_guard.take().unwrap(), [ae, be]);
                    let [a, b] = comps.get();
                    let dx = a.x - b.x;
                    let dy = a.y - b.y;
                    let dst = (dx * dx + dy * dy).sqrt();
                    a.x += dst * 0.01;
                    a.y += dst * 0.01;

                    reusable_guard = Some(comps.drop());
                }
            }
        }

        fn not_ecs() {
            let start_init = Instant::now();
            let mut points = vec![];
            let mut circles = vec![];
            for x in 0..POINT_COUNT {
                for y in 0..POINT_COUNT {
                    points.push(Point {
                        x: x as f32,
                        y: y as f32,
                        vx: 0.0,
                        vy: 0.0,
                    });
                    circles.push(Circle {
                        x: x as f32,
                        y: y as f32,
                        r: (x + y) as f32,
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
                        {
                            let a = &points[i];
                            let b = &points[j];

                            let dx = a.x - b.x;
                            let dy = a.y - b.y;

                            points[i].vx -= dx;
                            points[i].vy -= dy;
                        }
                        {
                            let a = &circles[i];
                            let b = &circles[j];
                            let dx = a.x - b.x;
                            let dy = a.y - b.y;
                            let dst = (dx * dx + dy * dy).sqrt() - (a.r + b.r);
                            circles[i].x += dst * 0.01;
                            circles[i].y += dst * 0.01;
                        }
                    }
                }
            }
            println!("iteration took {:?}", iteration_init.elapsed().as_micros());
        }
        fn ecs() {
            let mut state = State {
                world: World::new().into(),
                generator: DefaultEntityIdGenerator::new().into(),
            };
            let mut scheduler: PooledScheduler<SystemGroups, State> = PooledScheduler::new(Some(2));

            scheduler.add_systems([start_points, start_circles], SystemGroups::Start);
            scheduler.add_systems([tick_points, tick_circles], SystemGroups::Tick);
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
