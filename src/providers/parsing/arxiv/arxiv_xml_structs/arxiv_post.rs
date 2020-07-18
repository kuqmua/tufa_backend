use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ArxivPost {
    pub title: String,
    pub link: String,
    pub description: String,
    pub creators: Vec<Creator>,
}

impl Display for ArxivPost {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(
            fmt,
            "title = {}\nlink = {}\ndescription ={}\ncreators = {:#?}\n",
            self.title, self.link, self.description, self.creators
        )
    }
}

impl ArxivPost {
    pub fn new() -> Self {
        ArxivPost {
            title: "".to_string(),
            link: "".to_string(),
            description: "".to_string(),
            creators: Vec::<Creator>::new(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Creator {
    pub name: String,
    pub link: String,
}

impl Display for Creator {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "name = {}\nlink = {}\n", self.name, self.link)
    }
}

impl Creator {
    pub fn new() -> Self {
        Creator {
            name: "".to_string(),
            link: "".to_string(),
        }
    }
}
