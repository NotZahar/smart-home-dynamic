use num_traits::FromPrimitive;

use crate::utils::random::{RandomGenerator, SimpleRandomGenerator};
use crate::utils::trait_alias::{Number, RandomNumber};

#[derive(Clone, Copy)]
pub enum SocketState {
    ON,
    OFF,
}

pub trait Socket<PowerT: Number> {
    const DEFAULT_INACTIVE_POWER: PowerT;

    #[must_use]
    fn new(default_active_power: PowerT, max_power_offset: PowerT) -> Self;

    fn turn_on(&mut self);

    fn turn_off(&mut self);

    #[must_use]
    fn get_state(&self) -> SocketState;

    #[must_use]
    fn get_power(&mut self) -> PowerT;
}

pub struct PowerSocket<PowerT: RandomNumber> {
    state: SocketState,
    max_power_offset: PowerT,
    default_active_power: PowerT,
    random_offset_generator: SimpleRandomGenerator<PowerT>,
}

impl<PowerT: RandomNumber + FromPrimitive> Socket<PowerT> for PowerSocket<PowerT> {
    const DEFAULT_INACTIVE_POWER: PowerT = PowerT::ZERO;

    fn new(default_active_power: PowerT, max_power_offset: PowerT) -> Self {
        assert!(Self::DEFAULT_INACTIVE_POWER <= max_power_offset);

        PowerSocket {
            state: SocketState::OFF,
            max_power_offset,
            default_active_power,
            random_offset_generator: SimpleRandomGenerator::new(),
        }
    }

    fn turn_on(&mut self) {
        match self.state {
            SocketState::ON => {}
            SocketState::OFF => self.state = SocketState::ON,
        }
    }

    fn turn_off(&mut self) {
        match self.state {
            SocketState::ON => self.state = SocketState::OFF,
            SocketState::OFF => {}
        }
    }

    fn get_state(&self) -> SocketState {
        self.state
    }

    fn get_power(&mut self) -> PowerT {
        match self.state {
            SocketState::ON => {
                self.default_active_power
                    + self
                        .random_offset_generator
                        .generate(Self::DEFAULT_INACTIVE_POWER, self.max_power_offset)
            }
            SocketState::OFF => Self::DEFAULT_INACTIVE_POWER,
        }
    }
}
