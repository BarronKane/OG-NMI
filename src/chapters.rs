use std::fmt::Debug;
use std::collections::HashMap;
use std::sync::RwLock;
use serde::{Deserialize, Serialize};

static CHAPTERS_CACHE: RwLock<Option<Chapters>> = RwLock::new(None);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub name: String,
    pub role_id: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapters {
    pub chapters: Vec<Chapter>,
}

impl Chapters {
    pub fn load() -> Self {
        // Try to read from cache first.
        if let Ok(cache) = CHAPTERS_CACHE.read() {
            if let Some(chapters) = cache.as_ref() {
                return chapters.clone();
            }
        }

        // Cache miss - load from disk.
        let file = std::fs::File::open("chapters.json").expect("chapters.json not found");
        let chapters: Chapters = serde_json::from_reader(file).expect("chapters.json not valid");

        // Update cache.
        if let Ok(mut cache) = CHAPTERS_CACHE.write() {
            *cache = Some(chapters.clone());
        }

        chapters
    }

    pub fn add_chapter(&mut self, chapter: Chapter) {
        self.chapters.push(chapter);
        self.sort();
        self.save();
    }

    pub fn remove_chapter(&mut self, id: u8) {
        self.chapters.remove(id as usize);
        self.save();
    }

    pub fn save(&mut self) {
        let file = std::fs::File::create("../chapters.json").expect("Could not create chapters.json");
        serde_json::to_writer_pretty(file, &self.chapters).expect("Could not write to chapters.json");

        // Update cache with new values.
        if let Ok(mut cache) = CHAPTERS_CACHE.write() {
            *cache = Some(self.clone());
        }
    }

    fn sort(&mut self) {
        self.chapters.sort_by(|a, b| a.name.cmp(&b.name));
    }

    pub fn get_by_id(&self, id: usize) -> Option<&Chapter> {
        self.chapters.get(id)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Chapter> {
        //self.by_name.get(name).and_then(|&index| self.chapters.get(index))
        self.chapters.iter().find(|chapter| chapter.name == name)
    }

    pub fn get_id_by_name(&self, name: &str) -> Option<u8> {
        //self.by_name.get(name).map(|&index| index as u8)
        self.chapters.iter().position(|chapter| chapter.name == name).map(|index| index as u8)
    }

    pub fn get_count(&self) -> usize {
        self.chapters.len()
    }

    pub fn all(&self) -> &[Chapter] {
        &self.chapters
    }

    pub fn to_formatted_list(&self) -> String {
        let mut result = String::from("Available Chapters:\n\n```");
        let mut num = 1;
        let num_pad = 2;

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
        result.push_str("\n```");
        result
    }
}

impl Chapter {
    pub fn get_role_id(&self) -> u64 {
        self.role_id.clone()
    }
}


