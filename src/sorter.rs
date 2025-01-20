use std::sync::{Arc, Condvar, Mutex};
use std::thread::{spawn, JoinHandle};

pub enum Step {
    Read(usize),
    Swap(usize, usize),
}

pub struct Interface {
    state: Arc<(Mutex<State>, Condvar)>,
}

impl Interface {
    pub fn new(state: Arc<(Mutex<State>, Condvar)>) -> Interface {
        Self { state }
    }

    pub fn read(&self, i: usize) -> u32 {
        let (state, cvar) = &*self.state;
        let mut state = state.lock().unwrap();
        state = cvar.wait(state).unwrap();
        state.step = Some(Step::Read(i));
        state.data[i]
    }

    pub fn swap(&self, i: usize, j: usize) {
        let (state, cvar) = &*self.state;
        let mut state = state.lock().unwrap();
        state = cvar.wait(state).unwrap();
        state.step = Some(Step::Swap(i, j));
        state.data.swap(i, j);
    }

    pub fn len(&self) -> usize {
        let (state, _) = &*self.state;
        state.lock().unwrap().data.len()
    }
}

type Method = fn(Interface);

pub struct State {
    pub sorting: bool,
    pub data: Vec<u32>,
    pub step: Option<Step>,
}

pub struct Sorter {
    pub state: Arc<(Mutex<State>, Condvar)>,
    method: Method,
    pub thread: Option<JoinHandle<()>>,
}

impl Sorter {
    pub fn new(data: Vec<u32>, method: Method) -> Self {
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
            method,
            thread: None,
        }
    }

    pub fn start(&mut self) {
        let state = self.state.clone();
        let (state1, _) = &*state.clone();
        let mut state1 = state1.lock().unwrap();

        if state1.sorting {
            return;
        }
        state1.sorting = true;

        let method = self.method;

        self.thread = Some(spawn(move || {
            let state2 = state.clone();

            method(Interface::new(state2.clone()));

            let mut state2 = state2.0.lock().unwrap();
            state2.sorting = false;
            state2.step = None;
        }));
    }
}
