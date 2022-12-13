use super::Rssi;

pub(crate) struct NoiceFloor {
    value: Rssi,
    average_window: usize,
}

impl NoiceFloor {
    pub const fn new(initial_value: Rssi) -> Self {
        Self {
            value: initial_value,
            average_window: 8,
        }
    }

    pub fn value(&self) -> Rssi {
        self.value
    }

    pub fn add(&mut self, rssi: Rssi) {
        let mut accumulated = self.value as isize * self.average_window as isize;
        accumulated = accumulated - self.value as isize + rssi as isize;
        self.value = (accumulated / self.average_window as isize) as Rssi;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_accumulate() {
        let mut floor = NoiceFloor::new(-120);
        assert_eq!(-120, floor.value());

        floor.add(-118);
        assert_eq!(-119, floor.value());

        floor.add(-116);
        assert_eq!(-118, floor.value());
    }

    #[test]
    fn can_accumulate_after_window() {
        let mut floor = NoiceFloor::new(-120);
        for _ in 0..8 {
            floor.add(-120);
        }
        assert_eq!(-120, floor.value());

        floor.add(-112);
        assert_eq!(-119, floor.value());
    }
}
