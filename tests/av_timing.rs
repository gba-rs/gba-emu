use gba_emulator::{
    gamepak::{BackupType, GamePack},
    gba::GBA,
};

fn step(gba: &mut GBA, cycles: usize) {
    gba.gpu.step(
        cycles,
        &mut gba.memory_bus.mem_map,
        &mut gba.interrupt_handler,
        &mut gba.dma_control,
    );
}

#[test]
fn frame_has_228_lines_and_vblank_ends_on_line_227() {
    let mut gba = GBA::default();
    step(&mut gba, 1232 * 227);
    assert_eq!(gba.gpu.vertical_count.get_current_scanline(), 227);
    assert_eq!(gba.gpu.display_status.get_vblank_flag(), 0);
    assert!(!gba.gpu.frame_ready);
    step(&mut gba, 1231);
    assert!(!gba.gpu.frame_ready);
    step(&mut gba, 1);
    assert!(gba.gpu.frame_ready);
    assert_eq!(gba.gpu.vertical_count.get_current_scanline(), 0);
}

#[test]
fn hblank_palette_write_affects_next_line_only() {
    let mut gba = GBA::default();
    gba.memory_bus.write_u16(0x0500_0000, 0x001f);
    step(&mut gba, 960);
    assert_eq!(gba.gpu.frame_buffer[0], 0xff0000);
    gba.memory_bus.write_u16(0x0500_0000, 0x03e0);
    step(&mut gba, 272 + 960);
    assert_eq!(gba.gpu.frame_buffer[0], 0xff0000);
    assert_eq!(gba.gpu.frame_buffer[240], 0x00ff00);
}

#[test]
fn fifo_accepts_byte_halfword_and_word_in_sample_order() {
    let mut gba = GBA::default();
    gba.memory_bus.write_u8(0x0400_00a0, 1);
    gba.memory_bus.write_u16(0x0400_00a2, 0x0302);
    gba.memory_bus.write_u32(0x0400_00a0, 0x07060504);
    gba.memory_bus.write_u16(0x0400_00a6, 0x0908);
    assert_eq!(
        gba.memory_bus
            .mem_map
            .fifo_a
            .iter()
            .copied()
            .collect::<Vec<_>>(),
        [1, 2, 3, 4, 5, 6, 7]
    );
    assert_eq!(
        gba.memory_bus
            .mem_map
            .fifo_b
            .iter()
            .copied()
            .collect::<Vec<_>>(),
        [8, 9]
    );
    for _ in 0..40 {
        gba.memory_bus.write_u8(0x0400_00a0, 10);
    }
    assert_eq!(gba.memory_bus.mem_map.fifo_a.len(), 32);
}

#[test]
fn classic_nes_ignores_the_decoy_sram_marker() {
    let mut rom = vec![0; 1024];
    rom[0xac..0xb0].copy_from_slice(b"FMRE");
    rom[0x200..0x204].copy_from_slice(b"SRAM");
    assert_eq!(
        GamePack::from_bytes(rom, vec![]).backup_type,
        BackupType::Eeprom
    );
}

#[test]
fn rebinding_memory_preserves_sound_bias() {
    let mut gba = GBA::default();
    gba.apu.sound_bias.set_bias_level(0x80);
    gba.register_memory();
    assert_eq!(gba.apu.sound_bias.get_bias_level(), 0x80);
}

#[test]
fn stopping_fifo_dma_does_not_start_an_extra_transfer() {
    let mut gba = GBA::default();
    gba.memory_bus.write_u32(0x0400_00bc, 0x0300_1000);
    gba.memory_bus.write_u32(0x0400_00c0, 0x0400_00a0);
    gba.memory_bus.write_u32(0x0400_00c4, 0xb200_0000);
    gba.dma_control
        .update(&mut gba.memory_bus, &mut gba.interrupt_handler, [false; 2]);
    assert_eq!(gba.dma_control.dma_channels[1].internal_word_count, 4);
    // Actual Classic NES sound-driver stop sequence: change timing, then clear
    // enable. The enable bit stays high during the timing change, so it must not
    // request an immediate transfer beyond the end of the PCM buffer.
    gba.memory_bus.mem_map.fifo_a.extend([12, 14, 16]);
    gba.memory_bus.write_u32(0x0400_00c4, 0x8440_0004);
    gba.dma_control
        .update(&mut gba.memory_bus, &mut gba.interrupt_handler, [false; 2]);
    gba.memory_bus.cycle_clock.get_cycles();
    gba.dma_control
        .update(&mut gba.memory_bus, &mut gba.interrupt_handler, [false; 2]);
    assert_eq!(gba.memory_bus.cycle_clock.get_cycles(), 0);
    assert_eq!(gba.memory_bus.mem_map.fifo_a.iter().copied().collect::<Vec<_>>(), [12, 14, 16]);
    assert_eq!(gba.dma_control.dma_channels[1].internal_source_address, 0x0300_1000);
    assert!(!gba.dma_control.dma_channels[1].pending_immediate);
}

