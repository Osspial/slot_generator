use std::{
    cmp::Ordering,
    ops::Range,
};
use num_traits::PrimInt;

#[derive(Debug, Clone)]
pub struct SlotGenerator<T>
where
    T: PrimInt,
    Range<T>: ExactSizeIterator,
{
    slots: Vec<Range<T>>,
}

impl<T> SlotGenerator<T>
where
    T: PrimInt,
    Range<T>: ExactSizeIterator,
{
    pub fn new(range: Range<T>) -> SlotGenerator<T> {
        SlotGenerator { slots: vec![range] }
    }

    pub fn get_slot(&mut self) -> T {
        let range = self.slots.last_mut().unwrap();
        let slot = range.start;
        range.start = range.start + T::one();
        if range.len() == 0 {
            self.slots.pop();
        }
        slot
    }

    pub fn replace_slot(&mut self, slot: T) {
        let _1 = T::one();
        let index = self.slots.binary_search_by(|range| {
            let equal =
                range.start.checked_sub(&_1) == Some(slot) || range.end == slot || range.contains(&slot);
            if equal {
                Ordering::Equal
            } else {
                slot.cmp(&range.start)
            }
        });
        match index {
            Err(i) => self.slots.insert(i, slot..slot+_1),
            Ok(i) => {
                let range = &mut self.slots[i];
                if range.start.checked_sub(&_1) == Some(slot) {
                    range.start = slot;
                } else if range.end == slot {
                    range.end = range.end + _1;
                }
                let range = range.clone();
                let prev_range = i.checked_sub(1).map(|i| &mut self.slots[i]);
                if let Some(prev_range) = prev_range {
                    if prev_range.start == range.end {
                        prev_range.start = range.start;
                        self.slots.remove(i);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_slots() {
        let mut slots = SlotGenerator::new(1..u32::max_value());
        assert_eq!(1, slots.get_slot());
        assert_eq!(2, slots.get_slot());
        assert_eq!(3, slots.get_slot());
    }

    #[test]
    fn replace_slots() {
        let mut slots = SlotGenerator::new(1..u32::max_value());
        let a = slots.get_slot();
        let b = slots.get_slot();
        let c = slots.get_slot();
        let d = slots.get_slot();
        assert_eq!(1, a);
        assert_eq!(2, b);
        assert_eq!(3, c);
        assert_eq!(4, d);

        slots.replace_slot(a);
        slots.replace_slot(c);
        slots.replace_slot(b);
        slots.replace_slot(d);
        assert_eq!(1, slots.slots.len());
    }
}
