/// Extremely simple use of arguments to create matrix options
#[cfg(all(target_arch = "aarch64", target_vendor = "unknown", target_os = "linux", target_env = "gnu"))]
use clap::{crate_version, App};
#[cfg(all(target_arch = "aarch64", target_vendor = "unknown", target_os = "linux", target_env = "gnu"))]
use rpi_led_matrix::args;

#[cfg(all(target_arch = "aarch64", target_vendor = "unknown", target_os = "linux", target_env = "gnu"))]
fn main() {
    let app = args::add_matrix_args(
        App::new("Argument Example")
            .about("shows basic usage of matrix arguments")
            .version(crate_version!()),
    );
    let matches = app.get_matches();
    let (options, rt_options) = args::matrix_options_from_args(&matches);
    println!("Options: {:?}", options);
    println!("Runtime Options: {:?}", rt_options);
}

#[cfg(not(all(target_arch = "aarch64", target_vendor = "unknown", target_os = "linux", target_env = "gnu")))]
fn main() {
    println!("Example only available on rpi.")
}
