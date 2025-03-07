use macroquad::{
    color::WHITE,
    input::{is_key_down, is_key_released, KeyCode},
    math::{vec2, Vec2},
    miniquad::window::screen_size,
    rand::gen_range,
    shapes::draw_circle,
    window::{next_frame, screen_height},
};
use retaker::{
    work::{ThreadedWork, Work},
    world::{LockedWorld, World},
};

pub struct Camera {
    position: Vec2,
    scale: f32,
    follow_average: bool,
    hide_ui: bool,
}

pub struct ParticleParameters {
    delta_time: f32,
    particle_mass: f32,
    reset: bool,
    particle_count: f32,
    velocity_variation: f32,
}

pub struct Particle {
    position: Vec2,
    velocity: Vec2,
}

fn start_camera(world: &LockedWorld) {
    let mut world = world.lock_exclusive();

    world.create_resource(Camera {
        position: Vec2::ZERO,
        scale: 1.0,
        follow_average: false,
        hide_ui: false
    });
}

fn start_particles(world: &LockedWorld) {
    let mut world = world.lock_exclusive();

    world.create_resource(ParticleParameters {
        delta_time: 1.0 / 30.0,
        particle_mass: 1.0,
        reset: true,
        particle_count: 100.0,
        velocity_variation: 0.0,
    });

    reset_particles(&mut world);
}

fn reset_particles(world: &mut World) {
    if let Some(mut particles) = world.component_list_mut::<Particle>() {
        particles.clear();
    }
    let mut parameters = world.resource_mut::<ParticleParameters>().unwrap();
    parameters.reset = false;
    let particle_count = parameters.particle_count;
    let velocity_variation = parameters.velocity_variation;
    drop(parameters);
    for _ in 0..particle_count as usize {
        let particle = world.create_entity();
        world.insert(
            &particle,
            Particle {
                position: vec2(gen_range(0.0, 600.0), gen_range(0.0, 600.0)),
                velocity: vec2(
                    gen_range(-velocity_variation, velocity_variation),
                    gen_range(-velocity_variation, velocity_variation),
                ),
            },
        );
    }
}

fn update_particles(world: &LockedWorld) {
    let world = world.lock_shared();
    let parameters = world.resource_ref::<ParticleParameters>().unwrap();
    let mut entities = world.component_list_mut::<Particle>().unwrap();
    let query = entities.query();
    for a_id in query.clone() {
        for b_id in query.clone() {
            if a_id == b_id {
                continue;
            }
            let [a, b] = entities.get_many_mut([&a_id, &b_id]).unwrap();

            let difference = (b.position - a.position) * parameters.particle_mass;
            a.velocity += difference * parameters.delta_time;
            b.velocity -= difference * parameters.delta_time;
        }
    }
    for id in query {
        let particle = entities.get_mut(&id).unwrap();
        particle.position += particle.velocity * parameters.delta_time;
    }
}

fn update_ui(world: &LockedWorld) {
    let world = world.lock_shared();
    let mut camera = world.resource_mut::<Camera>().unwrap();
    if camera.hide_ui {
        return;
    }
    let mut parameters = world.resource_mut::<ParticleParameters>().unwrap();
    macroquad::ui::root_ui().window(
        1,
        vec2(0.0, screen_height() - 160.0),
        vec2(360.0, 160.0),
        |ui| {
            parameters.reset = ui.button(vec2(0.0, 140.0), "reset");
            ui.slider(
                1,
                "particle mass",
                0.01..10.0,
                &mut parameters.particle_mass,
            );
            ui.slider(
                2,
                "particle count",
                3.0..1000.0,
                &mut parameters.particle_count,
            );
            ui.slider(3, "delta time", 0.001..1.0, &mut parameters.delta_time);
            ui.slider(4, "camera scale", 0.01..5.0, &mut camera.scale);
            ui.slider(
                5,
                "velocity variation",
                0.0..1000.0,
                &mut parameters.velocity_variation,
            );
            ui.checkbox(6, "camera follow", &mut camera.follow_average);
        },
    );
}

fn update_camera(world: &LockedWorld) {
    let world = world.lock_shared();
    let mut camera = world.resource_mut::<Camera>().unwrap();
    if is_key_released(KeyCode::Z) {
        camera.hide_ui = if camera.hide_ui {false} else {true};
    }
    if camera.follow_average {
        let entities = world.component_list_ref::<Particle>().unwrap();
        let mut average = Vec2::ZERO;
        let mut particle_count = 0.;
        for id in entities.query() {
            let particle = entities.get(&id).unwrap();
            average += particle.position;
            particle_count += 1.;
        }
        camera.position =
            (average / particle_count) - Vec2::from(screen_size()) / 2. / camera.scale;
        return;
    }
    if is_key_down(KeyCode::W) {
        camera.position.y -= 10. / camera.scale;
    }
    if is_key_down(KeyCode::A) {
        camera.position.x -= 10. / camera.scale;
    }
    if is_key_down(KeyCode::S) {
        camera.position.y += 10. / camera.scale;
    }
    if is_key_down(KeyCode::D) {
        camera.position.x += 10. / camera.scale;
    }
}

fn check_reset(world: &LockedWorld) {
    let world = world.lock_upgradable();
    let parameters = world.resource_ref::<ParticleParameters>().unwrap();
    if parameters.reset {
        drop(parameters);

        let mut world = world.upgrade();
        reset_particles(&mut world);
    }
}

fn draw(world: &LockedWorld) {
    let world = world.lock_shared();
    let camera = world.resource_ref::<Camera>().unwrap();
    let entities = world.component_list_ref::<Particle>().unwrap();
    let query = entities.query();
    for id in query {
        let particle = entities.get(&id).unwrap();
        draw_circle(
            (particle.position.x - camera.position.x) * camera.scale,
            (particle.position.y - camera.position.y) * camera.scale,
            5.0,
            WHITE,
        );
    }
}

#[macroquad::main("")]
async fn main() {
    let world = LockedWorld::new();

    let start_work = ThreadedWork::new()
        .add_system(start_particles)
        .add_system(start_camera);
    let update_work = ThreadedWork::new()
        .add_system(update_particles)
        .add_system(check_reset);
    let draw_work = Work::new()
        .add_system(draw)
        .add_system(update_ui)
        .add_system(update_camera);

    start_work.run(&world);
    loop {
        update_work.run(&world);
        draw_work.run(&world);
        next_frame().await;
    }
}
