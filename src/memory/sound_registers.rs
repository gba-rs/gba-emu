use wasm_bindgen::__rt::core::cell::RefCell;
use wasm_bindgen::__rt::std::rc::Rc;
use memory_macros::*;
use super::GbaMem;
use serde::{Serialize, Deserialize};


/* ------------ Channels 1 and 2 ------------ */
io_register! (
    SoundChannelControlSweep => 2, 0x4000060,
    number_of_sweep_shift: 0, 3,
    sweep_frequency_direction: 3, 1,
    sweep_time: 4, 3,
);

io_register! (
    SoundChannelControlDLE => 2, [0x4000062, 0x4000068],
    sound_length: 0, 6,
    wave_pattern_duty: 6, 2,
    envelope_step_time: 8, 3,
    envelope_direction: 11, 1,
    initial_volume_of_envelope: 12, 4
);

io_register! (
    SoundChannelControlFC => 2, [0x4000064, 0x400006C],
    frequency: 0, 11,
    length_flag: 14, 1,
    initial: 15, 1,
);

/* ------------ Channel 3 ------------------- */
io_register!(
    SoundChannelControlWaveLow => 2, 0x4000070,
    wave_ram_dimension: 5, 1,
    wave_ram_bank_number: 6, 1,
    sound_channel_3_off: 7, 1
);

io_register!(
    SoundChannelControlWaveHigh => 2, 0x4000072,
    sound_length: 0, 8,
    sound_volume: 13, 2,
    force_volume: 15, 1
);

io_register!(
    SoundChannelControlWaveX => 2, 0x4000074,
    sample_rate: 0, 11,
    length_flag: 14, 1,
    initial: 15, 1
);

// Im not sure we want to do it this way
// since this is just a block of memory
// but until its used this will work
io_register!(
    WaveRam => 4, [0x4000090, 0x4000094, 0x4000098, 0x400009C],
    wave_pattern_low: 0, 16,
    wave_pattern_high: 16,16
);

/* ------------ Channel 4 ------------------- */
io_register!(
    SoundChannelControlNoiseLow => 2, 0x4000078,
    sound_length: 0, 6,
    envelope_step_time: 8, 3,
    envelope_direction: 11, 1,
    initial_volume_of_envelope: 12, 4,
);

io_register!(
    SoundChannelControlNoiseHigh => 2, 0x400007C,
    dividing_ratio_of_frequencies: 0, 3,
    counter_step_width: 3, 1,
    shift_clock_frequency: 4, 4,
    length_flag: 14, 1,
    initial: 15, 1
);

/* ------ Channel A and B DMA Sound --------- */
io_register!(
    SoundChannelFifo => 4, [0x40000A0, 0x40000A4],
    data0: 0, 8,
    data1: 8, 8,
    data2: 16, 8,
    data3: 24, 8
);

/* ------ Sound Control --------------------- */
io_register!(
    SoundControlLow => 2, 0x4000080,
    sound_master_volume_right: 0, 3,
    sound_master_volume_left: 4, 3,
    sound_enable_flags_right: 8, 4,
    sound_enable_flags_left: 12, 4
);

io_register!(
    SoundControlHigh => 2, 0x4000082,
    sound_volume: 0, 2,
    dma_sound_a_volume: 2, 1,
    dma_sound_b_volume: 3, 1,
    dma_sound_a_enable_right: 8, 1,
    dma_sound_a_enable_left: 9, 1,
    dma_sound_a_timer_select: 10, 1,
    dma_sound_a_reset_fifo: 11, 1,
    dma_sound_b_enable_right: 12, 1,
    dma_sound_b_enable_left: 13, 1,
    dma_sound_b_timer_select: 14, 1,
    dma_sound_b_reset_fifo: 15, 1,
);

io_register!(
    SoundControlX => 2, 0x4000084,
    sound_1_on_flag: 0, 1,
    sound_2_on_flag: 1, 1,
    sound_3_on_flag: 2, 1,
    sound_4_on_flag: 3, 1,
    psg_fifo_master_enable: 7, 1
);

io_register!(
    SoundBias => 2, 0x4000088,
    bias_level: 1, 9,
    amplitude_resolution_sampling_cycle: 14, 2
);
