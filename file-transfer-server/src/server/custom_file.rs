use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct FileObject {
    pub name: String,
    pub content: String
}

impl FileObject {
    /// Createa a new File struct.
    /// 
    /// Name references to the name of the file and content to the value
    /// 
    /// # Panics
    /// 
    /// The `new` function will panic if either the name or content is empty.
    pub fn new(name: String, content: String) -> FileObject {
        assert!(!name.is_empty() || !content.is_empty());
        FileObject { name, content }
    }

    pub fn serialize(&self) -> String {
        let serialized_string = serde_json::to_string(self).expect("Serializing failed");
        serialized_string
    }

    pub fn parse(serialized: &str) -> FileObject {
        let parsed_string = serde_json::from_str(serialized).expect("Converting string to File failed");
        parsed_string
    }
}