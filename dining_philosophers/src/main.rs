use std::thread;
use std::time::Duration;
use std::sync::{Mutex, Arc};

struct Phiosopher {
    name: String,
    left: usize,
    right: usize,
}

struct Table {
    forks: Vec<Mutex<()>>,
}

impl Phiosopher {
    fn new(name: &str, left: usize, right: usize) -> Phiosopher {
        Phiosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        thread::sleep(Duration::from_millis(150));
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);

        thread::sleep(Duration::from_millis(1000));

        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});
    let phiosophers = vec![
        Phiosopher::new("Suzuki 1", 0, 1),
        Phiosopher::new("Suzuki 2", 1, 2),
        Phiosopher::new("Suzuki 3", 2, 3),
        Phiosopher::new("Suzuki 4", 3, 4),
        Phiosopher::new("Suzuki 5", 0, 4),
    ];

    let handlers: Vec<_> = phiosophers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handlers {
        h.join().unwrap();
    }
}
