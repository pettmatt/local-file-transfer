use serde::{Deserialize, Serialize};
use yew::Properties;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Properties, Serialize, Deserialize)]
pub struct File {
    pub id: usize,
    pub name: String
}
