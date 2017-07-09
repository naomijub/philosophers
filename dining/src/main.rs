use std::thread;
use std::sync::{Mutex, Arc};

struct Table {
    forks: Vec<Mutex<()>>,
}

struct DBZFighter {
    name: String,
    time_to_eat: u32,
    left: usize,
    right: usize,
}
impl DBZFighter {
    fn new(name: String, time_to_eat: u32, left: usize, right: usize) -> DBZFighter {
        DBZFighter {
            name: name,
            time_to_eat: time_to_eat,
            left: left,
            right: right,
        }
    }

    fn ate(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} has started eating.", self.name);

        thread::sleep_ms(self.time_to_eat);

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

    let fighters = vec![
        DBZFighter::new("Goku".to_string(), 5000, 0, 1),
        DBZFighter::new("Chichi".to_string(), 100, 1, 2),
        DBZFighter::new("Videl".to_string(), 500, 2, 3),
        DBZFighter::new("Pan".to_string(), 800, 3, 4),
        DBZFighter::new("Gohan".to_string(), 4000, 0, 4),
    ];

    let handles: Vec<_> = fighters.into_iter().map(|f| {
        let table = table.clone();

        thread::spawn(move || {
            f.ate(&table);
        })
    }).collect();

    for fh in handles {
        fh.join().unwrap();
    }
}
