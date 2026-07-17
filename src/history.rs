#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Insert(usize, char),
    Delete(usize, char),
}

pub struct History {
    undo_stack: Vec<Action>,
    redo_stack: Vec<Action>,
}

impl History {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::with_capacity(1024),
            redo_stack: Vec::with_capacity(1024),
        }
    }

    pub fn record(&mut self, action: Action) {
        self.undo_stack.push(action);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self) -> Option<Action> {
        let action = self.undo_stack.pop()?;
        self.redo_stack.push(action);
        match action {
            Action::Insert(idx, c) => Some(Action::Delete(idx, c)),
            Action::Delete(idx, c) => Some(Action::Insert(idx, c)),
        }
    }

    pub fn redo(&mut self) -> Option<Action> {
        let action = self.redo_stack.pop()?;
        self.undo_stack.push(action);
        Some(action)
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}