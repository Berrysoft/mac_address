use crate::MacAddressError;
use std::{ffi::CStr, ptr};
use widestring::U16CStr;
use windows_sys::Win32::{
    NetworkManagement::IpHelper::{GetAdaptersAddresses, IP_ADAPTER_ADDRESSES_LH},
    Networking::WinSock::AF_UNSPEC,
};

//const GAA_FLAG_NONE: ULONG = 0x0000;

/// Uses bindings to the `Iphlpapi.h` Windows header to fetch the interface devices
/// list with [GetAdaptersAddresses][https://msdn.microsoft.com/en-us/library/windows/desktop/aa365915(v=vs.85).aspx]
/// then loops over the returned list until it finds a network device with a MAC address,
/// and returns it.
///
/// If it fails to find a device, it returns a `NoDevicesFound` error.
pub fn get_mac(name: Option<&str>) -> Result<Option<[u8; 6]>, MacAddressError> {
    let mut adapters = get_adapters()?;
    // Pointer to the current location in the linked list
    let mut ptr = adapters.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH;

    loop {
        // Break if we've gone through all devices
        if ptr.is_null() {
            break;
        }

        let addr_lh = unsafe { ptr.read_unaligned() };

        let bytes = convert_mac_bytes(&addr_lh);

        if let Some(name) = name {
            let adapter_name = unsafe { U16CStr::from_ptr_str(addr_lh.FriendlyName) };

            if adapter_name.to_string_lossy() == name {
                return Ok(Some(bytes));
            } else {
                let adapter_name = unsafe { CStr::from_ptr(addr_lh.AdapterName as *const _) };
                if adapter_name.to_str() == Ok(name) {
                    return Ok(Some(bytes));
                }
            }
        } else if bytes.iter().any(|&x| x != 0) {
            return Ok(Some(bytes));
        }

        // Otherwise go to the next device
        ptr = addr_lh.Next;
    }

    Ok(None)
}

pub fn get_ifname(mac: &[u8; 6]) -> Result<Option<String>, MacAddressError> {
    let mut adapters = get_adapters()?;
    // Pointer to the current location in the linked list
    let mut ptr = adapters.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH;

    loop {
        // Break if we've gone through all devices
        if ptr.is_null() {
            break;
        }

        let addr_lh = unsafe { ptr.read_unaligned() };

        let bytes = convert_mac_bytes(&addr_lh);

        if &bytes == mac {
            let adapter_name = unsafe { U16CStr::from_ptr_str(addr_lh.FriendlyName) };
            let adapter_name = adapter_name
                .to_string()
                .map_err(|_| MacAddressError::StringError)?;
            return Ok(Some(adapter_name));
        }

        // Otherwise go to the next device
        ptr = addr_lh.Next;
    }

    Ok(None)
}

/// Copy over the 6 MAC address bytes to the buffer.
pub(crate) fn convert_mac_bytes(addr_lh: &IP_ADAPTER_ADDRESSES_LH) -> [u8; 6] {
    addr_lh.PhysicalAddress[..6].try_into().unwrap()
}

pub(crate) fn get_adapters() -> Result<Vec<u8>, MacAddressError> {
    let mut buf_len = 0;

    // This will get the number of bytes we need to allocate for all devices
    unsafe {
        GetAdaptersAddresses(
            AF_UNSPEC as u32,
            0,
            ptr::null_mut(),
            ptr::null_mut(),
            &mut buf_len,
        );
    }

    // Allocate `buf_len` bytes, and create a raw pointer to it
    let mut adapters_list = vec![0u8; buf_len as usize];
    let adapter_addresses: *mut IP_ADAPTER_ADDRESSES_LH = adapters_list.as_mut_ptr() as *mut _;

    // Get our list of adapters
    let result = unsafe {
        GetAdaptersAddresses(
            AF_UNSPEC as u32,
            0,
            ptr::null_mut(),
            adapter_addresses as *mut _,
            &mut buf_len,
        )
    };

    // Make sure we were successful
    if result != 0 {
        return Err(MacAddressError::IoError(std::io::Error::from_raw_os_error(
            result as _,
        )));
    }

    Ok(adapters_list)
}
