use std::marker::PhantomData;

use bevy::prelude::*;

use super::grid_location::GridLocation;

#[derive(Event)]
pub struct DirtyGridEvent<T>(pub GridLocation, pub PhantomData<T>);