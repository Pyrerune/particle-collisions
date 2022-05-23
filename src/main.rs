use nannou::geom::Rect;
use nannou::rand::Rng;
use nannou::{color::rgb, prelude::*};
use nannou_conrod as ui;
use nannou_conrod::prelude::*;
use ui::widget::button::Flat;
trait VecReplace {
    fn replace_all(&mut self, other: &Self, base: usize);
}
impl VecReplace for Vec<Particle> {
    fn replace_all(&mut self, other: &Self, base: usize) {
        for i in 0..other.len() {
            self[base+i] = other[i];
        }
    }
}
impl Particle {
    fn overlaps(&self, other: &Particle) -> bool {
        let self_x = self.position.x;
        let other_x = other.position.x;
        let self_left = self_x - self.scale;
        let other_left = other_x - other.scale;
        let self_right = self_x + self.scale;
        let other_right = other_x + other.scale;
        (self_left > other_left && self_left < other_right) || (self_right < other_right && self_right > other_left)
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
    id: usize,
    
}
struct Sim {
    acceleration: f32,
    resistance: f32,
    num_particle: usize,
    particles: Vec<Particle>,
}
struct Model {
    sim: Sim,
    ui: Ui,
    ids: Ids,
    stop_label: &'static str,
    stop: bool,
}
impl Model {
    fn particles(&self) -> &Vec<Particle> {
        &self.sim.particles
    }
}
impl Sim {
    fn create_particles(&mut self, global: Rect) {
        let mut rand = nannou::rand::thread_rng();
        let mut available_ids = (0..self.num_particle).collect::<Vec<_>>();
        for _i in 0..self.num_particle {
            let mass = rand.gen_range(0.5..1.0);
            let scale = mass * 15.0;
            let position = pt2(rand.gen_range(global.left()..global.right()), rand.gen_range(global.bottom()..global.top()));
            let id = available_ids.remove(rand.gen_range(0..available_ids.len()));
            let collision_box = Rect::from_xy_wh(position, pt2(scale * 2.0 + 2.0, scale * 2.0 + 2.0));
            let color = rgb(rand.gen_range(0.1..1.0), rand.gen_range(0.1..1.0), rand.gen_range(0.1..1.0));
            let particle = Particle {
                velocity: pt2(rand.gen_range(-100.0..100.0), rand.gen_range(-100.0..100.0)),
                position,
                collision_box,
                mass,
                scale,
                color,
                id,
            };
            self.particles.push(particle);
        }
        self.particles.sort_by(|a, b| {
            a.position.x.partial_cmp(&b.position.x).unwrap_or(std::cmp::Ordering::Equal)
        });
    }
    fn update_particles(&mut self, delta: f32, global: &Rect) {

        for i in 0..self.particles.len() {
            let mut particle = self.particles[i];
            self.collide(&mut particle);
            particle.apply_acceleration(self.acceleration * delta, self.resistance * delta);
            particle.detect_global(global);
            particle.update_position(delta);
            self.particles[i] = particle;
        }
    }
    fn collide(&mut self, particle: &mut Particle) {
        collide(particle, &mut self.particles);
    }
}

widget_ids! {
    struct Ids {
        particle_red,
        particle_green,
        particle_blue,
        acceleration,
        particles,
        resistance,
        stop,

    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(rgb(0.0, 0.0, 0.0));
    for particle in model.particles() {
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
        sim: Sim {
            
            particles: vec![],
            acceleration: 0.0,
            num_particle: 2,
            resistance: 0.0,
        },
        ui,
        ids,
        stop_label: "Start",
        stop: true,
    }
}

fn raw_window_event(app: &App, model: &mut Model, event: &ui::RawWindowEvent) {
    model.ui.handle_raw_event(app, event);
}

fn update(app: &App, model: &mut Model, update: Update) {
    let sim = &mut model.sim;
    let ui = &mut model.ui.set_widgets();
    fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            .w_h(100.0, 30.0)
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
    }
    fn button(label: &'static str) -> widget::Button<'static, Flat> {
        widget::Button::new()
        .w_h(100.0, 30.0)
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .label(label)

    }

    //Interface
    {
        let t = button(model.stop_label).top_left_with_margin(5.0).set(model.ids.stop, ui);
        if t.was_clicked() {
            if model.stop {
                model.stop_label = "Stop";
            } else {
                model.stop_label = "Start";
            }
            model.stop = !model.stop;
        }
        for value in slider(sim.acceleration, 0.0, 500.0)
            .down(10.0)
            .label("Accel")
            .set(model.ids.acceleration, ui)
        {
            sim.acceleration = value.round();
        }
        for value in slider(sim.num_particle as f32, 2.0, 512.0)
            .down(10.0)
            .label("Particles")
            .set(model.ids.particles, ui)
        {
            sim.num_particle = value.trunc() as usize;
        }
        for value in slider(sim.resistance, 0.0, 1000.0)
            .down(10.0)
            .label("Resistance")
            .set(model.ids.resistance, ui) {
                sim.resistance = value;
            }
    }
    let global = app.window_rect();
    let delta = update.since_last.as_secs_f32();
    if model.stop {
        sim.particles.clear();
    } else if sim.particles.is_empty() {

        sim.create_particles(global);
    }
    sim.update_particles(delta, &global)
    
}
fn collide_a(particle: &mut Particle, active: &mut Vec<(Particle, usize)>) {
    for (other, _) in active {
        if other.id != particle.id {
            if particle.position.distance(other.position) < particle.scale + other.scale {
                let normal = pt2(other.position.x - particle.position.x, other.position.y - particle.position.y);
                
                let unit_normal = normal / (normal.x.powf(2.0) + normal.y.powf(2.0)).sqrt();
                let unit_tangent = pt2(-unit_normal.y, unit_normal.x);
                
                let s_self_n = particle.velocity.dot(unit_normal);
                let s_self_t = particle.velocity.dot(unit_tangent);

                let s_other_n = other.velocity.dot(unit_normal);
                let s_other_t = other.velocity.dot(unit_tangent);

                let denominator = particle.mass + other.mass;

                let s_prime_self_n = ((s_self_n * (particle.mass - other.mass)) + (2.0 * other.mass * s_other_n)) / denominator;
                let s_prime_other_n = ((s_other_n * (other.mass - particle.mass)) + (2.0 * particle.mass * s_self_n)) / denominator;

                let v_self_n = s_prime_self_n * unit_normal;
                let v_self_t = s_self_t * unit_tangent;

                let v_other_n = s_prime_other_n * unit_normal;
                let v_other_t = s_other_t * unit_tangent;
            
                particle.velocity = v_self_n + v_self_t;
                other.velocity = v_other_n + v_other_t;
            }
        }
    }

}
fn collide(particle: &mut Particle, vec: &mut Vec<Particle>) {
    for other in vec {
        if other.id != particle.id {
            if particle.position.distance(other.position) < particle.scale + other.scale {
                let normal = pt2(other.position.x - particle.position.x, other.position.y - particle.position.y);
                
                let unit_normal = normal / (normal.x.powf(2.0) + normal.y.powf(2.0)).sqrt();
                let unit_tangent = pt2(-unit_normal.y, unit_normal.x);
                
                let s_self_n = particle.velocity.dot(unit_normal);
                let s_self_t = particle.velocity.dot(unit_tangent);

                let s_other_n = other.velocity.dot(unit_normal);
                let s_other_t = other.velocity.dot(unit_tangent);

                let denominator = particle.mass + other.mass;

                let s_prime_self_n = ((s_self_n * (particle.mass - other.mass)) + (2.0 * other.mass * s_other_n)) / denominator;
                let s_prime_other_n = ((s_other_n * (other.mass - particle.mass)) + (2.0 * particle.mass * s_self_n)) / denominator;

                let v_self_n = s_prime_self_n * unit_normal;
                let v_self_t = s_self_t * unit_tangent;

                let v_other_n = s_prime_other_n * unit_normal;
                let v_other_t = s_other_t * unit_tangent;
            
                particle.velocity = v_self_n + v_self_t;
                other.velocity = v_other_n + v_other_t;
            }
        }
    }

}
fn main() {
    nannou::app(model).update(update).run();
}
