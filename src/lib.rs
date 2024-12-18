pub mod component;
pub mod entity;
pub mod system;

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::{
        component::{EntityIdGenerator, World},
        system::ThreadedScheduler,
    };

    struct State {
        world: World,
        entity_gen: EntityIdGenerator,
    }

    #[test]
    fn calculate_par() {
        #[derive(PartialEq, Eq, Hash)]
        pub enum Group {
            Initialize,
            Calculate,
        }

        struct Add(f32, f32, f32);
        struct Sub(f32, f32, f32);
        struct Mul(f32, f32, f32);
        struct Div(f32, f32, f32);

        const N: usize = 2000;

        fn add_additions(state: &State) {
            let mut calcs = vec![];
            for a in 0..N {
                for b in 0..N {
                    let add = state.entity_gen.generate();
                    calcs.push((add, Add(a as f32, b as f32, 0.0)));
                }
            }
            state.world.insert_many(calcs);
        }

        fn add_subtractions(state: &State) {
            let mut calcs = vec![];
            for a in 0..N {
                for b in 0..N {
                    let entity = state.entity_gen.generate();
                    calcs.push((entity, Sub(a as f32, b as f32, 0.0)));
                }
            }
            state.world.insert_many(calcs);
        }

        fn add_multiplications(state: &State) {
            let mut calcs = vec![];
            for a in 0..N {
                for b in 0..N {
                    let entity = state.entity_gen.generate();
                    calcs.push((entity, Mul(a as f32, b as f32, 0.0)));
                }
            }
            state.world.insert_many(calcs);
        }

        fn add_divisions(state: &State) {
            let mut calcs = vec![];
            for a in 0..N {
                for b in 0..N {
                    let entity = state.entity_gen.generate();
                    calcs.push((entity, Div(a as f32, b as f32, 0.0)));
                }
            }
            state.world.insert_many(calcs);
        }

        fn perform_additions(state: &State) {
            state.world.query_mut::<Add, (), _>(|calculation| {
                let nums = calculation.mut_component::<Add>().unwrap();
                nums.2 = nums.0 + nums.1;
            });
            println!("running_additions");
        }

        fn perform_subtractions(state: &State) {
            state.world.query_mut::<Sub, (), _>(|calculation| {
                let nums = calculation.mut_component::<Sub>().unwrap();
                nums.2 = nums.0 - nums.1;
            });
            println!("running_subtractions");
        }

        fn perform_multiplications(state: &State) {
            state.world.query_mut::<Mul, (), _>(|calculation| {
                let nums = calculation.mut_component::<Mul>().unwrap();
                nums.2 = nums.0 * nums.1;
            });
            println!("running_multiplications");
        }

        fn perform_divisions(state: &State) {
            state.world.query_mut::<Div, (), _>(|calculation| {
                let nums = calculation.mut_component::<Div>().unwrap();
                nums.2 = nums.0 / nums.1;
            });
            println!("running_divisions");
        }

        fn single_threaded() {
            let state = State {
                world: World::new(),
                entity_gen: EntityIdGenerator::new(),
            };

            let scheduler: ThreadedScheduler<_, _> = ThreadedScheduler::new(1)
                .add_systems(
                    [
                        add_additions,
                        add_subtractions,
                        add_multiplications,
                        add_divisions,
                    ],
                    Group::Initialize,
                )
                .add_systems(
                    [
                        perform_additions,
                        perform_subtractions,
                        perform_multiplications,
                        perform_divisions,
                    ],
                    Group::Calculate,
                );

            let init_start = Instant::now();
            scheduler.run_group_local(&state, &Group::Initialize);
            println!("singlethreaded initialization: {:?}ms", init_start.elapsed().as_millis());

            let proccess_start = Instant::now();
            scheduler.run_group_local(&state, &Group::Calculate);
            println!("singlethreaded proccesing: {:?}ms", proccess_start.elapsed().as_millis());
        }

        fn multi_threaded() {
            let state = State {
                world: World::new(),
                entity_gen: EntityIdGenerator::new(),
            };

            let scheduler: ThreadedScheduler<_, _> = ThreadedScheduler::new(4)
                .add_systems(
                    [
                        add_additions,
                        add_subtractions,
                        add_multiplications,
                        add_divisions,
                    ],
                    Group::Initialize,
                )
                .add_systems(
                    [
                        perform_additions,
                        perform_subtractions,
                        perform_multiplications,
                        perform_divisions,
                    ],
                    Group::Calculate,
                );

            let init_start = Instant::now();
            scheduler.run_group_par(&state, &Group::Initialize);
            println!("multithreaded initialization: {:?}ms", init_start.elapsed().as_millis());
            let proccess_start = Instant::now();
            scheduler.run_group_par(&state, &Group::Calculate);
            println!("multithreaded proccesing: {:?}ms", proccess_start.elapsed().as_millis());
        }

        single_threaded();
        multi_threaded();
    }
}
