use std::thread;

struct DBZFighter {
    name: String,
    time_to_eat: u32,
}
impl DBZFighter {
    fn new(name: String, time_to_eat: u32) -> DBZFighter {
        DBZFighter {
            name: name,
            time_to_eat: time_to_eat,
        }
    }

    fn ate(&self) {
        println!("{} has started eating.", self.name);

        thread::sleep_ms(self.time_to_eat);

        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let fighters = vec![
        DBZFighter::new("Goku".to_string(), 5000),
        DBZFighter::new("Chichi".to_string(), 100),
        DBZFighter::new("Videl".to_string(), 500),
        DBZFighter::new("Pan".to_string(), 800),
        DBZFighter::new("Gohan".to_string(), 4000),
    ];

    let handles: Vec<_> = fighters.into_iter().map(|f| {
        thread::spawn(move || {
            f.ate();
        })
    }).collect();

    for fh in handles {
        fh.join().unwrap();
    }
}
