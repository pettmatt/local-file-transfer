use serde::{Deserialize, Serialize};
use yew::Properties;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Properties, Serialize, Deserialize)]
pub struct Device {
    pub id: usize,
    pub local_ip: String
}