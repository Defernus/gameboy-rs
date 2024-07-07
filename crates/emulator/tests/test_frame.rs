use emulator::*;

#[test]
fn test_frame() {
    let mut emulator = Emulator::default();

    assert!(!emulator.is_frame_available);

    for i in 0..153 {
        // frame becomes available after 144 scanlines
        assert_eq!(emulator.is_frame_available, i >= 144);

        for scanline_dot in 0..456 {
            if i >= 144 {
                assert_eq!(
                    emulator.reg::<RegisterSTAT>().get_ppu_mode(),
                    PpuMode::Mode1
                );
            } else {
                if scanline_dot < 80 {
                    assert_eq!(
                        emulator.reg::<RegisterSTAT>().get_ppu_mode(),
                        PpuMode::Mode2
                    );
                }
            }
            assert_eq!(emulator.reg::<RegisterLY>().0, i);
            emulator.handle_dot();
        }

        assert_eq!(emulator.reg::<RegisterLY>().0, i + 1);
    }

    for _ in 0..456 {
        assert_eq!(emulator.reg::<RegisterLY>().0, 153);
        emulator.handle_dot();
    }

    assert_eq!(emulator.reg::<RegisterLY>().0, 0);
}
