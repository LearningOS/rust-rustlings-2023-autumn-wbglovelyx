/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
///
/// The caller is responsible for ensuring that the provided `address` is a valid
/// pointer to a mutable `u32` value. Modifying the value at the given address is
/// considered safe under this condition.
unsafe fn modify_by_address(address: usize) {
    // SAFETY: The caller is responsible for providing a valid `address`.
    unsafe {
        let value = &mut *(address as *mut u32);
        *value = 0xAABBCCDD;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
