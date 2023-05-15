# alisaie 

`alisaie` is composed of two parts: `alisaie-api` and `alisaie-bot`, providing
an API wrapper over [XIVAPI](https://xivapi.com) and a Discord bot for
FFXIV-related functionalities.

## crates

### [`alisaie-api`](alisaie-api)

an API wrapper for [XIVAPI](https://xivapi.com). this implementation is mostly
based off of [ascclemens'](https://github.com/ascclemens) [API
wrapper](https://github.com/ascclemens/xivapi-rs), but rewritten for newer
versions of Rust and dependencies.

### [`alisaie-bot`](alisaie-bot)

a Discord bot using [`serenity-rs`](https://github.com/serenity-rs/serenity),
utilizing `alisaie-api` for FFXIV functionalities.

## license

licensed under either of

*   Apache License, Version 2.0  
    ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
*   MIT license  
	([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## contribution

unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
