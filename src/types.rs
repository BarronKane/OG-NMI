use std::fmt::Debug;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub name: String,
    pub role_id: String,
}

#[derive(Debug, Deserialize)]
struct ChaptersConfig {
    chapters: Vec<Chapter>,
}

pub struct Chapters {
    chapters: Vec<Chapter>,
    by_name: HashMap<String, usize>,
}

impl Chapters {
    pub fn load() -> Self {
        let file = std::fs::File::open("chapters.json").expect("chapters.json not found");
        let config: ChaptersConfig = serde_json::from_reader(file).expect("chapters.json not valid");
        
        let mut by_name = HashMap::new();
        
        for (index, chapter) in config.chapters.iter().enumerate() {
            by_name.insert(chapter.name.clone(), index);
        }
        
        Self {
            chapters: config.chapters,
            by_name,
        }
    }

    pub fn get_by_id(&self, id: u8) -> Option<&Chapter> {
        self.chapters.get(id as usize)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Chapter> {
        self.by_name.get(name).and_then(|&index| self.chapters.get(index))
    }

    pub fn get_id_by_name(&self, name: &str) -> Option<u8> {
        self.by_name.get(name).map(|&index| index as u8)
    }

    pub fn all(&self) -> &[Chapter] {
        &self.chapters
    }

    pub fn to_formatted_list(&self) -> String {
        let mut result = String::from("Available Chapters:\n\n");
        let mut num = 1;
        let num_pad = 3;
        
        for (id, chapter) in self.chapters.iter().enumerate() {
            let chars = chapter.name.chars().count();
            result.push_str(&format!("[{}] {}", id, chapter.name));
            let pad = 16 - chars;
            if pad > 0 && num % num_pad != 0 {
                result.push_str(&" ".repeat(pad));
            }
            if id < 10 {
                result.push_str(" ");
            }
            if (num % num_pad) == 0 {
                result.push_str("\n");
            }
            num += 1;
        }
        result
    }
}

impl Chapter {
    pub fn get_role_id(&self) -> &str {
        &self.role_id
    }
}


