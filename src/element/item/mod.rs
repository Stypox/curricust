use yaml_rust::yaml::Hash;

use crate::attr::context::Context;

pub mod education_item;

pub trait SectionItem where Self: Sized {
    fn parse(hash: Hash, ctx: &Context) -> Result<Self, String>;
}
