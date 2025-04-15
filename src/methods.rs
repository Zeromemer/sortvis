use crate::sorter::{Interface, Method};
use rand::Rng;

pub static MODIFIERS: &[Method] = &[
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
        name: "invert",
        func: |int| {
            let len = int.len();
            let half = len / 2;
            for i in 0..half {
                int.swap(i, len - i - 1);
            }
        },
    },
    Method {
        name: "pyramid",
        func: |int| {
            let len = int.len();
            if len == 0 {
                return;
            }

            let mid = (len - 1) / 2;
            let mut target_pos = vec![0; len];
            let mut left = mid;
            let mut right = mid + 1;

            for i in (0..len).rev() {
                if i % 2 == 0 {
                    target_pos[i] = left;
                    if left > 0 {
                        left -= 1;
                    }
                } else {
                    target_pos[i] = right;
                    if right < len - 1 {
                        right += 1;
                    }
                }
            }

            for i in 0..len {
                while target_pos[i] != i {
                    let swap_with = target_pos[i];
                    int.swap(i, swap_with);
                    target_pos.swap(i, swap_with);
                }
            }
        },
    },
];

pub static METHODS: &[Method] = &[
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
    Method {
        name: "selection",
        func: |int| {
            let len = int.len();
            for i in 0..len {
                let mut min_index = i;
                let mut min_value = int.read(i);
                for j in (i + 1)..len {
                    if int.read(j) < min_value {
                        min_index = j;
                        min_value = int.read(j);
                    }
                }
                if min_index != i {
                    int.swap(i, min_index);
                }
            }
        },
    },
    Method {
        name: "gnome",
        func: |int| {
            let len = int.len();
            let mut i = 0;
            while i < len {
                if i == 0 || int.read(i - 1) <= int.read(i) {
                    i += 1;
                } else {
                    int.swap(i, i - 1);
                    i -= 1;
                }
            }
        },
    },
    Method {
        name: "shell",
        func: |int| {
            let len = int.len();
            let mut gap = len / 2;
            while gap > 0 {
                for i in gap..len {
                    let temp = int.read(i);
                    let mut j = i;
                    while j >= gap && int.read(j - gap) > temp {
                        int.swap(j, j - gap);
                        j -= gap;
                    }
                }
                gap /= 2;
            }
        },
    },
    Method {
        name: "cocktail",
        func: |int| {
            let len = int.len();
            let mut start = 0;
            let mut end = len - 1;
            let mut swapped = true;
            while swapped {
                swapped = false;
                for i in start..end {
                    if int.read(i) > int.read(i + 1) {
                        int.swap(i, i + 1);
                        swapped = true;
                    }
                }
                if !swapped {
                    break;
                }
                swapped = false;
                end -= 1;
                for i in (start..end).rev() {
                    if int.read(i) > int.read(i + 1) {
                        int.swap(i, i + 1);
                        swapped = true;
                    }
                }
                start += 1;
            }
        },
    },
];
