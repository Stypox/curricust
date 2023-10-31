use yaml_rust::yaml::Hash;

use crate::attr::context::Context;

pub mod award_item;
pub mod education_item;
pub mod job_item;
pub mod project_item;
pub mod talk_item;

pub trait SectionItem
where
    Self: Sized,
{
    fn parse(ctx: &Context, hash: Hash) -> Result<Option<(i64, Self)>, String>;
}
