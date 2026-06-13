use std::sync::LazyLock;

use libnoise::prelude::*;

#[allow(clippy::type_complexity)]
pub static SIMPLEX_GENERATOR: LazyLock<
    Blend<2, Fbm<2, Simplex<2>>, Scale<2, Worley<2>>, Scale<2, Worley<2>>>,
> = LazyLock::new(|| {
    Source::simplex(43) // start with simplex noise
        .fbm(5, 0.013, 2.0, 0.5) // apply fractal brownian motion
        .blend(
            // apply blending...
            Source::worley(43).scale([0.05, 0.05]), // ...with scaled worley noise
            Source::worley(44).scale([0.01, 0.01]),
        ) // ...controlled by other worley noise
});

pub mod resource_noise_tresholds {
    pub const WALL: (f64, f64) = (0.15, 1.);
    pub const COAL: (f64, f64) = (-0.18, -0.18);
    pub const MINERALS: (f64, f64) = (0.148, 0.15);
    pub const LAVA: (f64, f64) = (-1., -0.60);
}