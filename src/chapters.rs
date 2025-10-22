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

#[derive(Debug, Deserialize, Serialize)]
struct ChaptersConfig {
    chapters: Vec<Chapter>,
}

#[derive(Clone)]
pub struct Chapters {
    chapters: Vec<Chapter>,
    by_name: HashMap<String, usize>,
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
        let config: ChaptersConfig = serde_json::from_reader(file).expect("chapters.json not valid");
        
        let mut by_name = HashMap::new();
        
        for (index, chapter) in config.chapters.iter().enumerate() {
            by_name.insert(chapter.name.clone(), index);
        }
        
        let chapters = Self {
            chapters: config.chapters,
            by_name,
        };
        
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
        let config = ChaptersConfig {
            chapters: self.chapters.clone(),
        };
        
        let file = std::fs::File::create("chapters.json").expect("Could not create chapters.json");
        serde_json::to_writer_pretty(file, &config).expect("Could not write to chapters.json");
        
        // Update cache with new values.
        if let Ok(mut cache) = CHAPTERS_CACHE.write() {
            *cache = Some(self.clone());
        }
    }

    fn sort(&mut self) {
        self.chapters.sort_by(|a, b| a.name.cmp(&b.name));

        // Rebuild by_name map after sorting
        self.by_name.clear();
        for (index, chapter) in self.chapters.iter().enumerate() {
            self.by_name.insert(chapter.name.clone(), index);
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


