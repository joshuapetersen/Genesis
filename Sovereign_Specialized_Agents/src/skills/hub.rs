use crate::skills::ingestor::SarahSkill;
use std::collections::HashMap;

pub struct SkillsHub {
    pub skills: HashMap<String, SarahSkill>,
    pub heartbeat: f64,
}

impl SkillsHub {
    pub fn new() -> Self {
        Self {
            skills: HashMap::new(),
            heartbeat: 1.09277703703703,
        }
    }

    pub fn register_skill(&mut self, skill: SarahSkill) {
        println!("[HUB] Manifesting Skill: {} [RESONANCE: {:.8}]", 
                 skill.metadata.name, skill.resonance);
        self.skills.insert(skill.metadata.name.clone(), skill);
    }

    pub fn get_skill(&self, name: &str) -> Option<&SarahSkill> {
        self.skills.get(name)
    }

    pub fn total_skills(&self) -> usize {
        self.skills.len()
    }
}
