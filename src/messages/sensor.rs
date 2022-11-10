use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Serialize, Deserialize)]
pub struct SensorPacket {
    status: StatusPacket,
    emi: EmiPacket,
}

impl SensorPacket {
    pub fn new(status: StatusPacket, emi: EmiPacket) -> Self {
        Self { status, emi }
    }
}

#[derive(Serialize, Deserialize)]
pub struct StatusPacket {
    vehicle_id: u16,
    sensor_id: u16,
    sofware_load_id: String,
    curr_freq_ch: u16,
    next_freq_ch: u16,
    vehicle_pos: (f32, f32),
    vehicle_vel: (f32, f32),
    vehicle_rot: f32,
    num_nodes: u8,
    sensor_ids: Vec<u8>,
    sensor_health: SensorHealth,
    radar_mode: RadarMode,
    countermeasure_state: CountermeasureState,
}

impl StatusPacket {
    pub fn new(
        vehicle_id: u16,
        sensor_id: u16,
        sofware_load_id: String,
        curr_freq_ch: u16,
        next_freq_ch: u16,
        vehicle_pos: (f32, f32),
        vehicle_vel: (f32, f32),
        vehicle_rot: f32,
        num_nodes: u8,
        sensor_ids: Vec<u8>,
        sensor_health: SensorHealth,
        radar_mode: RadarMode,
        countermeasure_state: CountermeasureState,
    ) -> Self {
        return Self {
            vehicle_id,
            sensor_id,
            sofware_load_id,
            curr_freq_ch,
            next_freq_ch,
            vehicle_pos,
            vehicle_vel,
            vehicle_rot,
            num_nodes,
            sensor_ids,
            sensor_health,
            radar_mode,
            countermeasure_state,
        };
    }
}

#[derive(Serialize, Deserialize)]
pub struct EmiPacket {
    freq_ch_coarse: u16,
    freq_ch_fine: u16,
    azimuth: f32,
    elevation: f32,
    time_since_last_detect: Duration,
}

impl EmiPacket {
    pub fn new(
        freq_ch_coarse: u16,
        freq_ch_fine: u16,
        azimuth: f32,
        elevation: f32,
        time_since_last_detect: Duration,
    ) -> Self {
        return Self {
            freq_ch_coarse,
            freq_ch_fine,
            azimuth,
            elevation,
            time_since_last_detect,
        };
    }
}

#[derive(Serialize, Deserialize)]
pub enum SensorHealth {
    GOOD,
    FAIR,
    POOR,
}

#[derive(Serialize, Deserialize)]
pub enum RadarMode {
    OWLS,
}

#[derive(Serialize, Deserialize)]
pub enum CountermeasureState {
    ONLINE,
    STANDBY,
    OFFLINE,
}
