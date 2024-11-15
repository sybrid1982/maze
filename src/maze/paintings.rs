use bevy::prelude::*;

use crate::{consts, random::Random};
use rand::Rng;

#[repr(u32)]
#[derive(Copy, Clone, PartialEq)]
pub enum PaintingType {
    Square,
    LongHorizontal,
    Portrait
}

impl PaintingType {
    pub fn get_dimensions(&self) -> Vec2 {
        match self {
            PaintingType::Square => Vec2 {x: 2., y: 2.},
            PaintingType::LongHorizontal => Vec2 {x: 3., y: 1.},
            PaintingType::Portrait =>  Vec2 {x: 2., y: 3.},
        }
    }

    pub fn get_painting_type_from_index(index: u32) -> PaintingType {
        
        unsafe { ::std::mem::transmute(index) }
    }

    pub fn get_random_painting_type(rand: &mut ResMut<Random>) -> PaintingType {
        let index = rand.gen_range(0..3);
        PaintingType::get_painting_type_from_index(index)
    }

}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq)]
pub enum PaintingColor {
    Red,
    Yellow,
    Blue
}

impl PaintingColor {
    pub fn get_color(&self) -> Color {
        match self {
            PaintingColor::Red => Color::srgb(1.,0.,0.),
            PaintingColor::Yellow => Color::srgb(1.,1.,0.),
            PaintingColor::Blue => Color::srgb(0.,0.,1.),
        }
    }

    pub fn get_painting_color_from_index(index: u32) -> PaintingColor {
        
        unsafe { ::std::mem::transmute(index) }
    }

    pub fn get_random_painting_color(rand: &mut ResMut<Random>) -> PaintingColor {
        let index = rand.gen_range(0..3);
        PaintingColor::get_painting_color_from_index(index)
    }
}

#[derive(Clone)]
pub struct Painting {
    painting_type: PaintingType,
    painting_color: PaintingColor,
    is_north_west: bool
}

impl Painting {
    pub fn get_painting(&self, meshes: &mut ResMut<'_, Assets<Mesh>>, materials: &mut ResMut<'_, Assets<StandardMaterial>>) -> PbrBundle {
        let wall_offset = if self.is_north_west { consts::PAINTING_THICKNESS } else { -1.0 * consts::PAINTING_THICKNESS};
        PbrBundle {
            mesh: meshes.add(Cuboid::new(self.painting_type.get_dimensions().x, consts::WALL_THICKNESS, self.painting_type.get_dimensions().y)),
            material: materials.add(self.painting_color.get_color()),
            transform: Transform::from_xyz(0., wall_offset, 0.),
            ..default()
        }
    }

    pub fn generate_random_painting(rand: &mut ResMut<Random>) -> Option<Painting> {
        let random: f32 = rand.gen_range(0. .. 1.);

        if random > consts::PROBABILITY_PAINTING {
            return None
        }

        let painting = Painting {
            painting_type: PaintingType::get_random_painting_type(rand),
            painting_color: PaintingColor::get_random_painting_color(rand),
            is_north_west: rand.gen_bool(0.5)
        };

        Some(painting)
    }
}