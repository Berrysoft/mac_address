use crate::{MacAddress, MacAddressError};
use nix::ifaddrs;

/// An iterator over all available MAC addresses on the system.
pub struct MacAddressIterator {
    iter: std::iter::FilterMap<
        ifaddrs::InterfaceAddressIterator,
        fn(ifaddrs::InterfaceAddress) -> Option<MacAddress>,
    >,
}

impl MacAddressIterator {
    /// Creates a new `MacAddressIterator`.
    pub fn new() -> Result<MacAddressIterator, MacAddressError> {
        Ok(Self {
            iter: ifaddrs::getifaddrs()?.filter_map(filter_macs),
        })
    }
}

fn filter_macs(intf: ifaddrs::InterfaceAddress) -> Option<MacAddress> {
    let address = intf.address?;
    let link = address.as_link_addr()?;
    Some(MacAddress::new(link.addr()?))
}

impl Iterator for MacAddressIterator {
    type Item = MacAddress;

    fn next(&mut self) -> Option<MacAddress> {
        self.iter.next()
    }
}
