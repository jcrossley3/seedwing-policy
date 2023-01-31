use seedwing_policy_engine::runtime::{rationale::Rationale, EvaluationResult, TypeName};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct Result {
    name: Option<TypeName>,
    input: serde_json::Value,
    satisfied: bool,
    reason: String,
    rationale: Vec<String>,
}

impl Result {
    pub fn new(result: &EvaluationResult) -> Self {
        let input = match result.input() {
            Some(input) => input.as_json(),
            None => serde_json::Value::Null,
        };

        Self {
            name: result.ty().name(),
            input,
            satisfied: result.satisfied(),
            reason: reason(result.rationale()),
            rationale: support(result.rationale()),
        }
    }
}

fn reason(rationale: &Rationale) -> String {
    match rationale {
        Rationale::Anything => "anything is satisfied by anything".to_string(),
        Rationale::Nothing
        | Rationale::Const(_)
        | Rationale::Primordial(_)
        | Rationale::Expression(_) => "".to_string(),
        Rationale::Object(_) => if rationale.satisfied() {
            "because all fields were satisfied"
        } else {
            "because not all fields were satisfied"
        }
        .to_string(),
        Rationale::List(_terms) => if rationale.satisfied() {
            "because all members were satisfied"
        } else {
            "because not all members were satisfied"
        }
        .to_string(),
        Rationale::Chain(_terms) => if rationale.satisfied() {
            "because the chain was satisfied"
        } else {
            "because the chain was not satisfied"
        }
        .to_string(),
        Rationale::NotAnObject => "not an object".to_string(),
        Rationale::NotAList => "not a list".to_string(),
        Rationale::MissingField(name) => format!("missing field: {name}"),
        Rationale::InvalidArgument(name) => format!("invalid argument: {name}"),
        Rationale::Function(_, _, _) => String::new(),
        Rationale::Refinement(_primary, _refinement) => todo!(),
    }
}

fn support(rationale: &Rationale) -> Vec<String> {
    let mut result = vec![reason(rationale)];
    let mut v = match rationale {
        Rationale::Object(fields) => fields
            .iter()
            .flat_map(|(_, r)| support(r.rationale()))
            .collect(),
        Rationale::List(terms) | Rationale::Chain(terms) | Rationale::Function(_, _, terms) => {
            terms.iter().flat_map(|r| support(r.rationale())).collect()
        }
        Rationale::Anything
        | Rationale::Nothing
        | Rationale::NotAnObject
        | Rationale::NotAList
        | Rationale::MissingField(_)
        | Rationale::InvalidArgument(_)
        | Rationale::Const(_)
        | Rationale::Primordial(_)
        | Rationale::Expression(_) => Vec::new(),
        Rationale::Refinement(_primary, _refinement) => {
            todo!();
        }
    };
    result.append(&mut v);
    result
}
