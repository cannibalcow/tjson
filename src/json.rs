pub mod json {
    use serde_json::Number;
    use serde_json::Value;

    #[derive(Debug, PartialEq, PartialOrd)]
    pub enum JsonValue {
        PosInt(i64),
        Float(f64),
        Text(String),
        Boolean(bool),
        Null,
    }

    impl ToString for JsonValue {
        fn to_string(&self) -> String {
            match self {
                JsonValue::PosInt(v) => v.to_string(),
                JsonValue::Float(v) => v.to_string(),
                JsonValue::Text(v) => v.to_string(),
                JsonValue::Boolean(v) => v.to_string(),
                JsonValue::Null => "Null".to_string(),
            }
        }
    }

    pub type Title = String;

    #[derive(Debug, PartialEq, PartialOrd)]
    pub struct JsonEntity {
        pub title: Title,
        pub value: JsonValue,
    }

    #[derive(Debug, PartialEq, PartialOrd)]
    pub enum EntityResult {
        Entities(Vec<JsonEntity>),
        Entity(JsonEntity),
    }

    pub fn get_cell(json_object: &Value, pointer: &str) -> Option<EntityResult> {
        let title = get_field_name(pointer);

        if title.is_none() {
            return None;
        }

        let t = title.unwrap().to_string();

        match json_object.pointer(pointer) {
            Some(v) => match v {
                Value::Null => Some(EntityResult::Entity(JsonEntity {
                    title: t,
                    value: JsonValue::Null,
                })),

                Value::Bool(b) => Some(EntityResult::Entity(JsonEntity {
                    title: t,
                    value: JsonValue::Boolean(b.clone()),
                })),
                Value::Number(n) => Some(EntityResult::Entity(JsonEntity {
                    title: t,
                    value: to_value(n),
                })),
                Value::String(s) => Some(EntityResult::Entity(JsonEntity {
                    title: t,
                    value: JsonValue::Text(s.clone()),
                })),
                // Todo: return list of cells with only primitives
                Value::Array(_) => None,
                Value::Object(v) => {
                    let mut result = Vec::new();
                    for (key, value) in v.iter() {
                        let n: Option<JsonEntity> = match value {
                            Value::Null => None,
                            Value::Bool(t) => Some(JsonEntity {
                                title: key.to_owned(),
                                value: JsonValue::Boolean(t.clone()),
                            }),
                            Value::Number(n) => Some(JsonEntity {
                                title: key.to_owned(),
                                value: to_value(n),
                            }),
                            Value::String(s) => Some(JsonEntity {
                                title: key.to_owned(),
                                value: JsonValue::Text(s.clone()),
                            }),
                            Value::Array(_) => None,
                            Value::Object(_) => None,
                        };
                        match n {
                            Some(v) => result.push(v),
                            None => {}
                        }
                    }
                    Some(EntityResult::Entities(result))
                }
            },
            None => None,
        }
    }

    fn to_value(number: &Number) -> JsonValue {
        if number.is_i64() || number.is_u64() {
            JsonValue::PosInt(number.as_i64().expect("Invalid number"))
        } else {
            JsonValue::Float(number.as_f64().expect("Invalid number"))
        }
    }

    fn get_field_name(point: &str) -> Option<&str> {
        point.split('/').last()
    }
}

#[cfg(test)]
mod tests {

    use serde_json::Value;

    use crate::json::json::{get_cell, EntityResult, JsonEntity, JsonValue};
    static JSON_STR: &str = "{ \"status\": { \"state\": \"running\", \"done\": 1234, \"float\": 3.14, \"negafloat\": -3.14, \"negative\": -123 } }";

    #[test]
    fn parse_str() {
        let jo: Value = serde_json::from_str(JSON_STR).unwrap();
        let str = get_cell(&jo, "/status/state");

        assert_eq!(
            str,
            Some(EntityResult::Entity(JsonEntity {
                title: String::from("state"),
                value: JsonValue::Text("running".to_string())
            }))
        );
    }

    #[test]
    fn parse_posint() {
        let jo: Value = serde_json::from_str(JSON_STR).unwrap();
        let posint = get_cell(&jo, "/status/done");

        assert_eq!(
            posint,
            Some(EntityResult::Entity(JsonEntity {
                title: String::from("done"),
                value: JsonValue::PosInt(1234)
            }))
        );
    }

    #[test]
    fn parse_float() {
        let jo: Value = serde_json::from_str(JSON_STR).unwrap();
        let floatpos = get_cell(&jo, "/status/float");

        assert_eq!(
            floatpos,
            Some(EntityResult::Entity(JsonEntity {
                title: String::from("float"),
                value: JsonValue::Float(3.14)
            }))
        );
    }

    #[test]
    fn parse_negfloat() {
        let jo: Value = serde_json::from_str(JSON_STR).unwrap();
        let negafloat = get_cell(&jo, "/status/negafloat");

        assert_eq!(
            negafloat,
            Some(EntityResult::Entity(JsonEntity {
                title: String::from("negafloat"),
                value: JsonValue::Float(-3.14)
            }))
        );
    }

    #[test]
    fn parse_negative() {
        let jo: Value = serde_json::from_str(JSON_STR).unwrap();
        let negative = get_cell(&jo, "/status/negative");

        assert_eq!(
            negative,
            Some(EntityResult::Entity(JsonEntity {
                title: String::from("negative"),
                value: JsonValue::PosInt(-123)
            }))
        );
    }
}
