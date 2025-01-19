use std::sync::{Arc, Mutex, Condvar};
use std::thread::{spawn, JoinHandle};


pub enum Step {
    Read(usize),
    Swap(usize, usize)
}

pub struct Interface {
    state: Arc<(Mutex<State>, Condvar)>
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
    pub step: Option<Step>
}

pub struct Sorter {
    pub state: Arc<(Mutex<State>, Condvar)>,
    pub thread: JoinHandle<()>,
    #[allow(dead_code)]
    method: Method
}

impl Sorter {
    pub fn new(data: Vec<u32>, method: Method) -> Self {
        let state = Arc::new((
            Mutex::new(State {
                sorting: false,
                data,
                step: None
            }),
            Condvar::new()
        ));
        let state1 = state.clone();

        let thread = spawn(move || {
            let state2 = state1.clone();
            
            method(Interface::new(state2.clone()));
            
            let mut state3 = state2.0.lock().unwrap();
            state3.sorting = false;
            state3.step = None;
        });

        Self {
            state,
            thread,
            method
        }
    }
}
