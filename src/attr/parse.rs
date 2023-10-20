use yaml_rust::Yaml;

use crate::util::yaml::YamlConversions;

use super::context::{Context, AttributeType};

pub fn parse_attrs(attr_type: AttributeType, mut ctx: Context, hash_or_attr: Yaml) -> Result<Context, String> {
    if let Yaml::Hash(hash) = hash_or_attr {
        for (id, value) in hash {
            let id = id.einto_string()?;
            let value = value.einto_nullable_string()?;

            if id == "_" {
                ctx.set_attr(attr_type.clone(), value)
            } else {
                ctx.append_override(id, attr_type.clone(), value)
            }
        }
    } else {
        let value = hash_or_attr.einto_nullable_string()?;
        ctx.set_attr(attr_type, value);
    }
    Ok(ctx)
}