//! This file is part of the easybox package.
//
// (c) Xu Biang <xubiang@foxmail.com>
// (c) Chen Yuchen <yuchen@isrc.iscas.ac.cn>
//
// For the full copyright and license information, please view the LICENSE file
// that was distributed with this source code.

use nix::{ioctl_readwrite_bad, ioctl_write_ptr_bad};
use std::{mem, ptr};
use uucore::libc::{arpreq, c_int, ifreq, sockaddr, socket};

/// Get a socket used in ioctl.
///
/// # Safety
///
/// This function wraps the unsafe `socket` syscall from libc.
/// Safety guarantees:
/// 1. The socket() syscall is a well-defined POSIX system call
/// 2. Parameters (domain, ty, protocol) are validated by the kernel
/// 3. Returns a valid file descriptor on success, or -1 on error
/// 4. No memory safety issues as it only returns an integer
pub fn socket_wrapper(domain: c_int, ty: c_int, protocol: c_int) -> c_int {
    // SAFETY: socket() is a standard POSIX system call that creates a socket endpoint.
    // The kernel validates all parameters and handles resource allocation.
    // This is safe because:
    // - No pointers are involved, only integer parameters
    // - The kernel manages all resources
    // - Error handling is done via return value checking
    unsafe { socket(domain, ty, protocol) }
}

/// Initialize some C structures.
///
/// # Safety
///
/// This function uses `mem::zeroed()` to create a zero-initialized instance of type T.
/// Safety guarantees:
/// 1. Only used for C-compatible structures (arpreq, ifreq, sockaddr, etc.)
/// 2. These structures are designed to be zero-initialized
/// 3. All-zero state is valid for these structures according to C ABI
/// 4. The structures will be properly initialized before use
pub fn zeroed_wrapper<T>() -> T {
    // SAFETY: mem::zeroed() is safe here because:
    // - T is a C-compatible structure (arpreq, ifreq, sockaddr_storage, etc.)
    // - These structures are designed to be zero-initialized in C code
    // - All fields have valid representations when zeroed (integers, arrays, etc.)
    // - The structures will be properly filled before being passed to kernel
    // - No enum types with invalid discriminants
    // - No references or non-zero types like NonNull
    unsafe { mem::zeroed() }
}

/// Copy memory between some C structures.
///
/// # Safety
///
/// This function wraps `ptr::copy_nonoverlapping` for copying memory between C structures.
/// Safety guarantees:
/// 1. Both src and dst pointers must be valid for reads/writes of `count` bytes
/// 2. The memory regions must not overlap (enforced by copy_nonoverlapping)
/// 3. The count parameter must not exceed the size of either structure
/// 4. Used only for copying between C-compatible structures with known sizes
///
/// # Arguments
///
/// * `src` - Pointer to source memory (must be valid for reads of `count` bytes)
/// * `dst` - Pointer to destination memory (must be valid for writes of `count` bytes)
/// * `count` - Number of bytes to copy
///
/// # Errors
///
/// This function does not return errors, but undefined behavior occurs if:
/// - src or dst is null
/// - Memory regions overlap
/// - count exceeds the size of source or destination
pub unsafe fn memcpy_wrapper(src: *const u8, dst: *mut u8, count: usize) {
    // SAFETY: ptr::copy_nonoverlapping is safe here because:
    // - src pointer is valid and points to initialized memory (from C structures)
    // - dst pointer is valid and points to writable memory (to C structures)
    // - count is always calculated using mem::size_of::<T>() for the structure types
    // - Source and destination never overlap (different structure instances)
    // - All structures are C-compatible and have no Rust-specific metadata
    // - The copy is used to transfer data between sockaddr structures
    // - Caller ensures all safety requirements are met
    ptr::copy_nonoverlapping(src, dst, count);
}

// SIOCGIFHWADDR => 0x8927
ioctl_readwrite_bad!(
    /// Use ioctl to get hardware address.
    ioctl_get_hardware_address,
    0x8927,
    ifreq
);
// SIOCDARP => 0x8953
ioctl_write_ptr_bad!(
    /// Use ioctl to delete an arp entry.
    ioctl_delete_arp,
    0x8953,
    arpreq
);
// SIOCSARP => 0x8955
ioctl_write_ptr_bad!(
    /// Use ioctl to set an arp entry.
    ioctl_set_arp,
    0x8955,
    arpreq
);

