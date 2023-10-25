use yaml_rust::Yaml;

use crate::util::yaml::YamlConversions;

use super::context::{AttributeType, Context};

/// returns the unused value
pub fn try_parse_group(ctx: &mut Context, key: &str, value: Yaml) -> Result<Option<Yaml>, String> {
    match key {
        "locale" => parse_attr_group(AttributeType::Locale, ctx, value)?,
        "display" => parse_attr_group(AttributeType::Display, ctx, value)?,
        "order" => parse_order_group(ctx, value)?,
        "visibility" => parse_visibility_group(ctx, value)?,
        _ => return Ok(Some(value)),
    };
    Ok(None)
}

pub fn parse_attr_group(
    attr_type: AttributeType,
    ctx: &mut Context,
    hash_or_attr: Yaml,
) -> Result<(), String> {
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
    Ok(())
}

pub fn parse_order_group(ctx: &mut Context, hash_or_attr: Yaml) -> Result<(), String> {
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
    Ok(())
}

pub fn parse_visibility_group(ctx: &mut Context, hash_or_attr: Yaml) -> Result<(), String> {
    if let Yaml::Hash(hash) = hash_or_attr {
        for (id, value) in hash {
            let id = id.einto_string()?;
            let value = value.einto_bool()?;

            if id == "_" {
                ctx.set_visibility(value)
            } else {
                ctx.override_visibility(id, value)
            }
        }
    } else {
        let value = hash_or_attr.einto_bool()?;
        ctx.set_visibility(value);
    }
    Ok(())
}
