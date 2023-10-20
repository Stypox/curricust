use yaml_rust::Yaml;

use crate::util::yaml::YamlConversions;

use super::context::{Context, AttributeType};

pub fn parse_attrs(attr_type: AttributeType, mut ctx: Context, hash_or_attr: Yaml) -> Result<Context, String> {
    if let Yaml::Hash(hash) = hash_or_attr {
        for (id, value) in hash {
            let id = id.einto_string()?;
            let value = value.einto_nullable_string()?;

            if id == "_" {
                ctx.set_attr(attr_type, value)
            } else {
                ctx.override_attr(id, attr_type, value)
            }
        }
    } else {
        let value = hash_or_attr.einto_nullable_string()?;
        ctx.set_attr(attr_type, value);
    }
    Ok(ctx)
}

pub fn parse_order(mut ctx: Context, hash_or_attr: Yaml) -> Result<Context, String> {
    if let Yaml::Hash(hash) = hash_or_attr {
        for (id, value) in hash {
            let id = id.einto_string()?;
            let value = value.einto_nullable_int()?;

            if id == "_" {
                ctx.set_order(value)
            } else {
                ctx.override_order(id, value)
            }
        }
    } else {
        let value = hash_or_attr.einto_nullable_int()?;
        ctx.set_order(value);
    }
    Ok(ctx)
}