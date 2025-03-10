use std::sync::{Arc, Condvar, Mutex, Weak};
use std::thread::{spawn, JoinHandle};

#[derive(Clone)]
pub enum Step {
    Read(usize),
    Swap(usize, usize),
}

pub struct Interface {
    state: Weak<(Mutex<State>, Condvar)>,
}

impl Interface {
    pub fn new(state: Weak<(Mutex<State>, Condvar)>) -> Interface {
        Self { state }
    }

    fn modify_state<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut State) -> T,
    {
        if let Some(state) = self.state.upgrade() {
            let (state, cvar) = &*state;
            let mut state = state.lock().unwrap();
            state = cvar.wait(state).unwrap();
            (f)(&mut state)
        } else {
            panic!("sorter stopped, terminating thread");
        }
    }

    pub fn read(&self, i: usize) -> u32 {
        self.modify_state(|state| {
            state.step = Some(Step::Read(i));
            state.data[i]
        })
    }

    pub fn swap(&self, i: usize, j: usize) {
        self.modify_state(|state| {
            state.step = Some(Step::Swap(i, j));
            state.data.swap(i, j);
        })
    }

    pub fn len(&self) -> usize {
        self.modify_state(|state| state.data.len())
    }
}

pub struct Method {
    pub name: &'static str,
    pub func: fn(Interface),
}

#[derive(Clone)]
pub struct State {
    pub sorting: bool,
    pub data: Vec<u32>,
    pub step: Option<Step>,
}

pub struct Sorter {
    pub state: Arc<(Mutex<State>, Condvar)>,
    pub method: Option<fn(Interface)>,
    pub thread: Option<JoinHandle<()>>,
}

impl Sorter {
    pub fn new(data: Vec<u32>) -> Self {
        let state = Arc::new((
            Mutex::new(State {
                sorting: false,
                data,
                step: None,
            }),
            Condvar::new(),
        ));

        Self {
            state,
            method: None,
            thread: None,
        }
    }

    pub fn start(&mut self) {
        let state = self.state.clone();
        {
            let (state1, _) = &*state.clone();
            let mut state1 = state1.lock().unwrap();

            if state1.sorting {
                return;
            }
            state1.sorting = true;
        }

        if let Some(method) = self.method {
            self.thread = Some(spawn(move || {
                let state2 = Arc::downgrade(&state);
                drop(state);

                method(Interface::new(state2.clone()));

                let (state2, _) = &*state2.upgrade().unwrap();
                let mut state2 = state2.lock().unwrap();
                state2.sorting = false;
                state2.step = None;
            }));
        } else {
            panic!("No method");
        }
    }

    pub fn stop(&mut self) {
        let state_clone = self.state.clone();
        let (state, cvar) = &*state_clone;
        let mut state = state.lock().unwrap();
        state.sorting = false;
        state.step = None;
        self.state = Arc::new((Mutex::new(state.clone()), Condvar::new()));
        cvar.notify_one();
    }
}
