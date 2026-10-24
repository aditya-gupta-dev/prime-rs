use std::{os::unix::process::CommandExt};

fn main() {
    let offload_param ="__NV_PRIME_RENDER_OFFLOAD"; 
    let lib_param = "__GLX_VENDOR_LIBRARY_NAME"; 

    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        println!("Please provide a command to be offloaded to nvidia GPU."); 
        return; 
    }

    let mut command = std::process::Command::new(&args[1]);  

    if args.len() > 2 { 
        command.args(&args[2..]); 
    }

    command.env(offload_param, "1"); 
    command.env(lib_param, "nvidia");
    unsafe { 
    command.pre_exec(|| { 
        libc::setsid(); 
                Ok(())
    }); 
    } 
    let _ = command.spawn(); 
}
