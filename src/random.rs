use rand_chacha::ChaCha8Rng;
use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct Random(pub ChaCha8Rng);