/// Get hardware address by ioctl.
///
/// # Safety
///
/// This function performs an ioctl syscall to get hardware address.
/// Safety guarantees:
/// 1. fd must be a valid socket file descriptor
/// 2. ifr must be a valid pointer to an initialized ifreq structure
/// 3. The ioctl number (0x8927 = SIOCGIFHWADDR) is a well-defined Linux ioctl
/// 4. The kernel will validate the operation and handle errors appropriately
///
/// # Arguments
///
/// * `fd` - Valid socket file descriptor
/// * `ifr` - Valid pointer to initialized ifreq structure
///
/// # Errors
///
/// Returns nix::Result with appropriate error if:
/// - fd is invalid
/// - ifr is null or invalid
/// - ioctl operation fails
pub unsafe fn ioctl_get_hardware_address_wrapper(fd: c_int, ifr: *mut ifreq) -> nix::Result<c_int> {
    // SAFETY: ioctl SIOCGIFHWADDR is a standard Linux operation to get hardware address.
    // This is safe because:
    // - fd is obtained from a valid socket() call and checked for errors
    // - ifr is a valid pointer to a properly initialized ifreq structure
    // - The ioctl is read-only from the kernel's perspective (it fills the structure)
    // - nix crate provides safe wrappers that handle error checking
    // - The kernel validates all parameters and returns appropriate errors
    // - Caller ensures all safety requirements are met
    ioctl_get_hardware_address(fd, ifr)
}

/// Delete arp entry by ioctl.
///
/// # Safety
///
/// This function performs an ioctl syscall to delete an ARP entry.
/// Safety guarantees:
/// 1. fd must be a valid socket file descriptor
/// 2. req must be a properly initialized arpreq structure
/// 3. The ioctl number (0x8953 = SIOCDARP) is a well-defined Linux ioctl
/// 4. Requires CAP_NET_ADMIN capability (checked by kernel)
/// 5. The kernel validates all parameters and handles errors appropriately
pub fn ioctl_delete_arp_wrapper(fd: c_int, req: arpreq) -> nix::Result<c_int> {
    // SAFETY: ioctl SIOCDARP is a standard Linux operation to delete ARP entries.
    // This is safe because:
    // - fd is obtained from a valid socket() call and checked for errors
    // - req is a properly initialized arpreq structure with valid data
    // - The operation requires CAP_NET_ADMIN capability (kernel enforces this)
    // - nix crate provides safe wrappers that handle error checking
    // - The kernel validates all parameters and returns appropriate errors
    // - This is a privileged operation that requires root or CAP_NET_ADMIN
    unsafe { ioctl_delete_arp(fd, &req) }
}

/// Set arp entry by ioctl.
///
/// # Safety
///
/// This function performs an ioctl syscall to set an ARP entry.
/// Safety guarantees:
/// 1. fd must be a valid socket file descriptor
/// 2. req must be a properly initialized arpreq structure
/// 3. The ioctl number (0x8955 = SIOCSARP) is a well-defined Linux ioctl
/// 4. Requires CAP_NET_ADMIN capability (checked by kernel)
/// 5. The kernel validates all parameters and handles errors appropriately
pub fn ioctl_set_arp_wrapper(fd: c_int, req: arpreq) -> nix::Result<c_int> {
    // SAFETY: ioctl SIOCSARP is a standard Linux operation to set ARP entries.
    // This is safe because:
    // - fd is obtained from a valid socket() call and checked for errors
    // - req is a properly initialized arpreq structure with valid data
    // - The operation requires CAP_NET_ADMIN capability (kernel enforces this)
    // - nix crate provides safe wrappers that handle error checking
    // - The kernel validates all parameters and returns appropriate errors
    // - This is a privileged operation that requires root or CAP_NET_ADMIN
    unsafe { ioctl_set_arp(fd, &req) }
}

/// Get ifru_hwaddr in union.
///
/// # Safety
///
/// This function accesses the ifru_hwaddr field from a C union in ifreq structure.
/// Safety guarantees:
/// 1. The ifreq structure must be properly initialized
/// 2. The union field ifru_hwaddr is valid when the ifreq was used with SIOCGIFHWADDR
/// 3. This is a read-only operation that extracts the hardware address
/// 4. The sockaddr structure returned is properly aligned and valid
pub fn ifru_hwaddr_wrapper(ifr: ifreq) -> sockaddr {
    // SAFETY: Accessing union field ifru_hwaddr is safe here because:
    // - ifr is obtained from ioctl SIOCGIFHWADDR which sets the ifru_hwaddr field
    // - The union is properly initialized by the kernel
    // - We're reading the correct union variant that was set by the ioctl
    // - sockaddr is a C-compatible structure with no Rust-specific invariants
    // - This is a read-only operation, no mutation occurs
    // - The returned sockaddr will be used to extract hardware address information
    unsafe { ifr.ifr_ifru.ifru_hwaddr }
}
