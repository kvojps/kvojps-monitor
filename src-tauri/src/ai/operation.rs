//! Volatile, closed operational vocabulary shared by the two AI providers.
use serde::Serialize;

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum Origin {
    User,
    Automatic,
    Resume,
}
#[derive(Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) enum State {
    Running,
    Succeeded,
    Failed,
    Recovered,
    Paused,
    Unavailable,
}
#[derive(Serialize)]
pub(super) struct Operation {
    id: u64,
    origin: Origin,
    state: State,
}
#[derive(Default)]
pub(super) struct Operations {
    pub current: Option<Operation>,
    next_id: u64,
    resume: bool,
    failed: bool,
}
impl Operations {
    fn record(&mut self, origin: Origin, state: State) {
        self.next_id += 1;
        self.current = Some(Operation {
            id: self.next_id,
            origin,
            state,
        });
    }
    pub fn begin(&mut self, manual: bool) {
        let origin = if manual {
            Origin::User
        } else if self.resume {
            Origin::Resume
        } else {
            Origin::Automatic
        };
        self.resume = false;
        self.record(origin, State::Running);
    }
    pub fn finish(&mut self, state: State) {
        if let Some(op) = &mut self.current {
            op.state = state;
        }
        self.failed |= state == State::Unavailable;
    }
    pub fn success(&mut self) {
        self.finish(if self.failed {
            State::Recovered
        } else {
            State::Succeeded
        });
        self.failed = false;
    }
    pub fn failure(&mut self) {
        self.finish(State::Failed);
        self.failed = true;
    }
    pub fn pause(&mut self) {
        self.resume = true;
        if !self
            .current
            .as_ref()
            .is_some_and(|op| op.state == State::Paused)
        {
            self.record(Origin::Automatic, State::Paused);
        }
    }
    pub fn unavailable(&mut self) {
        if !self
            .current
            .as_ref()
            .is_some_and(|op| op.state == State::Unavailable)
        {
            self.record(Origin::Automatic, State::Unavailable);
        }
        self.failed = true;
    }
    pub fn clear(&mut self) {
        self.current = None;
        self.failed = false;
        self.resume = false;
    }
    pub fn discard(&mut self) {
        self.current = None;
        self.resume = true;
    }
}
