use crate::GLOBAL_STATE;
use std::panic::{panic_any, set_hook};
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

struct StopThread;

impl Interface {
    pub const fn new(state: Weak<Mutex<State>>) -> Self {
        Self { state }
    }

    fn modify_state<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut State) -> T,
    {
        self.state.upgrade().map_or_else(
            || {
                panic_any(StopThread);
            },
            |state| {
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
            },
        )
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
        });
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
    handle: Option<JoinHandle<()>>,
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
            handle: None,
        }
    }

    pub fn is_sorting(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.sorting
    }

    pub fn start(&mut self, track: bool) {
        let state = self.state.clone();
        {
            let mut state1 = state.lock().unwrap();

            if state1.sorting {
                return;
            }
            state1.sorting = true;
            if track {
                state1.start_time = Some(Instant::now());
            }
        }

        if let Some(method) = self.method {
            self.handle = Some(spawn(move || {
                set_hook(Box::new(move |panic_info| {
                    let payload = panic_info.payload();

                    if !payload.is::<StopThread>() {
                        let mut global = GLOBAL_STATE.lock().unwrap();
                        let payload_str = payload.downcast_ref::<&str>().map_or_else(
                            || {
                                payload.downcast_ref::<String>().map_or_else(
                                    || format!("{payload:?}"),
                                    std::clone::Clone::clone,
                                )
                            },
                            |s| String::from(*s),
                        );

                        global.panic = panic_info
                            .location()
                            .map_or_else(
                                || format!("Panic: {payload_str}"),
                                |location| {
                                    format!(
                                        "Panic at {}:{}: {}",
                                        location.file(),
                                        location.line(),
                                        payload_str
                                    )
                                },
                            )
                            .into();
                    }
                }));

                let state1 = Arc::downgrade(&state);
                drop(state);

                method(Interface::new(state1.clone()));

                let state1 = state1.upgrade().unwrap();
                let mut state1 = state1.lock().unwrap();
                state1.sorting = false;
                state1.step = None;
                if track {
                    state1.stop_time = Some(Instant::now());
                }
            }));
        } else {
            panic!("No method");
        }
    }

    pub fn resume(&self) {
        if let Some(handle) = &self.handle {
            handle.thread().unpark();
        }
    }

    pub fn stop(&mut self) {
        let state_clone = self.state.clone();
        let state = state_clone;
        let mut state = state.lock().unwrap();
        state.sorting = false;
        state.step = None;
        self.state = Arc::new(Mutex::new(state.clone()));
        drop(state);
        if let Some(handle) = self.handle.take() {
            handle.thread().unpark();
        }
    }
}
