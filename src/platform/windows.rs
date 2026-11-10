use windows_sys::Win32::Devices::DeviceAndDriverInstallation::{
    CM_Get_DevNode_PropertyW, CM_Get_Device_Interface_PropertyW, CM_LOCATE_DEVNODE_NORMAL,
    CM_Locate_DevNodeW, CR_SUCCESS,
};
use windows_sys::Win32::Devices::Properties::{
    DEVPKEY_Device_ContainerId, DEVPKEY_Device_InstanceId, DEVPROP_TYPE_GUID, DEVPROP_TYPE_STRING,
};

#[allow(unsafe_code)] // Raw Win32 calls are confined to this validated FFI boundary.
pub fn container_id(path: &str) -> Option<[u8; 16]> {
    if path.contains('\0') {
        return None;
    }
    let path: Vec<u16> = path.encode_utf16().chain(Some(0)).collect();
    let mut instance = [0_u16; 200];
    let mut property_type = 0;
    let mut byte_count = u32::try_from(std::mem::size_of_val(&instance)).ok()?;
    // SAFETY: path is NUL-terminated; all output pointers refer to initialized, sized buffers.
    let result = unsafe {
        CM_Get_Device_Interface_PropertyW(
            path.as_ptr(),
            &DEVPKEY_Device_InstanceId,
            &raw mut property_type,
            instance.as_mut_ptr().cast(),
            &raw mut byte_count,
            0,
        )
    };
    let length = usize::try_from(byte_count).ok()?;
    if result != CR_SUCCESS
        || property_type != DEVPROP_TYPE_STRING
        || length < 2
        || length > std::mem::size_of_val(&instance)
        || length % 2 != 0
        || instance[..length / 2]
            .iter()
            .position(|unit| *unit == 0)
            .is_none_or(|index| index == 0)
    {
        return None;
    }
    let mut devnode = 0;
    // SAFETY: instance contains a validated nonempty NUL-terminated UTF-16 string.
    let result = unsafe {
        CM_Locate_DevNodeW(
            &raw mut devnode,
            instance.as_ptr(),
            CM_LOCATE_DEVNODE_NORMAL,
        )
    };
    if result != CR_SUCCESS {
        return None;
    }
    let mut container = [0_u8; 16];
    byte_count = 16;
    // SAFETY: devnode was returned by Windows; the property buffer holds exactly 16 writable bytes.
    let result = unsafe {
        CM_Get_DevNode_PropertyW(
            devnode,
            &DEVPKEY_Device_ContainerId,
            &raw mut property_type,
            container.as_mut_ptr(),
            &raw mut byte_count,
            0,
        )
    };
    (result == CR_SUCCESS
        && property_type == DEVPROP_TYPE_GUID
        && byte_count == 16
        && container != [0; 16])
        .then_some(container)
}

#[cfg(test)]
mod tests {
    use super::container_id;

    #[test]
    fn embedded_nul_is_rejected_without_truncating_identity() {
        assert_eq!(container_id("synthetic\0interface"), None);
    }

    #[test]
    fn nonexistent_interface_has_no_container_metadata() {
        assert_eq!(container_id("synthetic://nonexistent-hid-interface"), None);
    }
}
