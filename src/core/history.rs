use std::collections::VecDeque;
use crate::core::parameter::{Parameter, ParameterValue};

#[derive(Debug)]
pub enum Command {
    SetParameter {
        param_id: String,
        old_value: ParameterValue,
        new_value: ParameterValue,
    },
    // Other command types (e.g., node graph changes) can be added here
}

pub struct HistoryManager {
    undo_stack: VecDeque<Command>,
    redo_stack: VecDeque<Command>,
    max_history: usize,
}

impl Default for HistoryManager {
    fn default() -> Self {
        Self {
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
            max_history: 50,
        }
    }
}

impl HistoryManager {
    pub fn push(&mut self, command: Command) {
        self.undo_stack.push_back(command);
        if self.undo_stack.len() > self.max_history {
            self.undo_stack.pop_front();
        }
        self.redo_stack.clear(); // Clear redo on new action
    }

    pub fn undo(&mut self, params: &mut Vec<Parameter>) -> Option<()> {
        let command = self.undo_stack.pop_back()?;
        
        match &command {
            Command::SetParameter { param_id, old_value, .. } => {
                if let Some(param) = params.iter_mut().find(|p| p.id == *param_id) {
                    param.value = old_value.clone();
                }
            }
        }
        
        self.redo_stack.push_back(command);
        Some(())
    }

    pub fn redo(&mut self, params: &mut Vec<Parameter>) -> Option<()> {
        let command = self.redo_stack.pop_back()?;
        
        match &command {
            Command::SetParameter { param_id, new_value, .. } => {
                if let Some(param) = params.iter_mut().find(|p| p.id == *param_id) {
                    param.value = new_value.clone();
                }
            }
        }
        
        self.undo_stack.push_back(command);
        Some(())
    }
}