#[test]
fn immediate_dma_requires_a_new_enable_edge_and_ignores_repeat() {
    let mut gba = GBA::default();
    gba.memory_bus.write_u32(0x0300_1000, 0x44332211);
    gba.memory_bus.write_u32(0x0400_00bc, 0x0300_1000);
    gba.memory_bus.write_u32(0x0400_00c0, 0x0300_2000);
    gba.memory_bus.write_u32(0x0400_00c4, 0x8600_0001); // immediate, repeat, 32-bit
    gba.dma_control.update(&mut gba.memory_bus, &mut gba.interrupt_handler, [false; 2]);
    assert_eq!(gba.memory_bus.mem_map.read_u32(0x0300_2000), 0);
    gba.dma_control.update(&mut gba.memory_bus, &mut gba.interrupt_handler, [false; 2]);
    assert_eq!(gba.memory_bus.mem_map.read_u32(0x0300_2000), 0x44332211);
    assert_eq!(gba.dma_control.dma_channels[1].control.get_dma_enable(), 0);
    gba.memory_bus.write_u32(0x0300_1000, 0x88776655);
    gba.memory_bus.write_u32(0x0400_00c4, 0x8400_0001);
    for _ in 0..2 {
        gba.dma_control.update(&mut gba.memory_bus, &mut gba.interrupt_handler, [false; 2]);
    }
    assert_eq!(gba.memory_bus.mem_map.read_u32(0x0300_2000), 0x88776655);
}

#[test]
fn psg_mixer_honors_quarter_half_and_full_volume() {
    let mut amplitudes = Vec::new();
    for volume in 0..3 {
        let mut gba = GBA::default();
        gba.apu.sound_control_x.set_psg_fifo_master_enable(1);
        gba.apu.sound_control_low.set_sound_master_volume_left(7);
        gba.apu.sound_control_low.set_sound_enable_flags_left(1);
        gba.apu.sound_control_high.set_sound_volume(volume);
        gba.memory_bus.write_u8(0x0400_0063, 0xf0);
        gba.apu.square1.on_trigger();
        gba.apu.step(512, [0; 4], &mut gba.memory_bus);
        amplitudes.push(gba.apu.sample_buffer[0]);
    }
    assert_ne!(amplitudes[0], 0);
    assert_eq!(amplitudes[0] * 2, amplitudes[1]);
    assert_eq!(amplitudes[1] * 2, amplitudes[2]);
}

#[test]
fn timer_batches_match_tick_by_tick_reference() {
    for (prescaler, frequency) in [1usize, 64, 256, 1024].iter().enumerate() {
        for reload in [0u16, 0xff00, 0xfffe, 0xffff] {
            let mut gba = GBA::default();
            let timer = &mut gba.timer_handler.timers[0];
            timer.controller.set_pre_scalar_selection(prescaler as u8);
            timer.controller.set_irq_enable(1);
            timer.timer.write_reload(reload);
            timer.timer.set_data(0xfffa);
            let mut reference = 0xfffau16;
            let mut remainder = 0usize;
            for cycles in [1, 2, 11, 513, 1232, 131071] {
                remainder += cycles;
                let mut expected_overflows = 0;
                while remainder >= *frequency {
                    remainder -= frequency;
                    reference = reference.wrapping_add(1);
                    if reference == 0 {
                        reference = reload;
                        expected_overflows += 1;
                    }
                }
                let actual = timer.update(cycles, &mut gba.interrupt_handler);
                assert_eq!(actual, expected_overflows);
                assert_eq!(timer.timer.get_data(), reference);
                assert_eq!(timer.cycles, remainder);
            }
        }
    }
}

#[test]
fn pcm_consumption_follows_timer_phase_and_restart() {
    let mut gba = GBA::default();
    gba.memory_bus.mem_map.fifo_a.extend([10, 20, 30]);
    gba.timer_handler.timers[0].timer.write_reload(0xfffc);
    gba.timer_handler.timers[0].controller.set_enable(1);
    let tick = |gba: &mut GBA, cycles| {
        let overflows = gba.timer_handler.update(cycles, &mut gba.interrupt_handler);
        gba.apu.step(cycles, overflows, &mut gba.memory_bus);
    };
    tick(&mut gba, 5); // Two startup cycles, then three timer ticks.
    assert_eq!(gba.apu.direct_sound_a.current_sample, 0);
    tick(&mut gba, 1);
    assert_eq!(gba.apu.direct_sound_a.current_sample, 10);
    gba.timer_handler.timers[0].controller.set_enable(0);
    tick(&mut gba, 1000);
    assert_eq!(gba.apu.direct_sound_a.current_sample, 10);
    gba.timer_handler.timers[0].controller.set_enable(1);
    tick(&mut gba, 5);
    assert_eq!(gba.apu.direct_sound_a.current_sample, 10);
    tick(&mut gba, 1);
    assert_eq!(gba.apu.direct_sound_a.current_sample, 20);
    assert!(gba.memory_bus.mem_map.fifo_a.is_empty());
    tick(&mut gba, 4);
    assert_eq!(gba.apu.direct_sound_a.current_sample, 30);
}
