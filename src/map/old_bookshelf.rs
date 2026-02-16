use bevy::prelude::*;
use avian2d::prelude::*;
use crate::map::physics::GameLayer;

pub(super) fn plugin(app: &mut App) {

}

#[derive(Component)] 
pub struct OldBookshelf {
    pub can_interact: bool,
    pub size: Vec2,
}

pub fn spawn_old_bookshelf() -> impl Bundle {
    let book_shelf_size = Vec2::new(100.0, 100.0);
    (
        Name::new("Old BookShelf"),
        OldBookshelf {
            can_interact: false,
            size: book_shelf_size,
        },
        Sprite {
            color: Color::srgb(103.0, 68.0, 34.0),
            custom_size: Some(book_shelf_size), 
            ..default()
        },
        Transform::from_xyz(2970.0, 250.0, -9.0),
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct BookshelfCollider;

pub fn spawn_bookshelf_collision(

) -> impl Bundle {
    (
        Name::new("Old Book Shelf Collision"),
        BookshelfCollider,
        LockedAxes::ROTATION_LOCKED,
        RigidBody::Static,
        Collider::rectangle(100.0, 100.0),
        CollisionLayers::new(GameLayer::OldBookshelf,[
            GameLayer::Player,
        ]),
        Transform::from_xyz(2970.0, 250.0, 0.0)
    )
}