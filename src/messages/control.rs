use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ControlPacket {
    system_id: u8,
    system_power: bool,
    frequency: f32,
    bandwidth: f32,
    prf: f32,
    num_pulses: u16,
    detection_thresh: f32,
}

impl ControlPacket {
    pub fn new(
        system_id: u8,
        system_power: bool,
        frequency: f32,
        bandwidth: f32,
        prf: f32,
        num_pulses: u16,
        detection_thresh: f32,
    ) -> Self {
        return Self {
            system_id,
            system_power,
            frequency,
            bandwidth,
            prf,
            num_pulses,
            detection_thresh,
        };
    }
}
