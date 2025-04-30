use bevy::prelude::{
    Entity,
    Node,
    ChildOf,
    Query,
    With,
};

pub type QueryEntityParent<'w, 's> = Query<
    'w, 's,
    WorldQuery,
    ReadOnlyWorldQuery
>;

pub type WorldQuery = (Entity, &'static ChildOf);
pub type ReadOnlyWorldQuery = With<Node>;
