use nannou::geom::Rect;
use nannou::rand::Rng;
use nannou::{color::rgb, prelude::*};
use nannou_conrod as ui;
use nannou_conrod::prelude::*;
impl Particle {
    fn update_velocity(&mut self, v: Point2) {
        self.velocity = v;
    }
    fn collide(&mut self, particles: &Vec<Particle>, scale: f32) -> Option<(Particle, Point2)> {
        for particle in particles {
            if self.position.distance(particle.position) < 0.5 {
                return None;
            }
            if self.position.distance(particle.position) < scale * 2.0 {
                assert!((self.mass + particle.mass) > 0.0);
                let mut mass = (2.0 * particle.mass) / (self.mass + particle.mass);
                let mut net_velocity = self.velocity - particle.velocity;
                let mut dist = self.position - particle.position;
                let mut v_numerator = (net_velocity.x * dist.x) + (net_velocity.y * dist.y);
                let mut v_denominator = (dist.x.abs() + dist.y.abs()).powf(2.0);
                assert!(v_denominator > 0.0);
                let v1_prime = mass * (v_numerator / v_denominator) * dist;
                mass = (2.0 * self.mass) / (self.mass + particle.mass);
                net_velocity = particle.velocity - self.velocity;
                dist = particle.position - self.position;
                v_numerator = (net_velocity.x * dist.x) + (net_velocity.y * dist.y);
                v_denominator = (dist.x.abs() + dist.y.abs()).powf(2.0);
                assert!(v_denominator > 0.0);
                let v2_prime = mass * (v_numerator / v_denominator) * dist;
                self.velocity = v1_prime;
                return Some((*particle, v2_prime));
            }
        }
        None
    }
    fn detect_global(&mut self, global: &Rect) {
        let cbox = self.position;
        if cbox.x - self.scale < global.left() || cbox.x + self.scale > global.right() {
            self.velocity.x *= -1.0;
        }
        if cbox.y - self.scale < global.bottom() || cbox.y + self.scale > global.top() {
            self.velocity.y *= -1.0;
        }
    }
    fn apply_acceleration(&mut self, acceleration: f32, resistance: f32) {
        let net = acceleration - resistance;
        if self.velocity.x < 0.0 {
            self.velocity.x -= net;
        } else {
            self.velocity.x += net;
        }
        if self.velocity.y < 0.0 {
            self.velocity.y -= net;
        } else {
            self.velocity.y += net;
        }
    }
    fn update_position(&mut self, delta: f32) {
        self.position += self.velocity * delta;
    }
}
#[derive(Clone, Copy, PartialEq, Debug)]
struct Particle {
    position: Point2,
    velocity: Point2,
    collision_box: Rect,
    mass: f32,
    scale: f32,
    color: Rgb,
}
struct Model {
    ui: Ui,
    ids: Ids,
    particles: Vec<Particle>,
    acceleration: f32,
    num_particle: u32,
    resistance: f32,
}

widget_ids! {
    struct Ids {
        particle_red,
        particle_green,
        particle_blue,
        acceleration,
        particles,
        resistance
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(rgb(0.0, 0.0, 0.0));
    for particle in &model.particles {
        draw.ellipse()
            .xy(particle.position)
            .radius(particle.scale)
            .color(particle.color);
    }
    draw.to_frame(app, &frame).unwrap();
    model.ui.draw_to_frame(app, &frame).unwrap();
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::RefreshSync);
    let wid = app
        .new_window()
        .size(500, 500)
        .title("Particle Collisions")
        .raw_event(raw_window_event)
        .view(view)
        .build()
        .unwrap();

    let mut ui = ui::builder(app).window(wid).build().unwrap();
    let ids = Ids::new(ui.widget_id_generator());

    Model {
        ui,
        ids,
        particles: vec![],
        acceleration: 0.0,
        num_particle: 2,
        resistance: 0.0,
    }
}

fn raw_window_event(app: &App, model: &mut Model, event: &ui::RawWindowEvent) {
    model.ui.handle_raw_event(app, event);
}

fn update(app: &App, model: &mut Model, update: Update) {
    let ui = &mut model.ui.set_widgets();
    fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            .w_h(100.0, 30.0)
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
    }

    //Slider Values
       {
        for value in slider(model.acceleration, 0.0, 500.0)
            .top_left_with_margin(5.0)
            .label("Accel")
            .set(model.ids.acceleration, ui)
        {
            model.acceleration = value.round();
        }
        for value in slider(model.num_particle as f32, 2.0, 512.0)
            .down(10.0)
            .label("Particles")
            .set(model.ids.particles, ui)
        {
            model.num_particle = value.trunc() as u32;
        }
        for value in slider(model.resistance, 0.0, 1000.0)
            .down(10.0)
            .label("Resistance")
            .set(model.ids.resistance, ui) {
                model.resistance = value;
            }
    }
    let mut particles = model.particles.clone();

    let mut rand = nannou::rand::thread_rng();
    for i in particles.len()..model.num_particle as usize {
        let mass = rand.gen_range(0.0..1.0);
        let scale = mass * 10.0;
        println!(
            "Not Enough Particles {}",
            model.num_particle as usize - particles.len()
        );
        let position = pt2(rand.gen_range(-50.0..50.0), rand.gen_range(-50.0..50.0));

        let collision_box = Rect::from_xy_wh(position, pt2(scale * 2.0, scale * 2.0));
        let color = rgb(rand.gen_range(0.1..1.0), rand.gen_range(0.1..1.0), rand.gen_range(0.1..1.0));
        let particle = Particle {
            velocity: pt2(rand.gen_range(-100.0..100.0), rand.gen_range(-100.0..100.0)),
            position,
            collision_box,
            mass,
            scale,
            color,
        };
        particles.push(particle);
    }

    if particles.len() > model.num_particle as usize {
        let (a, _) = particles.split_at(model.num_particle as usize);
        let vec = a.to_vec();
        particles = vec;
    }
    let global = app.window_rect();
    let delta = update.since_last.as_secs_f32();
    let temp = particles.clone();
    let mut new = vec![];
    for particle in &particles {
        let mut particle = particle.clone();
        
        if let Some((mut first, velocity)) = particle.collide(&temp, particle.scale) {
            if let Some(pos) = new.iter().position(|a: &Particle| a == &first) {
                let first = new.get_mut(pos).unwrap();

                first.update_velocity(velocity);
                //first.collided(true);
            }
        }
        
        particle.apply_acceleration(model.acceleration * delta, model.resistance * delta);
        
        particle.update_position(delta);
        particle.detect_global(&global);

        new.push(particle);
    }
    if new.len() > model.num_particle as usize {
        let (a, _) = particles.split_at(model.num_particle as usize);
        let vec = a.to_vec();
        new = vec;
    }

    model.particles = new;
}

fn main() {
    nannou::app(model).update(update).run();
}
