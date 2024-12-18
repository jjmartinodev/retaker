use std::sync::Arc;

use retaker::component::{EntityIdGenerator, World};

fn main() {
    let world = Arc::new(World::new());
    let entity_gen = Arc::new(EntityIdGenerator::new());
    let sara = entity_gen.generate();
    world.insert(&sara, String::from("Sara"));
    world.insert(&sara, 12u32);

    let alan = entity_gen.generate();
    world.insert(&alan, String::from("Alan"));
    world.insert(&alan, 14u32);
    world.insert(&alan, 181.0f32);

    println!("first!");
    world.query_mut::<String, (), _>(|e| {
        let s = e.mut_component::<String>();
        let a = e.mut_component::<u32>();
        let l = e.mut_component::<f32>();
        if let Some(name) = s {
            println!("{name}");
            *name = name.to_uppercase();
        }
        if let Some(age) = a {
            println!("{age}");
            *age += 1;
        }
        if let Some(height) = l {
            println!("{height}");
            *height += 0.5;
        }
    });
    world.query::<String, (), _>(|a| {
        world.clone().query::<String, (), _>(|b| {
            if a.id() == b.id() {
                
                return;
            }
            {
                
                let s = a.component::<String>();
                let d = a.component::<u32>();
                let l = a.component::<f32>();
                println!("{s:?} {d:?} {l:?}");
                
            }
            {
                let s = b.component::<String>();
                let c = b.component::<u32>();
                let l = b.component::<f32>();
                println!("{s:?} {c:?} {l:?}");
            }
        });
    });
}
