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

fn firefly_site(cell_x: i32, cell_z: i32) -> (i32, i32) {
    let seed = mix((cell_x as u32 as u64) << 32 | cell_z as u32 as u64);
    (
        cell_x * 8 + 1 + (seed % 6) as i32,
        cell_z * 8 + 1 + (mix(seed) % 6) as i32,
    )
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

        if !game::get_clock()
            .is_some_and(|clock| (11500..=23000).contains(&(clock.day_time % 24000)))
            || !tick.is_multiple_of(12)
        {
            return;
        }

        // A site's coordinates depend on the world grid, never the camera or player position.
        let cell = (tick / 12 % 9) as i32;
        let (fx, fz) = firefly_site(
            (player.x.floor() as i32).div_euclid(8) + cell % 3 - 1,
            (player.z.floor() as i32).div_euclid(8) + cell / 3 - 1,
        );
        if (fx as f64 - player.x).abs() > 12.0 || (fz as f64 - player.z).abs() > 12.0 {
            return;
        }
        for ground_y in (y - 8..=y + 3).rev() {
            let Some(ground) = game::sample_block(fx, ground_y, fz) else {
                break;
            };
            if ground.state == "minecraft:air" || ground.fluid != "minecraft:empty" {
                continue;
            }
            if !warm_biome(&ground.biome) {
                break;
            }
            let Some(air) = game::sample_block(fx, ground_y + 2, fz) else {
                break;
            };
            if air.state != "minecraft:air"
                || !game::sample_block(fx, ground_y + 1, fz)
                    .is_some_and(|above| above.fluid == "minecraft:empty")
                || !game::sample_block(fx, ground_y + 6, fz)
                    .is_some_and(|above| above.state == "minecraft:air")
            {
                break;
            }
            let seed = mix(tick ^ ((fx as u32 as u64) << 32) ^ fz as u32 as u64);
            let mut glow = particle(
                render::AmbientParticle::EndRod,
                fx as f64 + 0.5 + offset(seed) * 0.13,
                ground_y as f64 + 1.0 + (mix(seed ^ 11) % 120) as f64 / 100.0,
                fz as f64 + 0.5 + offset(seed ^ 23) * 0.13,
            );
            glow.velocity_x = offset(seed ^ 31) * 0.0005;
            glow.velocity_y = offset(seed ^ 41) * 0.00025;
            glow.velocity_z = offset(seed ^ 53) * 0.0005;
            render::emit_sprite_particles(&[render::SpriteParticle {
                sprite: "horizon_particular_moments:particle/firefly".into(),
                particle: render::ConfiguredParticle {
                    particle: glow,
                    color: 0xffe8a0,
                    alpha: 0.7,
                    size: 0.025,
                    lifetime: 120,
                    gravity: 0.0,
                    friction: 0.995,
                    collision: false,
                    full_bright: true,
                },
            }]);
            break;
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
        for x in -3..3 {
            for z in -3..3 {
                let (px, pz) = firefly_site(x, z);
                assert!((x * 8 + 1..x * 8 + 7).contains(&px));
                assert!((z * 8 + 1..z * 8 + 7).contains(&pz));
                assert_eq!((px, pz), firefly_site(x, z));
            }
        }
    }
}
