use crate::sorter::Method;
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
];
