use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue};

#[derive(pest_derive::Parser)]
#[grammar = "oudiasecond.pest"]
pub struct OuDiaSecondParser;

enum Structure<'a> {
    Struct(&'a str, Vec<Structure<'a>>),
    Pair(&'a str, Value<'a>),
}
enum Value<'a> {
    Single(&'a str),
    List(Vec<&'a str>),
}

fn parse_oud2(file: &str) -> Result<Structure<'_>, pest::error::Error<Rule>> {
    use pest::Parser;
    let oud2 = OuDiaSecondParser::parse(Rule::file, file)?
        .next()
        .unwrap()
        .into_inner()
        .next()
        .unwrap();
    use pest::iterators::Pair;
    fn parse_struct(pair: Pair<Rule>) -> Structure {
        match pair.as_rule() {
            Rule::r#struct => {
                let mut inner = pair.into_inner();
                let name = inner.next().unwrap().as_str();
                let mut fields = Vec::new();
                for field_pair in inner {
                    let field_struct = parse_struct(field_pair);
                    fields.push(field_struct);
                }
                Structure::Struct(name, fields)
            }
            Rule::wrapper => {
                let inner = pair.into_inner();
                let name = "file";
                let mut fields = Vec::new();
                for field_pair in inner {
                    let field_struct = parse_struct(field_pair);
                    fields.push(field_struct);
                }
                Structure::Struct(name, fields)
            }
            Rule::kvpair => {
                let mut inner = pair.into_inner();
                let key = inner.next().unwrap().as_str();
                let val = inner.next().unwrap();
                let val = match val.as_rule() {
                    Rule::value => Value::Single(val.as_str()),
                    Rule::list => {
                        let list_vals = val.into_inner().map(|v| v.as_str()).collect();
                        Value::List(list_vals)
                    }
                    _ => unreachable!(),
                };
                Structure::Pair(key, val)
            }
            _ => unreachable!(),
        }
    }
    Ok(parse_struct(oud2))
}

fn make_kdl(oud2_root: &Structure) -> String {
    fn to_kdl_node(node: &Structure) -> KdlNode {
        match node {
            Structure::Struct(name, fields) => {
                let mut kdl_node = KdlNode::new(*name);
                if !fields.is_empty() {
                    let mut children = KdlDocument::new();
                    for field in fields {
                        children.nodes_mut().push(to_kdl_node(field));
                    }
                    kdl_node.set_children(children);
                }
                kdl_node
            }
            Structure::Pair(key, value) => {
                let mut kdl_node = KdlNode::new(*key);
                let value = match value {
                    Value::Single(val) => &vec![*val],
                    Value::List(vals) => vals,
                };
                for val in value {
                    kdl_node.push(KdlEntry::new(KdlValue::String(val.trim().to_string())));
                }
                kdl_node
            }
        }
    }

    let mut document = KdlDocument::new();
    document.nodes_mut().push(to_kdl_node(oud2_root));
    document.autoformat();
    document.to_string()
}

pub fn convert(input: &str) -> Result<String, String> {
    let parsed = parse_oud2(input).map_err(|err| err.to_string())?;
    Ok(make_kdl(&parsed))
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::convert;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    pub fn convert_oud2_to_kdl(input: &str) -> Result<String, JsValue> {
        convert(input).map_err(|err| JsValue::from_str(&err))
    }
}
