use alloc::format;
use kstd::{debug_println, println, print, str::CString};

fn initialize_console() -> Result<(), ()> {
    kstd::console::load()?;
    kstd::console::open("my first console application")?;
    Ok(())
}

pub fn kmain() {
    /*
    initialize_console()
        .expect("cannot initialize console");
    */
    
    debug_println!("loading dll...");

    /*
    let lib_load_result =
        kstd::library::load("/sys/lib/console.obj");

    if let Ok(iter) = lib_load_result {
        for (name, ptr) in iter {
            println!("name: {}, ptr: {:08X}", name, ptr as u32);
        }
    }

    let lib_load_result2 =
        kstd::library::load("/sys/lib/console.obj");

    if let Ok(iter) = lib_load_result2 {
        for (name, ptr) in iter {
            println!("name: {}, ptr: {:08X}", name, ptr as u32);
        }
    }

    print!("no cr");
    println!("add cr");
    println!("hello, console!");
    println!("sum: dec({s}), hex({s:010X})", s = 10 + 11);
    
    let s2 = format!("d{}", 987654321);
    debug_println!("{}", s2);
    debug_println!("ending...");
    */

    let title = CString::new("hello, rust!");

    kapi::window::set_redraw_state(kapi::window::RedrawState::Begin);
    kapi::window::define_and_draw_window(title.as_str());
    kapi::window::set_redraw_state(kapi::window::RedrawState::End);

    loop {
        let window_event = kapi::window::wait_event();
        
        debug_println!("wnd_ev: {:?}", window_event);

        match window_event {
            kapi::window::WindowEvent::Redraw => {
                kapi::window::set_redraw_state(kapi::window::RedrawState::Begin);
                kapi::window::define_and_draw_window(title.as_str());
                kapi::window::set_redraw_state(kapi::window::RedrawState::End);
            }

            _ => {},
        }
    }
}
