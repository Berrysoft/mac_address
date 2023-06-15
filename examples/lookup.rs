use mac_address2::mac_address_by_name;

fn main() {
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "ios"))]
    let name = "eth0";

    #[cfg(target_os = "freebsd")]
    let name = "em0";

    #[cfg(target_os = "openbsd")]
    let name = "fxp0";

    #[cfg(target_os = "windows")]
    let name = "Ethernet";

    #[cfg(target_os = "android")]
    let name = "wlan0";

    match mac_address_by_name(name) {
        Ok(Some(ma)) => {
            println!("MAC addr of {} = {}", name, ma);
            println!("bytes = {:?}", ma.bytes());
        }
        Ok(None) => println!("Interface \"{}\" not found", name),
        Err(e) => println!("{:?}", e),
    }
}
