use crate::prelude::StyleSheet;
use bevy::prelude::{
    Children,
    Entity,
    ChildOf,
    Query,
};

pub type QueryUiNodes<'w, 's> = Query<
    'w, 's,
    WorldQuery,
    ReadOnlyWorldQuery,
>;

pub type WorldQuery = (
    Entity,
    Option<&'static ChildOf>,
    Option<&'static Children>,
    Option<&'static StyleSheet>
);

pub type ReadOnlyWorldQuery = ();
