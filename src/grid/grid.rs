use std::{marker::PhantomData, ops::{Index, IndexMut}};

use bevy::prelude::*;

use super::{dirty_grid_event::DirtyGridEvent, grid_location::GridLocation};

pub const GRID_SIZE: usize = 20;

// this is cribbed heavily from a tutorial I saw.  I don't know that I'm keeping this, but my thinking is to make the maze grid much more data oriented
// have the maze cells as a grid, maybe do walls and doors as a hash map where the keys are the two grid positions involved
// could then run pathfinding by looking at cells, and determine if you can pass from one cell to another via the hashmap
// OR we could have the doors only stored in the hash map, and then have the navigation work by the room

#[derive(Resource)]
pub struct Grid<T> {
    pub entities: [[Option<Entity>; GRID_SIZE]; GRID_SIZE],
    _marker: PhantomData<T>
}

impl<T> Index<&GridLocation> for Grid<T> {
    type Output = Option<Entity>;

    fn index(&self, index: &GridLocation) -> &Self::Output {
        &self.entities[index.x as usize][index.y as usize]
    }
}

impl<T> IndexMut<&GridLocation> for Grid<T> {
    fn index_mut(&mut self, index: &GridLocation) -> &mut Self::Output {
        &mut self.entities[index.x as usize][index.y as usize]
    }
}

impl<T> Grid<T> {
    pub fn iter(&self) -> impl Iterator<Item = (Entity, GridLocation)> + '_ {
        self.entities
            .iter()
            .flatten()
            .enumerate()
            .filter(|(_i, entity)| entity.is_some())
            .map(|(i, entity)| {
                (
                    entity.unwrap(),
                    GridLocation::new(i as u32 / GRID_SIZE as u32, i as u32 % GRID_SIZE as u32)
                )
            })
    }
}

fn remove_from_grid<T: Component>(
    mut grid: ResMut<Grid<T>>,
    mut query: RemovedComponents<T>,
    mut dirty: EventWriter<DirtyGridEvent<T>>
) {
    for removed_entity in query.read() {
        let removed = grid.iter().find(|(entity, _)| *entity == removed_entity);
        if let Some((_, location)) = removed {
            dirty.send(DirtyGridEvent::<T>(
                location.clone(),
                PhantomData::default()
            ));
            grid[&location] = None;
        }
    }
}

fn add_to_grid<T: Component>(
    mut grid: ResMut<Grid<T>>,
    query: Query<(Entity, &GridLocation), Added<T>>,
    mut dirty: EventWriter<DirtyGridEvent<T>>
) {
    for (entity, location) in &query {
        if let Some(existing) = grid[location] {
            if existing != entity {
                warn!("Over-writing entity in grid");
                dirty.send(DirtyGridEvent::<T>(
                    location.clone(),
                    PhantomData::default()    
                ));
            }
        } else {
            dirty.send(DirtyGridEvent::<T>(
                location.clone(),
                PhantomData::default()    
            ));
            grid[location] = Some(entity);
        }
    }
}

#[derive(Default)]
pub struct GridPlugin<T> {
    _marker: PhantomData<T>
}

impl<T: Component> Plugin for GridPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<Grid<T>>()
            .add_event::<DirtyGridEvent<T>>()
            .add_systems(PreUpdate, (add_to_grid::<T>, remove_from_grid::<T>));
    }
}

impl<T> Clone for Grid<T> {
    fn clone(&self) -> Self {
        Self {
            entities: self.entities,
            _marker: self._marker,
        }
    }
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Self {
            entities: [[None; GRID_SIZE]; GRID_SIZE],
            _marker: Default::default(),
        }
    }
}

impl<T> Grid<T> {
    pub fn occupied(&self, location: &GridLocation) -> bool {
        Grid::<T>::valid_index(location) && self[location].is_some()
    }

    pub fn valid_index(location: &GridLocation) -> bool {
        location.x >= 0
            && location.y >= 0
            && location.x < GRID_SIZE as i32
            && location.y < GRID_SIZE as i32
    }
}