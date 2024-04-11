use chrono::{DateTime, NaiveTime, Utc};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::convert::Into;

use crate::types;
use crate::types::GateState;

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ApiInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Group {
    pub name: String,
    pub services: Vec<Service>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    pub environments: Vec<Environment>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Environment {
    pub name: String,
    pub gate: Gate,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Gate {
    pub group: String,
    pub service: String,
    pub environment: String,
    pub state: GateState,
    pub comments: Vec<Comment>,
    pub last_updated: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_order: Option<u32>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GateStateRep {
    pub state: GateState,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub message: String,
    pub created: DateTime<Utc>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub system_time: DateTime<Utc>,
    pub active_hours_per_week: ActiveHoursPerWeek,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActiveHours {
    pub start: NaiveTime,
    pub end: NaiveTime,
}

impl From<types::ActiveHours> for ActiveHours {
    fn from(value: types::ActiveHours) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActiveHoursPerWeek {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monday: Option<ActiveHours>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tuesday: Option<ActiveHours>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wednesday: Option<ActiveHours>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thursday: Option<ActiveHours>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friday: Option<ActiveHours>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saturday: Option<ActiveHours>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sunday: Option<ActiveHours>,
}

impl From<types::ActiveHoursPerWeek> for ActiveHoursPerWeek {
    fn from(value: types::ActiveHoursPerWeek) -> Self {
        Self {
            monday: value.monday.map(Into::into),
            tuesday: value.tuesday.map(Into::into),
            wednesday: value.wednesday.map(Into::into),
            thursday: value.thursday.map(Into::into),
            friday: value.friday.map(Into::into),
            saturday: value.saturday.map(Into::into),
            sunday: value.sunday.map(Into::into),
        }
    }
}

impl From<types::Gate> for Gate {
    fn from(value: types::Gate) -> Self {
        Self {
            group: value.key.group,
            service: value.key.service,
            environment: value.key.environment,
            state: value.state,
            comments: value
                .comments
                .into_iter()
                .map_into::<Comment>()
                .sorted_by_key(|comment| comment.created)
                .collect(),
            last_updated: value.last_updated,
            display_order: value.display_order,
        }
    }
}
impl From<types::Gate> for GateStateRep {
    fn from(value: types::Gate) -> Self {
        Self { state: value.state }
    }
}

impl From<types::Comment> for Comment {
    fn from(value: types::Comment) -> Self {
        Self {
            id: value.id,
            message: value.message,
            created: value.created,
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use crate::types;
    use crate::types::representation::{Comment, Gate};
    use chrono::DateTime;
    use std::collections::HashSet;

    #[test]
    fn should_convert_domain_gate_to_representation() {
        let gate = some_gate("some-group", "some-service", "some-environment");
        let actual: Gate = gate.into();
        let expected = Gate {
            group: "some-group".to_owned(),
            service: "some-service".to_owned(),
            environment: "some-environment".to_owned(),
            state: types::GateState::Open,
            comments: vec![
                Comment {
                    id: "Comment1".into(),
                    message: "Some comment message".into(),
                    created: DateTime::parse_from_rfc3339("2021-04-12T20:10:57Z")
                        .expect("can not convert date")
                        .into(),
                },
                Comment {
                    id: "Comment2".into(),
                    message: "Some other comment message".into(),
                    created: DateTime::parse_from_rfc3339("2022-04-12T20:10:57Z")
                        .expect("can not convert date")
                        .into(),
                },
            ],
            last_updated: DateTime::parse_from_rfc3339("2023-04-12T22:10:57+02:00")
                .expect("can not convert date")
                .into(),
            display_order: Option::default(),
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn should_convert_comment() {
        let actual: Comment = types::Comment {
            id: "1234".to_string(),
            message: "Gate closed because of ticket #63468".to_owned(),
            created: DateTime::parse_from_rfc3339("2023-04-12T22:10:57+02:00")
                .expect("can not convert date")
                .into(),
        }
        .into();

        let expected = Comment {
            id: "1234".to_string(),
            message: "Gate closed because of ticket #63468".to_owned(),
            created: DateTime::parse_from_rfc3339("2023-04-12T22:10:57+02:00")
                .expect("can not convert date")
                .into(),
        };
        assert_eq!(actual, expected);
    }

    fn some_gate(group: &str, service: &str, environment: &str) -> types::Gate {
        types::Gate {
            key: types::GateKey {
                group: group.to_owned(),
                service: service.to_owned(),
                environment: environment.to_owned(),
            },
            state: types::GateState::Open,
            comments: HashSet::from([
                types::Comment {
                    id: "Comment1".to_owned(),
                    message: "Some comment message".to_owned(),
                    created: DateTime::parse_from_rfc3339("2021-04-12T22:10:57+02:00")
                        .expect("failed creating date")
                        .into(),
                },
                types::Comment {
                    id: "Comment2".to_owned(),
                    message: "Some other comment message".to_owned(),
                    created: DateTime::parse_from_rfc3339("2022-04-12T22:10:57+02:00")
                        .expect("failed creating date")
                        .into(),
                },
            ]),
            last_updated: DateTime::parse_from_rfc3339("2023-04-12T22:10:57+02:00")
                .expect("failed creating date")
                .into(),
            display_order: Option::default(),
        }
    }
}