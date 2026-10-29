fn main() {
    let offload_param = "__NV_PRIME_RENDER_OFFLOAD";
    let lib_param = "__GLX_VENDOR_LIBRARY_NAME";

    let args = std::env::args().collect::<Vec<_>>();

    if args.len() < 2 {
        println!("Please provide a command to be offloaded to nvidia GPU.");
        return;
    }

    if args.len() >= 2 {
        if args[1] == "--version" || args[1] == "-v" || args[1] == "--help" {
            println!(
                "\tprime-rs\nA rust based utility to offload your little\nprograms to your nvidia GPU.\n\t\t\t -By Aditya Gupta.\n\nhttps://github.com/aditya-gupta-dev/prime-rs"
            );
            return;
        }
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
