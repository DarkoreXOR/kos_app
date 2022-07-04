use crate::system_call::{syscall_abcdSD, syscall_ab, syscall_a_a};

#[derive(Debug)]
pub enum RedrawState {
    Begin,
    End,
}

#[derive(Debug)]
pub enum WindowEvent {
    /// Contains event code.
    Unknown(u32),
    Redraw,
}

pub fn set_redraw_state(redraw_state: RedrawState) {
    let ebx = match redraw_state {
        RedrawState::Begin => 1,
        RedrawState::End => 2,
    };

    unsafe {
        syscall_ab(12, ebx);
    }
}

pub fn define_and_draw_window(title: &str) {
    let position = (10, 10);
    let size = (100, 100);

    /*
    let ebx = (position.0 << 15) | size.0;
    let ecx = (position.1 << 15) | size.1;

    let window_style_x = 0b_0_0_1_1; // 0b_D_C_B_A
    let window_style_y = 2;
    let window_xy = window_style_x << 4 | window_style_y;
    let window_style_rgb = 0x7F7F7F;

    let edx = window_xy | window_style_rgb << 8;

    let header_x = 0; // no gradient
    let header_y = 0; // default window
    let header_xy = header_x << 4 | header_y;
    let header_rbg = 0x2F2F2F;

    let esi = header_xy | header_rbg << 8;

    let edi = 0x1F1F1F;
    */

    // tmp

    let ebx = 10 * 65536 + 150;
    let ecx = 40 * 65536 + 50;
    let edx = 0x33_FFFFFF;
    let esi = 0x00_000000;
    let edi = title.as_ptr() as u32;


    unsafe {
        syscall_abcdSD(0, ebx, ecx, edx, esi, edi);
    }
}

pub fn wait_event() -> WindowEvent {
    unsafe { 
        match syscall_a_a(10).0 {
            1 => WindowEvent::Redraw,
            other_event_code => WindowEvent::Unknown(other_event_code),
        }
    }
}
