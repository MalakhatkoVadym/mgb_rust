use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Init,
    Running,
    Stopped,
}

#[derive(Clone)]
pub struct StateMachine {
    state: Arc<RwLock<State>>,
}

impl StateMachine {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(State::Init)),
        }
    }

    pub async fn get_state(&self) -> State {
        self.state.read().await.clone()
    }

    pub async fn set_state(&self, s: State) {
        let mut w = self.state.write().await;
        *w = s;
    }

    pub async fn start(&self) {
        self.set_state(State::Running).await;
    }

    pub async fn stop(&self) {
        self.set_state(State::Stopped).await;
    }
}
