use rand_chacha::ChaCha8Rng;
use bevy::prelude::*;
use rand::Rng;

#[derive(Resource, Deref, DerefMut)]
pub struct Random(pub ChaCha8Rng);

impl Random {
    pub fn choose<T>(&mut self, options: &Vec<T>) -> T 
    where T: Clone
    {
        let index = self.0.gen_range(0..options.len());
        options[index].clone()
    }
}