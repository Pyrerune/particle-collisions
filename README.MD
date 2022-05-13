
# Calculus Project

<p>Simulates collisions between particles using linear algebra and physics</p>

---
## Notes

- Very poorly optimized so the maximum number of particles you can render at a time is 512

- Particles have a random color and mass

- Particle mass determines how large it is

- Sliders change the number of particles, their acceleration, and the resistance in the simulation

- Acceleration increases particle velocity and resistance decreases it

- Collisions between particles are calculated using each of their masses and velocities

- Due to some programming jank, only 1 collision can happen per particle per frame which means its possible for particles to overlap
- - However this only really happens when the frame rate is really low

- Currently, if a particle has a high enough velocity it can get stuck out of bounds (you may see some particles aggressively vibrating on the walls)

- For best results do not change the window border as it can also cause particles going out of bounds (luckily I store their position regardless of whether or not they are off the screen so if you make the window bigger they come back)
---

## Credits

- Written in Rust

- Uses the [Nannou](https://crates.io/crate/nannou) crate to render objects and the [Glam](https://crates.io/crate/glam) crate for vectors

- Physics equations collected from [Vobarian](https://www.vobarian.com/collisions/2dcollisions2.pdf) and [This youtube video](https://youtu.be/eED4bSkYCB8) by Reducible



