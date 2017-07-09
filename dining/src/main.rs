struct DBZFighter {
    name: String,
}
impl DBZFighter {
    fn new(name: String) -> DBZFighter {
        DBZFighter {
            name: name,
        }
    }

    fn ate(&self) {
        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let fighters = vec![
        DBZFighter::new("Goku".to_string()),
        DBZFighter::new("Chichi".to_string()),
        DBZFighter::new("Videl".to_string()),
        DBZFighter::new("Pan".to_string()),
        DBZFighter::new("Gohan".to_string()),
    ];

    for f in &fighters {
        f.ate();
    }
}
