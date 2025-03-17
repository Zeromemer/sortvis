use crate::sorter::{Interface, Method};
use rand::Rng;

pub static METHODS: &[Method] = &[
    Method {
        name: "shuffle",
        func: |int| {
            let len = int.len();
            for i in 0..len {
                let j = rand::thread_rng().gen_range(i..len);
                int.swap(i, j);
            }
        },
    },
    Method {
        name: "bubble",
        func: |int| {
            let len = int.len();
            for i in 0..len {
                let mut sorted = true;
                for j in 0..len - i - 1 {
                    if int.read(j) > int.read(j + 1) {
                        int.swap(j, j + 1);
                        sorted = false;
                    }
                }
                if sorted {
                    break;
                }
            }
        },
    },
    Method {
        name: "bogo",
        func: |int| {
            let len = int.len();
            loop {
                let mut sorted = true;
                for i in 0..(len - 1) {
                    if int.read(i) > int.read(i + 1) {
                        sorted = false;
                        break;
                    }
                }
                if sorted {
                    return;
                }

                for i in 0..len {
                    let j = rand::thread_rng().gen_range(i..len);
                    int.swap(i, j);
                }
            }
        },
    },
    Method {
        name: "quick",
        func: |int| {
            fn quick_sort(int: &Interface, lo: usize, hi: usize) {
                if lo < hi {
                    let pivot = int.read(lo);
                    let mut i = lo;
                    let mut j = hi;

                    let p = loop {
                        while int.read(i) < pivot {
                            i += 1;
                        }
                        while int.read(j) > pivot {
                            j -= 1;
                        }
                        if i >= j {
                            break j;
                        }
                        int.swap(i, j);
                    };

                    quick_sort(int, lo, p);
                    quick_sort(int, p + 1, hi);
                }
            }

            quick_sort(&int, 0, int.len() - 1);
        },
    },
    Method {
        name: "insertion",
        func: |int| {
            let len = int.len();
            for i in 1..len {
                let key = int.read(i);
                let mut j = i;

                while j > 0 && int.read(j - 1) > key {
                    int.swap(j, j - 1);
                    j -= 1;
                }
            }
        },
    },
];
