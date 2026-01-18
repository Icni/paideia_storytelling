#[derive(Debug, Default)]
pub struct CutoutDomain {
    pub events: Vec<String>,
    pub story_length: u32,
}

#[derive(Debug, Default)]
pub struct CutoutStory(pub String);

impl CutoutDomain {
    pub fn generate_story(&self) -> Option<CutoutStory> {
        let mut story = String::new();

        for _ in 0..self.story_length {
            let rand_event = rand::random_range(0..self.events.len());
            story += &self.events[rand_event];
            story.push('\n');
        }

        Some(CutoutStory(story))
    }
}

