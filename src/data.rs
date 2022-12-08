pub struct FanSpeed {
    steps: Vec<SpeedStep>,
    min_duty_cycle: f32,
    min_threshold: f32,
    max_threshold: f32,
}

impl FanSpeed {
    pub fn new(min_duty_cycle: f32, min_threshold: f32, max_threshold: f32) -> FanSpeed {
        if min_threshold > max_threshold {
            panic!("Incorrectly configure FanSpeed !, Minimum fan threshold  greater than maximum threshold ")
        } else if min_duty_cycle < 0f32 || min_duty_cycle > 1f32 {
            panic!("Incorrectly configure FanSpeed ! Minimum Duty cycle is not in the valid range (0-1) ")
        }
        let steps = Vec::new();

        let mut f = FanSpeed {
            steps,
            min_duty_cycle,
            min_threshold,
            max_threshold,
        };

        f.steps.push(SpeedStep {
            temperature: max_threshold,
            fan_speed: 1.0f32,
        });

        f.steps.push(SpeedStep {
            temperature: min_threshold,
            fan_speed: min_duty_cycle,
        });

        return f;
    }

    pub fn push(&mut self, s: SpeedStep) {
        if s.temperature > self.min_threshold || s.temperature > self.max_threshold {
            panic!("Temperature is out of fan threshold range !")
        }
        if s.fan_speed < self.min_duty_cycle || s.fan_speed > 100f32 {
            panic!("Fan Duty Cycle is out of range !")
        }
        let mut index: usize = 0;
        for (i, step) in self.steps.iter().enumerate() {
            if s.temperature > step.temperature {
                index = i;
                break;
            }
        }
        self.steps.insert(index, s);
    }

    pub fn get_speed(&self, temp: f32) -> f32 {
        let mut speed = 0f32;
        if temp < self.min_threshold {
            speed = 0f32
        } else if temp >= self.max_threshold {
            speed = 1.0f32
        } else {
            for (i, low_step) in self.steps.iter().enumerate() {
                if temp < low_step.temperature {
                    let high_step = self.steps[i + 1];
                    let variance = temp - low_step.temperature;
                    let unit_speed = (high_step.fan_speed - low_step.fan_speed)
                        / (high_step.temperature - low_step.temperature);
                    speed = (low_step.fan_speed) + (unit_speed * variance)
                };
            }
        };
        speed
    }
}

#[derive(Copy, Clone)]
pub struct SpeedStep {
    temperature: f32,
    fan_speed: f32,
}

impl SpeedStep {
    pub fn new(temperature: f32, fan_speed: f32) -> SpeedStep {
        if fan_speed > 1f32 || fan_speed < 0f32 {
            panic!("FanSpeed should ber between 0-1")
        } else if temperature < 0f32 || temperature > 100f32 {
            panic!("Temperature should be between 0-100")
        };
        SpeedStep {
            fan_speed,
            temperature,
        }
    }
}
