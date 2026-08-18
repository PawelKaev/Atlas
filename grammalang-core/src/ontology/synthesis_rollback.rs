use super::synthesis_integrator::*;

/// Механизм отката синтеза
#[derive(Debug, Clone)]
pub struct SynthesisRollback {
    /// История снапшотов машины
    pub history: Vec<MachineSnapshot>,
}

#[derive(Debug, Clone)]
pub struct MachineSnapshot {
    pub nodes: Vec<MachineNode>,
    pub edges: Vec<Edge>,
    pub metrics: MachineMetrics,
}

impl SynthesisRollback {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
        }
    }
    
    /// Создание снапшота
    pub fn snapshot(&mut self, machine: &MachineState) {
        self.history.push(MachineSnapshot {
            nodes: machine.nodes.clone(),
            edges: machine.edges.clone(),
            metrics: machine.metrics.clone(),
        });
    }
    
    /// Откат к последнему снапшоту
    pub fn rollback(&mut self, machine: &mut MachineState) -> Result<(), RollbackError> {
        if let Some(snapshot) = self.history.pop() {
            machine.nodes = snapshot.nodes;
            machine.edges = snapshot.edges;
            machine.metrics = snapshot.metrics;
            Ok(())
        } else {
            Err(RollbackError::NoSnapshot)
        }
    }
    
    /// Откат к конкретному снапшоту
    pub fn rollback_to(&mut self, machine: &mut MachineState, index: usize) -> Result<(), RollbackError> {
        if index < self.history.len() {
            let snapshot = &self.history[index];
            machine.nodes = snapshot.nodes.clone();
            machine.edges = snapshot.edges.clone();
            machine.metrics = snapshot.metrics.clone();
            
            // Удаляем снапшоты после index
            self.history.truncate(index);
            
            Ok(())
        } else {
            Err(RollbackError::InvalidIndex(index))
        }
    }
    
    /// Очистка истории
    pub fn clear(&mut self) {
        self.history.clear();
    }
    
    /// Количество снапшотов
    pub fn len(&self) -> usize {
        self.history.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.history.is_empty()
    }
}

#[derive(Debug, Clone)]
pub enum RollbackError {
    NoSnapshot,
    InvalidIndex(usize),
}
