use yew::Properties;

#[derive(Clone, PartialEq, Properties)]
pub struct Device {
    pub id: usize,
    pub local_ip: String
}
