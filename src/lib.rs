use std::time::Duration;
use std::thread::sleep;
use std::io::{self, Write};

pub struct Countdown {
    duration: usize,
}

impl Countdown {

    pub fn new(duration: usize) -> Self {
        Countdown { duration: duration }
    }

    pub fn start(&self) {
        let mut duration_remaining = self.duration;
        while duration_remaining > 0 {
            self.countdown_one_second_from(&duration_remaining);
            duration_remaining -= 1;
        }
        println!("");
    }

    //-------- private ----------//

    fn countdown_one_second_from(&self, start_second: &usize) {
        let quarter_of_second = Duration::from_millis(250);
        print!("{}", start_second);
        for _ in 0..3 {
            print!(".");
            io::stdout().flush().unwrap();
            sleep(quarter_of_second);
        }
    }
}
