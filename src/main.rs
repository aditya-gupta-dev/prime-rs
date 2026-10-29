fn main() {
    let offload_param = "__NV_PRIME_RENDER_OFFLOAD";
    let lib_param = "__GLX_VENDOR_LIBRARY_NAME";

    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        println!("Please provide a command to be offloaded to nvidia GPU.");
        return;
    }

    if args.len() > 2 {
        let _ = std::process::Command::new(&args[1])
            .env(offload_param, "1")
            .env(lib_param, "nvidia")
            .args(&args[2..])
            .spawn()
            .expect("failed to start the process");
    } else {
        let _ = std::process::Command::new(&args[1])
            .env(offload_param, "1")
            .env(lib_param, "nvidia")
            .spawn()
            .expect("failed to start the process");
    }
    std::process::exit(0);
}
