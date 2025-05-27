use std::str::FromStr;

use sea_query::{Alias, SimpleExpr, Value};
use serde::Serialize;
use uuid::Uuid;

use crate::model::{chat_role::ChatRoleEnum, role::RoleEnum, token::TokenTypeEnum};

pub fn prepare_sea_query_fields(fields: Vec<(String, Value)>) -> (Vec<Alias>, Vec<SimpleExpr>) {
    let columns = fields
        .iter()
        .map(|(key, _)| Alias::new(key))
        .collect::<Vec<_>>();
    let values = fields
        .iter()
        .map(|(_, value)| {
            // cast to custom enum
            match value {
                Value::String(Some(value_str)) => {
                    if let Ok(_) = TokenTypeEnum::from_str(value_str) {
                        SimpleExpr::Value(value.to_owned()).cast_as(Alias::new("token_type_enum"))
                    } else if let Ok(_) = RoleEnum::from_str(value_str) {
                        SimpleExpr::Value(value.to_owned()).cast_as(Alias::new("role_enum"))
                    } else if let Ok(_) = ChatRoleEnum::from_str(value_str) {
                        SimpleExpr::Value(value.to_owned()).cast_as(Alias::new("chat_role_enum"))
                    } else {
                        SimpleExpr::Value(value.to_owned())
                    }
                }
                // В других случаях, возвращаем оригинальное значение
                _ => SimpleExpr::Value(value.to_owned()),
            }
        })
        .collect::<Vec<_>>();

    (columns, values)
}

pub fn struct_to_vec<T: Serialize>(instance: &T) -> Vec<(String, Value)> {
    serde_json::to_value(instance)
        .unwrap()
        .as_object()
        .unwrap()
        .iter()
        // .map(|(key, value)| {
        //     let value_str = serde_value_to_sea(value);
        //     (key.clone(), value_str)
        // })
        .filter_map(|(key, value)| {
            if !value.is_null() {
                Some((key.clone(), serde_value_to_sea(value)))
            } else {
                None
            }
        })
        .collect()
}

fn serde_value_to_sea(value: &serde_json::Value) -> Value {
    match value {
        serde_json::Value::String(s) => {
            if let Ok(uuid) = Uuid::from_str(s) {
                Value::Uuid(Some(Box::new(uuid)))
            // } else if let Ok(token_type) = TokenTypeEnum::from_str(s) {
            //     Value::String(Some(Box::new(token_type.as_str().to_owned())))
            } else {
                Value::String(Some(Box::new(s.to_owned())))
            }
        }
        serde_json::Value::Bool(b) => Value::Bool(Some(*b)),
        serde_json::Value::Number(n) => {
            if let Some(v) = n.as_i64() {
                if v >= i8::MIN as i64 && v <= i8::MAX as i64 {
                    Value::TinyInt(Some(v as i8))
                } else if v >= i16::MIN as i64 && v <= i16::MAX as i64 {
                    Value::SmallInt(Some(v as i16))
                } else if v >= i32::MIN as i64 && v <= i32::MAX as i64 {
                    Value::Int(Some(v as i32))
                } else {
                    Value::BigInt(Some(v))
                }
            } else if let Some(v) = n.as_u64() {
                if v <= u8::MAX as u64 {
                    Value::TinyUnsigned(Some(v as u8))
                } else if v <= u16::MAX as u64 {
                    Value::SmallUnsigned(Some(v as u16))
                } else if v <= u32::MAX as u64 {
                    Value::Unsigned(Some(v as u32))
                } else {
                    Value::BigUnsigned(Some(v))
                }
            } else if let Some(v) = n.as_f64() {
                if v as f32 as f64 == v {
                    Value::Float(Some(v as f32))
                } else {
                    Value::Double(Some(v))
                }
            } else {
                Value::String(None)
            }
        }
        _ => Value::String(None),
    }
}

#[cfg(test)]
mod test {
    use serde::Serialize;

    use crate::db::utils::struct_to_vec;

    #[derive(Serialize)]
    struct TestStruct {
        field1: Option<String>,
        field2: Option<String>,
    }

    #[tokio::test]
    async fn test_struct_to_vec() -> anyhow::Result<()> {
        let test_struct = TestStruct {
            field1: Some("value1".to_string()),
            field2: None,
        };

        let result = struct_to_vec(&test_struct);
        print!("{}", result.len());
        print!("{:?}", result);

        Ok(())
    }
}
