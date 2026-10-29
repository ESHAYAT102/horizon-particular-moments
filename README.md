# Horizon Particular Moments

An original Horizon mod inspired by the ambience of [Particular](https://modrinth.com/mod/particular-reforged). It adds falling leaves near trees, small fireflies in warm biomes at dusk, and splashes when nearby entities enter water. This is not a port and does not use Particular's source or artwork.

To see fireflies, stand outdoors in a plains or forest and set the time to dusk (`/time set 12000` in a world with commands enabled). Tiny amber pixels drift slowly above the ground at fixed world locations, from dusk through the night. They do not follow the player or camera. Falling leaves need nearby overhead leaf blocks. Water splashes need a fast entry into water. The effects do not appear on the title screen.

**Requires:** Horizon with mod API 14+ ([PR #2888](https://github.com/horizon-rust/horizon/pull/2888)). The current official client does not yet include these APIs. The mod is client-side. It does not edit worlds or require server installation.

## Install

Download `horizon_particular_moments.hmod` from [Releases](https://github.com/ESHAYAT102/horizon-particular-moments/releases) into your game home's `horizon-mods` folder. On Linux the default is `~/.local/share/horizon/horizon-mods/`. Restart Horizon and use the single **Horizon Particular Moments** ON/OFF control in **Options → Mods**. There is no second settings switch.

Or, after the release is available, use the install script:

```sh
curl -fsSL https://raw.githubusercontent.com/ESHAYAT102/horizon-particular-moments/main/install.sh | sh
```

To uninstall:

```sh
curl -fsSL https://raw.githubusercontent.com/ESHAYAT102/horizon-particular-moments/main/uninstall.sh | sh
```

Both scripts honor `HORIZON_HOME` for custom game folders. The install script backs up an existing package. The uninstall script removes only this mod's `.hmod`, not settings, worlds, or other mods. Neither script changes the installed Horizon client.

## Build from source

Use Rust with the `wasm32-unknown-unknown` target and Horizon's `cargo-horizon-mod` tool. Run `cargo horizon-mod build` from this directory. The manifest pins the SDK to the PR branch commit until its API reaches an official release. The package includes an original one-pixel firefly sprite and contains no third-party assets.

This is a small original set of effects, not every feature of Particular. Effects stop when the mod is disabled, and Horizon's particle quotas and player particle setting still apply.
