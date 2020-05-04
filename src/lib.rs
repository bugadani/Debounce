//! This crate implements a simple debouncer

#![cfg_attr(not(test), no_std)]

use typenum::Unsigned;
use core::marker::PhantomData;

pub use typenum::consts::*;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum State {
    Touched,
    Released
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Change {
    Touch,
    Release,
    NoChange(State)
}

impl Into<State> for bool {
    fn into(self) -> State {
        if self {
            State::Touched
        } else {
            State::Released
        }
    }
}

impl Into<State> for Change {
    fn into(self) -> State {
        match self {
            Change::Touch => State::Touched,
            Change::Release => State::Released,
            Change::NoChange(state) => state
        }
    }
}

impl State {
    pub fn to_change(self) -> Change {
        match self {
            State::Touched => Change::Touch,
            State::Released => Change::Release
        }
    }
}

pub struct Debounce<N> where N: Unsigned {
    samples: usize,
    state: State,
    _marker: PhantomData<N>
}

impl<N> Debounce<N> where N: Unsigned {
    pub fn new() -> Self {
        Self {
            samples: 0,
            state: State::Released,
            _marker: PhantomData
        }
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn update(&mut self, sample: State) -> Change {
        if sample != self.state {
            if self.samples == N::to_usize() - 1 {
                self.state = sample;
                self.samples = 0;

                self.state.to_change()
            } else {
                self.samples += 1;

                Change::NoChange(self.state)
            }
        } else {
            Change::NoChange(self.state)
        }
    }
}

#[cfg(test)]
mod tests {
    use typenum::consts::*;
    use super::*;

    #[test]
    fn returns_change() {
        let mut debounce: Debounce<U3> = Debounce::new();

        assert_eq!(Change::NoChange(State::Released), debounce.update(State::Touched));
        assert_eq!(Change::NoChange(State::Released), debounce.update(State::Touched));
        assert_eq!(Change::Touch, debounce.update(State::Touched));
        assert_eq!(Change::NoChange(State::Touched), debounce.update(State::Touched));

        assert_eq!(Change::NoChange(State::Touched), debounce.update(State::Released));
        assert_eq!(Change::NoChange(State::Touched), debounce.update(State::Released));
        assert_eq!(Change::Release, debounce.update(State::Released));
        assert_eq!(Change::NoChange(State::Released), debounce.update(State::Released));
    }
}
