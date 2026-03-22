use num_traits::FromPrimitive;

use crate::utils::random::{RandomGenerator, SimpleRandomGenerator};
use crate::utils::trait_alias::{Number, RandomNumber};

pub trait Thermometer<TemperatureT: Number> {
    #[must_use]
    fn new(
        initial_temperature: TemperatureT,
        min_temperature_offset: TemperatureT,
        max_temperature_offset: TemperatureT,
    ) -> Self;

    #[must_use]
    fn get_temperature(&mut self) -> TemperatureT;
}

pub struct CelsiusThermometer<TemperatureT: RandomNumber> {
    min_temperature_offset: TemperatureT,
    max_temperature_offset: TemperatureT,
    current_temperature: TemperatureT,
    random_offset_generator: SimpleRandomGenerator<TemperatureT>,
}

impl<TemperatureT: RandomNumber + FromPrimitive> Thermometer<TemperatureT>
    for CelsiusThermometer<TemperatureT>
{
    fn new(
        initial_temperature: TemperatureT,
        min_temperature_offset: TemperatureT,
        max_temperature_offset: TemperatureT,
    ) -> Self {
        assert!(min_temperature_offset < max_temperature_offset);

        CelsiusThermometer {
            min_temperature_offset,
            max_temperature_offset,
            current_temperature: initial_temperature,
            random_offset_generator: SimpleRandomGenerator::new(),
        }
    }

    fn get_temperature(&mut self) -> TemperatureT {
        self.current_temperature
            + self
                .random_offset_generator
                .generate(self.min_temperature_offset, self.max_temperature_offset)
    }
}
