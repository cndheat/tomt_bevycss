use super::CacheState;
use crate::selector::Selector;

use bevy::{platform::collections::HashMap, prelude::{Deref, DerefMut}};

/// Internal cache map. Used by [`crate::property::PropertyMeta`] to keep track of which properties was already parsed.
#[derive(Debug, Default, Deref, DerefMut)]
pub struct CachedProperties<T>(HashMap<Selector, CacheState<T>>);
