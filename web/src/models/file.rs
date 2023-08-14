use yew::Properties;

#[derive(Clone, PartialEq, Properties)]
pub struct File {
    pub id: usize,
    pub name: String
}
