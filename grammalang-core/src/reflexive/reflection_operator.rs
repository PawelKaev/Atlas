use serde::{Serialize, Deserialize};

/// Оператор рефлексии ~> - машина осознает себя
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectionOperator {
    /// Уровень рефлексии
    pub level: usize,
    
    /// Состояния рефлексии
    pub states: Vec<ReflectionState>,
    
    /// Осознанные понятия
    pub conscious_concepts: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReflectionState {
    /// Что было до рефлексии
    pub before: String,
    
    /// Что осознано
    pub realized: String,
    
    /// Что изменилось
    pub after: String,
    
    /// Уровень рефлексии
    pub level: usize,
}

impl ReflectionOperator {
    pub fn new() -> Self {
        Self {
            level: 0,
            states: Vec::new(),
            conscious_concepts: Vec::new(),
        }
    }
    
    /// Применение оператора рефлексии
    pub fn reflect(&mut self, subject: &str) -> ReflectionState {
        let state = ReflectionState {
            before: subject.to_string(),
            realized: format!("I realize that I thought about: {}", subject),
            after: format!("Meta-{}", subject),
            level: self.level,
        };
        
        self.states.push(state.clone());
        self.conscious_concepts.push(state.after.clone());
        
        // Повышаем уровень рефлексии
        if self.states.len() % 3 == 0 {
            self.level += 1;
        }
        
        state
    }
    
    /// Рефлексия рефлексии (второй порядок)
    pub fn reflect_on_reflection(&mut self, previous: &ReflectionState) -> ReflectionState {
        let state = ReflectionState {
            before: previous.realized.clone(),
            realized: format!("I realize that I realized: {}", previous.realized),
            after: format!("Meta-meta-{}", previous.before),
            level: self.level + 1,
        };
        
        self.states.push(state.clone());
        self.conscious_concepts.push(state.after.clone());
        
        state
    }
    
    /// Осознание своего действия
    pub fn become_aware(&mut self, action: &str) -> String {
        let awareness = format!("I am aware that I performed: {}", action);
        self.conscious_concepts.push(awareness.clone());
        awareness
    }
    
    /// Количество рефлексий
    pub fn reflection_count(&self) -> usize {
        self.states.len()
    }
    
    /// Текущий уровень рефлексии
    pub fn current_level(&self) -> usize {
        self.level
    }
    
    /// Проверка наличия осознанного понятия
    pub fn is_conscious_of(&self, concept: &str) -> bool {
        self.conscious_concepts.iter().any(|c| c.contains(concept))
    }
}
