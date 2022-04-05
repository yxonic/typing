pub struct Stats;

impl Stats {
    fn start(&self) {
        todo!()
    }
    fn pause(&self) {
        todo!()
    }
    fn hit(&self) {
        todo!()
    }
    fn miss(&self) {
        todo!()
    }
    fn hit_word(&self) {
        todo!()
    }
    fn report(&self) -> StatReport {
        todo!()
    }
    fn reset(&self) {
        todo!()
    }
}

pub struct StatReport {
    hpm: f64,
    mpm: f64,
    wpm: f64,
}
