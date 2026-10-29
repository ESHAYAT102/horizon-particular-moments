use horizon_mod_sdk::{Mod, Registration, game, render};

struct Moments;

// Mix a tick and a position into repeatable offsets without filesystem access or RNG state.
fn mix(mut n: u64) -> u64 {
    n ^= n >> 30;
    n = n.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    n ^= n >> 27;
    n = n.wrapping_mul(0x94d0_49bb_1331_11eb);
    n ^ (n >> 31)
}

fn offset(n: u64) -> f64 {
    (mix(n) % 1600) as f64 / 100.0 - 8.0
}

fn leaves(state: &str) -> bool {
    state.starts_with("minecraft:") && state.split('[').next().unwrap_or("").ends_with("_leaves")
}

fn warm_biome(biome: &str) -> bool {
    matches!(
        biome,
        "minecraft:plains"
            | "minecraft:sunflower_plains"
            | "minecraft:meadow"
            | "minecraft:forest"
            | "minecraft:flower_forest"
            | "minecraft:birch_forest"
            | "minecraft:old_growth_birch_forest"
            | "minecraft:swamp"
            | "minecraft:mangrove_swamp"
            | "minecraft:jungle"
            | "minecraft:sparse_jungle"
    )
}

fn particle(kind: render::AmbientParticle, x: f64, y: f64, z: f64) -> render::WorldParticle {
    render::WorldParticle {
        kind,
        x,
        y,
        z,
        velocity_x: 0.0,
        velocity_y: 0.0,
        velocity_z: 0.0,
    }
}

impl Mod for Moments {
    fn init() -> Result<(Self, Registration), String> {
        Ok((Self, Registration::new().on_tick()))
    }

    fn tick(&mut self, tick: u64) {
        // Always drain events, including when the player is temporarily absent.
        let events = game::poll_world_events();
        let Some(player) = game::get_player() else {
            return;
        };
        if player.dimension != "minecraft:overworld" {
            return;
        }

        let mut splashes = Vec::new();
        for event in events {
            if event.kind == game::WorldEventKind::EnteredWater && event.speed > 0.2 {
                let count = (event.speed * 3.0).ceil().clamp(2.0, 8.0) as u64;
                for i in 0..count {
                    let mut drop = particle(
                        render::AmbientParticle::Splash,
                        event.x + offset(event.tick ^ i) * 0.08,
                        event.y + 0.1,
                        event.z + offset(event.tick ^ (i + 113)) * 0.08,
                    );
                    drop.velocity_y = 0.08 + i as f64 * 0.01;
                    splashes.push(drop);
                }
                if splashes.len() >= 24 {
                    break;
                }
            }
        }
        if !splashes.is_empty() {
            render::emit_particles(&splashes[..splashes.len().min(24)]);
        }

        if !tick.is_multiple_of(4) {
            return;
        }
        let x = (player.x + offset(tick) * 0.5).floor() as i32;
        let z = (player.z + offset(tick ^ 0xaaaa) * 0.5).floor() as i32;
        let y = player.y.floor() as i32;
        for height in [2, 3, 4, 5, 6] {
            let Some(sample) = game::sample_block(x, y + height, z) else {
                continue;
            };
            if leaves(&sample.state)
                && game::sample_block(x, y + height - 1, z)
                    .is_some_and(|below| below.state == "minecraft:air")
            {
                render::emit_configured_particles(&[render::ConfiguredParticle {
                    particle: particle(
                        render::AmbientParticle::CherryLeaves,
                        x as f64 + 0.5,
                        (y + height) as f64,
                        z as f64 + 0.5,
                    ),
                    color: 0xbcae69,
                    alpha: 0.8,
                    size: 0.09,
                    lifetime: 90,
                    gravity: 0.12,
                    friction: 0.98,
                    collision: true,
                    full_bright: false,
                }]);
                break;
            }
        }

        let dusk = game::get_clock()
            .is_some_and(|clock| (11500..=14500).contains(&(clock.day_time % 24000)));
        if dusk {
            // Keep fireflies near the player's view instead of scattering them behind the camera.
            let yaw = f64::from(player.yaw).to_radians();
            let distance = 2.5 + offset(tick ^ 17).abs() * 0.25;
            let sideways = offset(tick ^ 97) * 0.3;
            let fx = (player.x - yaw.sin() * distance + yaw.cos() * sideways).floor() as i32;
            let fz = (player.z + yaw.cos() * distance + yaw.sin() * sideways).floor() as i32;
            if let Some(sample) = game::sample_block(fx, y + 1, fz) {
                if sample.state == "minecraft:air"
                    && warm_biome(&sample.biome)
                    && sample.sky_light > 0
                {
                    render::emit_configured_particles(&[render::ConfiguredParticle {
                        particle: particle(
                            render::AmbientParticle::EndRod,
                            fx as f64 + 0.5,
                            player.y + 1.1 + offset(tick ^ 211) * 0.08,
                            fz as f64 + 0.5,
                        ),
                        color: 0xffe879,
                        alpha: 0.8,
                        size: 0.085,
                        lifetime: 75,
                        gravity: 0.0,
                        friction: 0.99,
                        collision: false,
                        full_bright: true,
                    }]);
                }
            }
        }
    }
}

horizon_mod_sdk::export_mod!(Moments);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_filters_and_offsets() {
        assert!(leaves("minecraft:oak_leaves[persistent=false]"));
        assert!(!leaves("some_mod:oak_leaves"));
        assert!(!leaves("minecraft:oak_log"));
        assert!(warm_biome("minecraft:swamp"));
        assert!(!warm_biome("minecraft:snowy_plains"));
        for n in 0..1000 {
            assert!((-8.0..8.0).contains(&offset(n)));
        }
    }
}
