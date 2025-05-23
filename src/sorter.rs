use crate::GLOBAL_STATE;
use std::sync::{Arc, Mutex, Weak};
use std::thread;
use std::thread::{spawn, JoinHandle};
use std::time::Instant;

#[derive(Clone)]
pub enum Step {
    Read(usize),
    Swap(usize, usize),
}

pub struct Interface {
    state: Weak<Mutex<State>>,
}

impl Interface {
    pub fn new(state: Weak<Mutex<State>>) -> Interface {
        Self { state }
    }

    fn modify_state<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut State) -> T,
    {
        if let Some(state) = self.state.upgrade() {
            let global_state = GLOBAL_STATE.lock().unwrap();
            if global_state.paused {
                drop(global_state);
                thread::park();
            } else {
                let delay = global_state.delay;
                drop(global_state);
                thread::sleep(std::time::Duration::from_micros(delay));
            }

            let mut state = state.lock().unwrap();
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
    pub start_time: Option<Instant>,
    pub stop_time: Option<Instant>,
}

pub struct Sorter {
    pub state: Arc<Mutex<State>>,
    pub method: Option<fn(Interface)>,
    thread: Option<JoinHandle<()>>
}

impl Sorter {
    pub fn new(data: Vec<u32>) -> Self {
        let state = Arc::new(Mutex::new(State {
            sorting: false,
            data,
            step: None,
            start_time: None,
            stop_time: None,
        }));

        Self {
            state,
            method: None,
            thread: None,
        }
    }

    pub fn is_sorting(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.sorting
    }

    pub fn start(&mut self, track: bool) {
        let state = self.state.clone();
        {
            let state1 = state.clone();
            let mut state1 = state1.lock().unwrap();

            if state1.sorting {
                return;
            }
            state1.sorting = true;
            if track {
                state1.start_time = Some(Instant::now());
            }
        }

        if let Some(method) = self.method {
            self.thread = Some(spawn(move || {
                let state2 = Arc::downgrade(&state);
                drop(state);

                method(Interface::new(state2.clone()));

                let state2 = state2.upgrade().unwrap();
                let mut state2 = state2.lock().unwrap();
                state2.sorting = false;
                state2.step = None;
                if track {
                    state2.stop_time = Some(Instant::now());
                }
            }));
        } else {
            panic!("No method");
        }
    }

    pub fn resume(&self) {
        if let Some(thread) = &self.thread {
            thread.thread().unpark();
        }
    }

    pub fn stop(&mut self) {
        let state_clone = self.state.clone();
        let state = state_clone;
        let mut state = state.lock().unwrap();
        state.sorting = false;
        state.step = None;
        self.state = Arc::new(Mutex::new(state.clone()));
        if let Some(thread) = &self.thread {
            thread.thread().unpark();
        }
    }
}
