use super::ComponentFilter;

use bevy::{
    prelude::Resource,
    platform::collections::HashMap,
};

#[derive(Default, Resource)]
pub(crate) struct ComponentFilterRegistry(
    pub HashMap<&'static str, Box<dyn ComponentFilter + Send + Sync>>,
);
