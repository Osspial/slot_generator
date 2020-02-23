# `slot_generator`

[![Crates.io](https://img.shields.io/crates/v/slot_generator.svg)](https://crates.io/crates/slot_generator)
[![Docs.rs](https://docs.rs/slot_generator/badge.svg)](https://docs.rs/slot_generator)

Efficiently generate and reclaim slot IDs from a numerical range.

The [`SlotGenerator`](crate::SlotGenerator) type works by maintaining a list of unused slot
ranges, and pulls the lowest available value from the slot pool whenever the user requests
one. Slots can be reclaimed and placed back in the generator pool with the `replace_slot`
methods. It's not a terribly complex implementation, and it shouldn't be used if you need to
generate secure IDs, but you don't always need a secure generator and the complexity that comes
from that.
