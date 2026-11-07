//! postfix model — mail transfer agent.

#[derive(Debug, Clone)]
pub struct Queue {
    pub id: String,
    pub sender: String,
    pub recipient: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub queue: Vec<Queue>,
}

impl Default for Model {
    fn default() -> Self { Self { queue: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn enqueue(&mut self, sender: &str, recipient: &str) -> String {
        let id = format!("{:x}", self.queue.len() + 1);
        self.queue.push(Queue { id: id.clone(), sender: sender.into(), recipient: recipient.into(), status: "pending".into() });
        id
    }

    pub fn deliver(&mut self, id: &str) -> bool {
        if let Some(q) = self.queue.iter_mut().find(|q| q.id == id) { q.status = "delivered".into(); true } else { false }
    }

    pub fn list(&self) -> &[Queue] { &self.queue }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn enqueue_deliver() {
        let mut p = Model::new();
        let id = p.enqueue("alice@example.com", "bob@example.com");
        assert!(p.deliver(&id));
        assert_eq!(p.list()[0].status, "delivered");
    }
}
