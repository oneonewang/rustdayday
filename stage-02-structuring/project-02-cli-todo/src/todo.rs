// todo.rs — Todo 业务逻辑

use crate::error::AppError;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Todo {
    pub id: u32,
    pub text: String,
    pub done: bool,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct TodoList {
    pub items: Vec<Todo>,
    next_id: u32,
}

impl TodoList {
    pub fn add(&mut self, text: String) {
        let id = if self.next_id == 0 { 1 } else { self.next_id };
        self.next_id = id + 1;
        self.items.push(Todo { id, text, done: false });
    }

    pub fn complete(&mut self, id: u32) -> Result<(), AppError> {
        let item = self.items.iter_mut().find(|t| t.id == id)
            .ok_or(AppError::NotFound(id))?;
        item.done = true;
        Ok(())
    }

    pub fn remove(&mut self, id: u32) -> Result<(), AppError> {
        let pos = self.items.iter().position(|t| t.id == id)
            .ok_or(AppError::NotFound(id))?;
        self.items.remove(pos);
        Ok(())
    }

    pub fn print(&self) {
        if self.items.is_empty() {
            println!("(空)");
            return;
        }
        for t in &self.items {
            let mark = if t.done { "[x]" } else { "[ ]" };
            println!("{} #{:<3} {}", mark, t.id, t.text);
        }
    }
}
