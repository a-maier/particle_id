# particle_id

Particle numbers according to the [Monte Carlo Particle Numbering
Scheme](https://pdg.lbl.gov/2023/mcdata/mc_particle_id_contents.html)

## Example

```rust
use particle_id::light_baryons::*;

assert_eq!(proton.name().unwrap(), "proton");
assert_eq!(proton.id(), 2212);
assert_eq!(proton.anti().id(), -proton.id());
```

License: GPL-3.0-or-later
