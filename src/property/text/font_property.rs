use crate::{
    prelude::BevyCssError,
    property::{Property, PropertyValues},
};
use bevy::{
    ecs::query::QueryItem,
    prelude::{
        AssetServer,
        Commands,
        Node,
        With,
    }, text::TextFont,
};

/// Applies the `font` property on [`TextStyle::font`](`TextStyle`) property of all sections on matched [`Text`] components.
#[derive(Default)]
pub struct FontProperty;

impl Property
for FontProperty
{
    type Cache = String;
    type Components = &'static mut TextFont;
    type Filters = With<Node>;

    fn name(
        // no args
    ) -> &'static str {
        "font"
    }

    fn parse<'a>(
        values: &PropertyValues
    ) -> Result<Self::Cache, BevyCssError> {
        match values.string()
        {
            Some(path) => Ok(path),
            None => Err(BevyCssError::InvalidPropertyValue(Self::name().to_string()))
        }
    }

    fn apply<'w>(
        cache: &Self::Cache,
        mut components: QueryItem<Self::Components>,
        asset_server: &AssetServer,
        _commands: &mut Commands,
    ) {
        components.font = asset_server.load(cache);
    }
}
