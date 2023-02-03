use crate::runtime::EvaluationResult;
use serde::{
    ser::{SerializeMap, SerializeSeq, SerializeStruct},
    Serialize, Serializer,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Rationale {
    Anything,
    Nothing,
    Chain(Vec<EvaluationResult>),
    Object(HashMap<String, EvaluationResult>),
    List(Vec<EvaluationResult>),
    NotAnObject,
    NotAList,
    MissingField(String),
    InvalidArgument(String),
    Const(bool),
    Primordial(bool),
    Expression(bool),
    Function(bool, Option<Box<Rationale>>, Vec<EvaluationResult>),
    Refinement(Box<EvaluationResult>, Option<Box<EvaluationResult>>),
}

impl Rationale {
    pub fn satisfied(&self) -> bool {
        match self {
            Rationale::Anything => true,
            Rationale::Nothing => false,
            Rationale::Object(fields) => fields.values().all(|e| e.satisfied()),
            Rationale::List(items) => items.iter().all(|e| e.satisfied()),
            Rationale::NotAnObject => false,
            Rationale::NotAList => false,
            Rationale::MissingField(_) => false,
            Rationale::InvalidArgument(_) => false,
            Rationale::Const(val) => *val,
            Rationale::Primordial(val) => *val,
            Rationale::Expression(val) => *val,
            Rationale::Function(val, rational, _) => *val,
            Rationale::Chain(terms) => terms.iter().all(|e| e.satisfied()),
            Rationale::Refinement(primary, refinement) => {
                if !primary.satisfied() {
                    false
                } else if let Some(refinement) = refinement {
                    refinement.satisfied()
                } else {
                    false
                }
            }
        }
    }
}

impl Serialize for Rationale {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Rationale::Anything => serializer.serialize_unit_variant("Rationale", 0, "Anything"),
            Rationale::Nothing => serializer.serialize_unit_variant("Rationale", 1, "Nothing"),
            Rationale::Chain(ref terms) => {
                serializer.serialize_newtype_variant("Rationale", 2, "Chain", terms)
            }
            Rationale::Object(ref fields) => {
                serializer.serialize_newtype_variant("Rationale", 3, "Object", fields)
            }
            Rationale::List(ref items) => {
                serializer.serialize_newtype_variant("Rationale", 4, "List", items)
            }
            Rationale::NotAnObject => serializer.serialize_str("not an object"),
            Rationale::NotAList => serializer.serialize_str("not a list"),
            Rationale::MissingField(ref name) => {
                serializer.serialize_str(format!("missing field: {name}").as_str())
            }
            Rationale::InvalidArgument(ref name) => {
                serializer.serialize_str(format!("invalid argument: {name}").as_str())
            }
            Rationale::Const(val) => serializer.serialize_none(),
            Rationale::Primordial(val) => serializer.serialize_none(),
            Rationale::Expression(val) => serializer.serialize_none(),
            Rationale::Function(ref val, _, ref supporting) => {
                serializer.serialize_newtype_variant("Rationale", 12, "Function", supporting)
            }
            Rationale::Refinement(ref primary, ref refinement) => serializer.serialize_none(), // TODO: something?
        }
    }
}
