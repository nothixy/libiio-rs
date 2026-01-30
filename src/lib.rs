use std::ffi::CStr;
use std::ffi::CString;

fn cstr_to_string<'a>(char_array: *const libc::c_char) -> Result<&'a str, std::str::Utf8Error> {
    unsafe {
        let char_slice =
            std::slice::from_raw_parts(char_array as *mut u8, libc::strlen(char_array) + 1);
        let str = CStr::from_bytes_with_nul_unchecked(char_slice);
        str.to_str()
    }
}

pub struct ContextInfoList {
    ptr: *mut *mut IIOContextInfo,
    len: usize,
}

impl Drop for ContextInfoList {
    fn drop(&mut self) {
        unsafe {
            if !self.ptr.is_null() {
                iio_context_info_list_free(self.ptr);
            }
        }
    }
}

impl ContextInfoList {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn as_slice(&self) -> &[IIOContextInfo] {
        unsafe { std::slice::from_raw_parts(self.ptr as *const IIOContextInfo, self.len) }
    }

    pub fn iter(&self) -> impl Iterator<Item = &IIOContextInfo> {
        self.as_slice().iter()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct IIOContext {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

unsafe impl Send for IIOBuffer {}
unsafe impl Sync for IIOBuffer {}

unsafe impl Send for IIOContext {}
unsafe impl Sync for IIOContext {}

#[repr(C)]
pub struct IIOScanContext {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct IIOContextParams {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct IIOContextInfo {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub union IIOPointer {
    ctx: *const IIOContext,
    dev: *const IIODevice,
    chn: *const IIOChannel,
    buf: *const IIOBuffer,
}

#[repr(C)]
pub enum IIOAttrType {
    IIOAttrTypeDevice,
    IIOAttrTypeDebug,
    IIOAttrTypeBuffer,
    IIOAttrTypeChannel,
    IIOAttrTypeContext,
}

#[repr(C)]
pub struct IIOAttr {
    iio: IIOPointer,
    attr_type: IIOAttrType,
    name: *const libc::c_char,
    filename: *const libc::c_char,
}

#[repr(C)]
#[derive(Debug)]
pub struct IIODevice {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
#[derive(Debug)]
pub struct IIOChannel {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct IIOChannelsMask {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub struct IIOBlock {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
#[derive(Debug)]
pub struct IIODataFormat {
    pub length: libc::c_uint,
    pub bits: libc::c_uint,
    pub shift: libc::c_uint,
    pub is_signed: bool,
    pub is_fully_defined: bool,
    pub is_be: bool,
    pub with_scale: bool,
    pub scale: libc::c_double,
    pub repeat: libc::c_uint,
    pub offset: libc::c_double,
}

#[repr(C)]
#[derive(Debug)]
pub struct IIOBuffer {
    _data: (),
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

#[repr(C)]
pub enum IIOModifier {
    IIONoMod,
    IIOModX,
    IIOModY,
    IIOModZ,
    IIOModXAndY,
    IIOModXAndZ,
    IIOModYAndZ,
    IIOModXAndYAndZ,
    IIOModXOrY,
    IIOModXOrZ,
    IIOModYOrZ,
    IIOModXOrYOrZ,
    IIOModLightBoth,
    IIOModLightIr,
    IIOModRootSumSquaredXY,
    IIOModSumSquaredXYZ,
    IIOModLightClear,
    IIOModLightRed,
    IIOModLightGreen,
    IIOModLightBlue,
    IIOModQuaternion,
    IIOModTempAmbient,
    IIOModTempObject,
    IIOModNorthMagn,
    IIOModNorthTrue,
    IIOModNorthMagnTiltComp,
    IIOModNorthTrueTiltComp,
    IIOModRunning,
    IIOModJogging,
    IIOModWalking,
    IIOModStill,
    IIOModRootSumSquaredXYZ,
    IIOModI,
    IIOModQ,
    IIOModCo2,
    IIOModVoc,
    IIOModLightUv,
    IIOModLightDuv,
    IIOModPm1,
    IIOModPm2p5,
    IIOModPm4,
    IIOModPm10,
    IIOModEthanol,
    IIOModH2,
    IIOModO2,
    IIOModLinearX,
    IIOModLinearY,
    IIOModLinearZ,
    IIOModPitch,
    IIOModYaw,
    IIOModRoll,
    IIOModLightUva,
    IIOModLightUvb,
    IIOModRms,
    IIOModActive,
    IIOModReactive,
    IIOModApparent,
}

#[repr(C)]
pub enum IIOChanType {
    IioVoltage,
    IioCurrent,
    IioPower,
    IioAccel,
    IioAnglVel,
    IioMagn,
    IioLight,
    IioIntensity,
    IioProximity,
    IioTemp,
    IioIncli,
    IioRot,
    IioAngl,
    IioTimestamp,
    IioCapacitance,
    IioAltvoltage,
    IioCct,
    IioPressure,
    IioHumidityrelative,
    IioActivity,
    IioSteps,
    IioEnergy,
    IioDistance,
    IioVelocity,
    IioConcentration,
    IioResistance,
    IioPh,
    IioUvindex,
    IioElectricalconductivity,
    IioCount,
    IioIndex,
    IioGravity,
    IioPositionrelative,
    IioPhase,
    IioMassconcentration,
    IioDeltaAngl,
    IioDeltaVelocity,
    IioColortemp,
    IioChromaticity,
    IioAttention,
    IioAltcurrent,
    IioChanTypeUnknown = libc::INT_MAX as isize,
}

#[derive(Clone, Debug)]
pub struct IIOVersion {
    major: u32,
    minor: u32,
    tag: String,
}

impl IIOVersion {
    pub fn new(major: u32, minor: u32, tag: String) -> IIOVersion {
        IIOVersion { major, minor, tag }
    }

    pub fn get_major(&self) -> u32 {
        self.major
    }

    pub fn get_minor(&self) -> u32 {
        self.minor
    }

    pub fn get_tag(&self) -> &str {
        self.tag.as_str()
    }
}

impl Drop for IIOScanContext {
    fn drop(&mut self) {
        self.destroy()
    }
}

impl IIOScanContext {
    pub fn destroy(&mut self) {
        unsafe { iio_scan_context_destroy(self) }
    }

    pub fn new(backend: Option<&str>, flags: u32) -> Option<Box<IIOScanContext>> {
        unsafe {
            let context_ptr = if let Some(backend_str) = backend {
                iio_create_scan_context(
                    CString::new(backend_str).expect("Invalid String").as_ptr(),
                    flags,
                )
            } else {
                iio_create_scan_context(std::ptr::null(), flags)
            };
            if context_ptr.is_null() {
                None
            } else {
                Some(Box::from_raw(context_ptr))
            }
        }
    }

    pub fn get_info_list(&mut self) -> Result<ContextInfoList, ()> {
        unsafe {
            let mut info_ptr: *mut *mut IIOContextInfo = std::ptr::null_mut();
            let len = iio_scan_context_get_info_list(self, &mut info_ptr);

            if len < 0 || info_ptr.is_null() {
                return Err(());
            }

            Ok(ContextInfoList {
                ptr: info_ptr,
                len: len as usize,
            })
        }
    }
}

impl IIOContextInfo {
    pub fn get_description<'a>(&self) -> Result<&'a str, std::str::Utf8Error> {
        unsafe {
            let char_array = iio_context_info_get_description(self);
            cstr_to_string(char_array)
        }
    }

    pub fn get_uri<'a>(&self) -> Result<&'a str, std::str::Utf8Error> {
        unsafe {
            let char_array = iio_context_info_get_uri(self);
            cstr_to_string(char_array)
        }
    }
}

impl Drop for IIOContext {
    fn drop(&mut self) {
        self.destroy()
    }
}

impl IIOContext {
    pub fn destroy(&mut self) {
        unsafe { iio_context_destroy(self) }
    }

    pub fn get_name<'a>(&self) -> Result<&'a str, std::str::Utf8Error> {
        unsafe {
            let char_array = iio_context_get_name(self);
            cstr_to_string(char_array)
        }
    }

    pub fn get_description<'a>(&self) -> Result<&'a str, std::str::Utf8Error> {
        unsafe {
            let char_array = iio_context_get_description(self);
            cstr_to_string(char_array)
        }
    }

    pub fn get_xml<'a>(&self) -> Result<&'a str, std::str::Utf8Error> {
        unsafe {
            let char_array = iio_context_get_xml(self);
            cstr_to_string(char_array)
        }
    }

    pub fn get_version(&self) -> Result<IIOVersion, i32> {
        unsafe {
            let mut git_tag_buf = [0 as libc::c_char; 8];
            let git_tag_cstr = std::ptr::addr_of_mut!(git_tag_buf) as *mut libc::c_char;
            let mut major = 0;
            let mut minor = 0;
            let return_code = iio_context_get_version(
                self,
                std::ptr::addr_of_mut!(major),
                std::ptr::addr_of_mut!(minor),
                git_tag_cstr,
            );
            if return_code < 0 {
                Err(return_code)
            } else {
                let sliced = std::slice::from_raw_parts(
                    git_tag_cstr as *mut u8,
                    libc::strlen(git_tag_cstr) + 1,
                );
                let stringed = CStr::from_bytes_with_nul_unchecked(sliced)
                    .to_string_lossy()
                    .to_string();
                let version = IIOVersion::new(major, minor, stringed);
                Ok(version)
            }
        }
    }

    pub fn get_attrs_count(&mut self) -> u32 {
        unsafe { iio_context_get_attrs_count(self) }
    }

    pub fn get_attr<'a>(&self, index: u32) -> Result<(&'a str, &'a str), ()> {
        unsafe {
            let mut name = std::ptr::null_mut::<libc::c_char>();
            let mut value = std::ptr::null_mut::<libc::c_char>();
            let ioattr_res = iio_context_get_attr(
                self,
                index,
                std::ptr::addr_of_mut!(name),
                std::ptr::addr_of_mut!(value),
            );
            if ioattr_res < 0 {
                Err(())
            } else {
                let name_slice =
                    std::slice::from_raw_parts(name as *mut u8, libc::strlen(name) + 1);
                let value_slice =
                    std::slice::from_raw_parts(value as *mut u8, libc::strlen(value) + 1);
                let name_str = CStr::from_bytes_with_nul_unchecked(name_slice)
                    .to_str()
                    .map_err(|_| ())?;
                let value_str = CStr::from_bytes_with_nul_unchecked(value_slice)
                    .to_str()
                    .map_err(|_| ())?;
                Ok((name_str, value_str))
            }
        }
    }

    pub fn get_devices_count(&self) -> u32 {
        unsafe { iio_context_get_devices_count(self) }
    }

    pub fn get_device(&self, index: u32) -> Option<&IIODevice> {
        unsafe { iio_context_get_device(self, index).as_ref() }
    }

    pub fn find_device(&self, name: &str) -> Result<&IIODevice, ()> {
        unsafe {
            let device =
                iio_context_find_device(self, CString::new(name).expect("Invalid String").as_ptr());
            if device.is_null() {
                Err(())
            } else {
                Ok(&*device)
            }
        }
    }

    pub fn set_timeout(&self, timeout_ms: u32) -> Result<(), i32> {
        unsafe {
            let res = iio_context_set_timeout(self, timeout_ms);
            if res < 0 { Err(res) } else { Ok(()) }
        }
    }

    pub fn context_clone(&self) -> Option<Box<IIOContext>> {
        unsafe {
            let context_ptr = iio_context_clone(self);
            if context_ptr.is_null() {
                None
            } else {
                Some(Box::from_raw(context_ptr))
            }
        }
    }

    pub fn create_local() -> Option<Box<IIOContext>> {
        unsafe {
            let context_ptr = iio_create_local_context();
            if context_ptr.is_null() {
                None
            } else {
                Some(Box::from_raw(context_ptr))
            }
        }
    }

    pub fn create_xml(xml_file: &str) -> Option<Box<IIOContext>> {
        unsafe {
            let context_ptr =
                iio_create_xml_context(CString::new(xml_file).expect("Invalid String").as_ptr());
            if context_ptr.is_null() {
                None
            } else {
                Some(Box::from_raw(context_ptr))
            }
        }
    }

    pub fn create_network(host: &str) -> Option<Box<IIOContext>> {
        unsafe {
            let context_ptr =
                iio_create_network_context(CString::new(host).expect("Invalid String").as_ptr());
            if context_ptr.is_null() {
                None
            } else {
                Some(Box::from_raw(context_ptr))
            }
        }
    }

    pub fn create_default() -> Option<Box<IIOContext>> {
        unsafe {
            let context_ptr = iio_create_default_context();
            if context_ptr.is_null() {
                None
            } else {
                Some(Box::from_raw(context_ptr))
            }
        }
    }

    pub fn create_from_uri(uri: &str) -> Option<Box<IIOContext>> {
        unsafe {
            let string = CString::new(uri).expect("Invalid String");
            let context_ptr = iio_create_context_from_uri(string.as_ptr());
            if context_ptr.is_null() {
                None
            } else {
                Some(Box::from_raw(context_ptr))
            }
        }
    }
}

impl IIODevice {
    pub fn get_id(&self) -> Result<String, std::str::Utf8Error> {
        unsafe {
            let char_array = iio_device_get_id(self);
            Ok(cstr_to_string(char_array)?.to_owned())
        }
    }

    pub fn get_name(&self) -> Result<String, std::str::Utf8Error> {
        unsafe {
            let char_array = iio_device_get_name(self);
            Ok(cstr_to_string(char_array)?.to_owned())
        }
    }

    pub fn get_label(&self) -> Option<String> {
        unsafe {
            let char_array = iio_device_get_label(self);
            if char_array.is_null() {
                return None;
            }
            cstr_to_string(char_array)
                .map(|f| Some(f.to_owned()))
                .unwrap_or(None)
        }
    }

    pub fn get_attrs_count(&self) -> u32 {
        unsafe { iio_device_get_attrs_count(self) }
    }

    pub fn get_attr(&self, index: u32) -> Result<&str, std::str::Utf8Error> {
        unsafe {
            let cstr = iio_device_get_attr(self, index);
            let sliced = std::slice::from_raw_parts(cstr as *mut u8, libc::strlen(cstr) + 1);
            let str = CStr::from_bytes_with_nul_unchecked(sliced);
            str.to_str()
        }
    }

    pub fn attr_read<'a>(&self, name: &str, len: usize) -> Result<(&'a str, isize), ()> {
        if len > 1024 {
            Err(())
        } else {
            unsafe {
                let mut dst = [0 as libc::c_char; 1024];
                let size = iio_device_attr_read(
                    self,
                    CString::new(name).expect("Invalid String").as_ptr(),
                    dst.as_mut_ptr(),
                    len,
                );
                let sliced = std::slice::from_raw_parts(
                    dst.as_mut_ptr() as *mut u8,
                    libc::strlen(dst.as_mut_ptr()) + 1,
                );
                let stringed = CStr::from_bytes_with_nul_unchecked(sliced)
                    .to_str()
                    .map_err(|_| ())?;
                Ok((stringed, size))
            }
        }
    }

    pub fn attr_write(&self, name: &str, src: &str) -> isize {
        unsafe {
            iio_device_attr_write(
                self,
                CString::new(name).expect("Invalid String").as_ptr(),
                CString::new(src).expect("Invalid String").as_ptr(),
            )
        }
    }

    pub fn get_debug_attrs_count(&self) -> u32 {
        unsafe { iio_device_get_debug_attrs_count(self) }
    }

    pub fn get_debug_attr<'a>(&self, index: u32) -> Result<&'a str, std::str::Utf8Error> {
        unsafe {
            let cstr = iio_device_get_debug_attr(self, index);
            let sliced = std::slice::from_raw_parts(cstr as *mut u8, libc::strlen(cstr) + 1);
            let str = CStr::from_bytes_with_nul_unchecked(sliced);
            str.to_str()
        }
    }

    pub fn debug_attr_read<'a>(&self, name: &str, len: usize) -> Result<(&'a str, isize), ()> {
        if len > 1024 {
            Err(())
        } else {
            unsafe {
                let mut dst = [0 as libc::c_char; 1024];
                let size = iio_device_debug_attr_read(
                    self,
                    CString::new(name).expect("Invalid String").as_ptr(),
                    dst.as_mut_ptr(),
                    len,
                );
                let sliced = std::slice::from_raw_parts(
                    dst.as_mut_ptr() as *mut u8,
                    libc::strlen(dst.as_mut_ptr()) + 1,
                );
                let stringed = CStr::from_bytes_with_nul_unchecked(sliced)
                    .to_str()
                    .map_err(|_| ())?;
                Ok((stringed, size))
            }
        }
    }

    pub fn debug_attr_write(&self, name: &str, src: &str) -> isize {
        unsafe {
            iio_device_debug_attr_write(
                self,
                CString::new(name).expect("Invalid String").as_ptr(),
                CString::new(src).expect("Invalid String").as_ptr(),
            )
        }
    }

    pub fn get_buffer_attrs_count(&self) -> u32 {
        unsafe { iio_device_get_buffer_attrs_count(self) }
    }

    pub fn get_buffer_attr(&self, index: u32) -> Result<&str, std::str::Utf8Error> {
        unsafe {
            let cstr = iio_device_get_buffer_attr(self, index);
            let sliced = std::slice::from_raw_parts(cstr as *mut u8, libc::strlen(cstr) + 1);
            let str = CStr::from_bytes_with_nul_unchecked(sliced);
            str.to_str()
        }
    }

    pub fn buffer_attr_read<'a>(&self, name: &str, len: usize) -> Result<(&'a str, isize), ()> {
        if len > 1024 {
            Err(())
        } else {
            unsafe {
                let mut dst = [0 as libc::c_char; 1024];
                let size = iio_device_buffer_attr_read(
                    self,
                    CString::new(name).expect("Invalid String").as_ptr(),
                    dst.as_mut_ptr(),
                    len,
                );
                let sliced = std::slice::from_raw_parts(
                    dst.as_mut_ptr() as *mut u8,
                    libc::strlen(dst.as_mut_ptr()) + 1,
                );
                let stringed = CStr::from_bytes_with_nul_unchecked(sliced)
                    .to_str()
                    .map_err(|_| ())?;
                Ok((stringed, size))
            }
        }
    }

    pub fn buffer_attr_write(&self, name: &str, src: &str) -> isize {
        unsafe {
            iio_device_buffer_attr_write(
                self,
                CString::new(name).expect("Invalid String").as_ptr(),
                CString::new(src).expect("Invalid String").as_ptr(),
            )
        }
    }

    pub fn get_context(&self) -> Option<&IIOContext> {
        unsafe { iio_device_get_context(self).as_ref() }
    }

    pub fn find_channel(&self, name: &str, output: bool) -> Result<&IIOChannel, ()> {
        unsafe {
            let channel = iio_device_find_channel(
                self,
                CString::new(name).expect("Invalid String").as_ptr(),
                output,
            );
            if channel.is_null() {
                Err(())
            } else {
                Ok(&*channel)
            }
        }
    }

    pub fn reg_write(&mut self, address: u32, value: u32) -> i32 {
        unsafe { iio_device_reg_write(self, address, value) }
    }

    pub fn reg_read(&mut self, address: u32) -> Result<u32, i32> {
        unsafe {
            let mut value: u32 = 0;
            let res = iio_device_reg_read(self, address, &mut value);
            if res < 0 { Err(res) } else { Ok(value) }
        }
    }

    pub fn get_channels_count(&self) -> u32 {
        unsafe { iio_device_get_channels_count(self) }
    }

    pub fn get_channel(&self, index: u32) -> Option<&IIOChannel> {
        unsafe { iio_device_get_channel(self, index).as_ref() }
    }

    pub fn get_sample_size(&self) -> isize {
        unsafe { iio_device_get_sample_size(self) }
    }

    pub fn is_trigger(&self) -> bool {
        unsafe { iio_device_is_trigger(self) }
    }

    pub fn get_trigger(&self) -> Option<&IIODevice> {
        unsafe { iio_device_get_trigger(self).as_ref() }
    }

    pub fn set_trigger(&self, trigger: &IIODevice) -> i32 {
        unsafe { iio_device_set_trigger(self, trigger) }
    }

    pub fn set_kernel_buffers_count(&self, nb_buffers: u32) -> Result<(), i32> {
        unsafe {
            let res = iio_device_set_kernel_buffers_count(self, nb_buffers);
            if res < 0 { Err(res) } else { Ok(()) }
        }
    }

    pub fn create_buffer(&self, samples_count: usize, cyclic: bool) -> Option<Box<IIOBuffer>> {
        unsafe {
            let buffer_ptr = iio_device_create_buffer(self, samples_count, cyclic);
            if buffer_ptr.is_null() {
                None
            } else {
                Some(Box::from_raw(buffer_ptr))
            }
        }
    }
}

impl IIOChannel {
    pub fn get_id(&self) -> String {
        unsafe {
            let cstr = iio_channel_get_id(self);
            let sliced = std::slice::from_raw_parts(cstr as *mut u8, libc::strlen(cstr) + 1);
            let str = CStr::from_bytes_with_nul_unchecked(sliced);
            str.to_string_lossy().to_string()
        }
    }

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            let cstr = iio_channel_get_name(self);
            if cstr.is_null() {
                return None;
            }
            let sliced = std::slice::from_raw_parts(cstr as *mut u8, libc::strlen(cstr) + 1);
            let str = CStr::from_bytes_with_nul_unchecked(sliced);
            Some(str.to_string_lossy().to_string())
        }
    }

    pub fn is_output(&self) -> bool {
        unsafe { iio_channel_is_output(self) }
    }

    pub fn is_scan_element(&self) -> bool {
        unsafe { iio_channel_is_scan_element(self) }
    }

    pub fn get_attrs_count(&self) -> u32 {
        unsafe { iio_channel_get_attrs_count(self) }
    }

    pub fn get_attr(&self, index: u32) -> String {
        unsafe {
            let cstr = iio_channel_get_attr(self, index);
            let sliced = std::slice::from_raw_parts(cstr as *mut u8, libc::strlen(cstr) + 1);
            let str = CStr::from_bytes_with_nul_unchecked(sliced);
            str.to_string_lossy().to_string()
        }
    }

    pub fn attr_get_filename(&self, name: &str) -> Result<&str, std::str::Utf8Error> {
        unsafe {
            let cstr = iio_channel_attr_get_filename(
                self,
                CString::new(name).expect("Invalid String").as_ptr(),
            );
            let sliced = std::slice::from_raw_parts(cstr as *mut u8, libc::strlen(cstr) + 1);
            let str = CStr::from_bytes_with_nul_unchecked(sliced);
            str.to_str()
        }
    }

    pub fn attr_read(&self, name: &str, len: usize) -> Result<(String, isize), ()> {
        if len > 1024 {
            Err(())
        } else {
            unsafe {
                let mut dst = [0 as libc::c_char; 1024];
                let size = iio_channel_attr_read(
                    self,
                    CString::new(name).expect("Invalid String").as_ptr(),
                    dst.as_mut_ptr(),
                    len,
                );
                let sliced = std::slice::from_raw_parts(
                    dst.as_mut_ptr() as *mut u8,
                    libc::strlen(dst.as_mut_ptr()) + 1,
                );
                let stringed = CStr::from_bytes_with_nul_unchecked(sliced)
                    .to_str()
                    .map_err(|_| ())?;
                // println!("Reading value = {}", stringed);
                Ok((stringed.to_owned(), size))
            }
        }
    }

    pub fn attr_write(&self, name: &str, src: &str) -> isize {
        // println!("Setting attribute {} to {}", name, src);
        unsafe {
            iio_channel_attr_write(
                self,
                CString::new(name).expect("Invalid String").as_ptr(),
                CString::new(src).expect("Invalid String").as_ptr(),
            )
        }
    }

    pub fn enable(&self) {
        unsafe { iio_channel_enable(self) }
    }

    pub fn disable(&self) {
        unsafe { iio_channel_disable(self) }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { iio_channel_is_enabled(self) }
    }

    pub fn read(&self, buffer: &IIOBuffer, len: usize, raw: bool) -> Result<Vec<u8>, ()> {
        unsafe {
            let mut buf = Vec::<u8>::with_capacity(len);
            let size = iio_channel_read(
                self,
                buffer,
                buf.as_mut_ptr() as *mut libc::c_void,
                len,
                raw,
            );
            buf.set_len(size);
            Ok(buf)
        }
    }

    pub fn read_raw(&self, buffer: &IIOBuffer, len: usize, raw: bool) -> Result<Vec<u8>, ()> {
        unsafe {
            let mut buf = Vec::<u8>::with_capacity(len);
            let size = iio_channel_read_raw(
                self,
                buffer,
                buf.as_mut_ptr() as *mut libc::c_void,
                len,
                raw,
            );
            buf.set_len(size);
            Ok(buf)
        }
    }

    pub fn write(&self, buffer: &mut IIOBuffer, src: Vec<u8>, raw: bool) -> usize {
        unsafe {
            iio_channel_write(
                self,
                buffer,
                src.as_ptr() as *mut libc::c_void,
                src.len(),
                raw,
            )
        }
    }

    pub fn write_raw(&self, buffer: &mut IIOBuffer, src: Vec<u8>, raw: bool) -> usize {
        unsafe {
            iio_channel_write_raw(
                self,
                buffer,
                src.as_ptr() as *mut libc::c_void,
                src.len(),
                raw,
            )
        }
    }

    pub fn get_device(&self) -> Option<&IIODevice> {
        unsafe { iio_channel_get_device(self).as_ref() }
    }

    pub fn get_index(&self) -> i64 {
        unsafe { iio_channel_get_index(self) }
    }

    pub fn get_data_format(&self) -> Option<&IIODataFormat> {
        unsafe { iio_channel_get_data_format(self).as_ref() }
    }

    pub fn get_modifier(&self) -> IIOModifier {
        unsafe { iio_channel_get_modifier(self) }
    }

    pub fn get_type(&self) -> IIOChanType {
        unsafe { iio_channel_get_type(self) }
    }
}

impl Drop for IIOBuffer {
    fn drop(&mut self) {
        self.destroy()
    }
}

impl IIOBuffer {
    pub fn destroy(&mut self) {
        unsafe { iio_buffer_destroy(self) }
    }

    pub fn refill(&mut self) -> isize {
        unsafe { iio_buffer_refill(self) }
    }

    pub fn push_partial(&mut self, samples_count: usize) -> isize {
        unsafe { iio_buffer_push_partial(self, samples_count) }
    }

    pub fn start(&self) -> *mut u8 {
        unsafe { iio_buffer_start(self) as *mut u8 }
    }

    pub fn end(&self) -> *mut u8 {
        unsafe { iio_buffer_end(self) as *mut u8 }
    }

    pub fn cancel(&mut self) {
        unsafe { iio_buffer_cancel(self) }
    }

    pub fn get_device(&self) -> Option<&IIODevice> {
        unsafe { iio_buffer_get_device(self).as_ref() }
    }

    pub fn get_poll_fd(&mut self) -> i32 {
        unsafe { iio_buffer_get_poll_fd(self) }
    }

    pub fn step(&self) -> i64 {
        unsafe { iio_buffer_step(self) }
    }

    pub fn set_blocking_mode(&mut self, blocking: bool) -> Result<(), i32> {
        unsafe {
            let res = iio_buffer_set_blocking_mode(self, blocking);
            if res < 0 { Err(res) } else { Ok(()) }
        }
    }
}

pub fn get_version() -> IIOVersion {
    unsafe {
        let mut git_tag_buf = [0 as libc::c_char; 8];
        let git_tag_cstr = std::ptr::addr_of_mut!(git_tag_buf) as *mut libc::c_char;
        let mut major = 0;
        let mut minor = 0;
        iio_library_get_version(
            std::ptr::addr_of_mut!(major),
            std::ptr::addr_of_mut!(minor),
            git_tag_cstr,
        );
        let sliced =
            std::slice::from_raw_parts(git_tag_cstr as *mut u8, libc::strlen(git_tag_cstr) + 1);
        let stringed = CStr::from_bytes_with_nul_unchecked(sliced)
            .to_string_lossy()
            .to_string();
        IIOVersion::new(major, minor, stringed)
    }
}

pub fn get_strerror(error_code: i32) -> Result<String, std::str::Utf8Error> {
    unsafe {
        let mut dst = [0 as libc::c_char; 1024];
        iio_strerror(error_code, dst.as_mut_ptr(), dst.len());
        let sliced = std::slice::from_raw_parts(
            dst.as_mut_ptr() as *mut u8,
            libc::strlen(dst.as_mut_ptr()) + 1,
        );
        let stringed = CStr::from_bytes_with_nul_unchecked(sliced).to_str()?;
        Ok(stringed.to_owned())
    }
}

#[link(name = "iio")]
#[allow(dead_code)]
unsafe extern "C" {
    fn iio_get_backends_count() -> libc::c_uint;
    fn iio_get_backend(index: libc::c_uint) -> *const libc::c_char;
    fn iio_create_scan_context(
        backend: *const libc::c_char,
        flags: libc::c_uint,
    ) -> *mut IIOScanContext;
    fn iio_scan_context_destroy(context: *mut IIOScanContext);
    fn iio_has_backend(params: *const IIOContextParams, backend: *const libc::c_char) -> bool;
    fn iio_strerror(err: libc::c_int, dst: *mut libc::c_char, len: libc::size_t);
    fn iio_scan_context_get_info_list(
        ctx: *mut IIOScanContext,
        info: *mut *mut *mut IIOContextInfo,
    ) -> libc::ssize_t;
    fn iio_context_info_list_free(info: *mut *mut IIOContextInfo);
    fn iio_context_info_get_description(info: *const IIOContextInfo) -> *const libc::c_char;
    fn iio_context_info_get_uri(info: *const IIOContextInfo) -> *const libc::c_char;
    fn iio_create_local_context() -> *mut IIOContext;
    fn iio_create_xml_context(xml_file: *const libc::c_char) -> *mut IIOContext;
    fn iio_create_network_context(host: *const libc::c_char) -> *mut IIOContext;
    fn iio_create_default_context() -> *mut IIOContext;
    fn iio_create_context_from_uri(uri: *const libc::c_char) -> *mut IIOContext;
    fn iio_context_destroy(ctx: *mut IIOContext);
    fn iio_context_get_name(ctx: *const IIOContext) -> *const libc::c_char;
    fn iio_context_get_description(ctx: *const IIOContext) -> *const libc::c_char;
    fn iio_context_get_xml(ctx: *const IIOContext) -> *mut libc::c_char;
    fn iio_library_get_version(
        major: *mut libc::c_uint,
        minor: *mut libc::c_uint,
        git_tag: *mut libc::c_char,
    );
    fn iio_context_get_version(
        ctx: *const IIOContext,
        major: *mut libc::c_uint,
        minor: *mut libc::c_uint,
        git_tag: *mut libc::c_char,
    ) -> libc::c_int;
    fn iio_context_get_attrs_count(ctx: *mut IIOContext) -> libc::c_uint;
    fn iio_context_get_attr(
        ctx: *const IIOContext,
        index: libc::c_uint,
        name: *const *mut libc::c_char,
        value: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn iio_context_get_devices_count(ctx: *const IIOContext) -> libc::c_uint;
    fn iio_context_get_device(ctx: *const IIOContext, index: libc::c_uint) -> *mut IIODevice;
    fn iio_context_find_device(ctx: *const IIOContext, name: *const libc::c_char)
    -> *mut IIODevice;
    fn iio_context_set_timeout(ctx: *const IIOContext, timeout_ms: libc::c_uint) -> libc::c_int;
    fn iio_context_clone(old_ctx: *const IIOContext) -> *mut IIOContext;
    fn iio_device_get_id(dev: *const IIODevice) -> *const libc::c_char;
    fn iio_device_get_name(dev: *const IIODevice) -> *const libc::c_char;
    fn iio_device_get_label(dev: *const IIODevice) -> *const libc::c_char;
    fn iio_device_get_attrs_count(dev: *const IIODevice) -> libc::c_uint;
    fn iio_device_get_attr(dev: *const IIODevice, index: libc::c_uint) -> *const libc::c_char;
    fn iio_device_attr_read(
        dev: *const IIODevice,
        name: *const libc::c_char,
        dst: *mut libc::c_char,
        len: libc::size_t,
    ) -> libc::ssize_t;
    fn iio_device_attr_write(
        dev: *const IIODevice,
        name: *const libc::c_char,
        src: *const libc::c_char,
    ) -> libc::ssize_t;
    fn iio_device_get_debug_attrs_count(dev: *const IIODevice) -> libc::c_uint;
    fn iio_device_get_debug_attr(dev: *const IIODevice, index: libc::c_uint)
    -> *const libc::c_char;
    fn iio_device_debug_attr_read(
        dev: *const IIODevice,
        name: *const libc::c_char,
        dst: *mut libc::c_char,
        len: libc::size_t,
    ) -> libc::ssize_t;
    fn iio_device_debug_attr_write(
        dev: *const IIODevice,
        name: *const libc::c_char,
        src: *const libc::c_char,
    ) -> libc::ssize_t;
    fn iio_device_get_buffer_attrs_count(dev: *const IIODevice) -> libc::c_uint;
    fn iio_device_get_buffer_attr(
        dev: *const IIODevice,
        index: libc::c_uint,
    ) -> *const libc::c_char;
    fn iio_device_buffer_attr_read(
        dev: *const IIODevice,
        name: *const libc::c_char,
        dst: *mut libc::c_char,
        len: libc::size_t,
    ) -> libc::ssize_t;
    fn iio_device_buffer_attr_write(
        dev: *const IIODevice,
        name: *const libc::c_char,
        src: *const libc::c_char,
    ) -> libc::ssize_t;
    fn iio_device_get_context(dev: *const IIODevice) -> *const IIOContext;
    fn iio_device_find_channel(
        dev: *const IIODevice,
        name: *const libc::c_char,
        output: bool,
    ) -> *mut IIOChannel;
    fn iio_device_reg_write(dev: *mut IIODevice, address: u32, value: u32) -> libc::c_int;
    fn iio_device_reg_read(dev: *mut IIODevice, address: u32, value: *mut u32) -> libc::c_int;
    fn iio_device_get_channels_count(dev: *const IIODevice) -> libc::c_uint;
    fn iio_device_get_channel(dev: *const IIODevice, index: libc::c_uint) -> *mut IIOChannel;
    fn iio_device_get_sample_size(dev: *const IIODevice) -> libc::ssize_t;
    // fn iio_device_get_sample_size(dev: *const IIODevice, mask: *const IIOChannelsMask) -> libc::ssize_t;
    fn iio_device_is_trigger(dev: *const IIODevice) -> bool;
    fn iio_device_get_trigger(dev: *const IIODevice) -> *mut IIODevice;
    fn iio_device_set_trigger(dev: *const IIODevice, trigger: *const IIODevice) -> libc::c_int;
    fn iio_device_set_kernel_buffers_count(
        dev: *const IIODevice,
        nb_buffers: libc::c_uint,
    ) -> libc::c_int;
    fn iio_device_create_buffer(
        dev: *const IIODevice,
        samples_count: usize,
        cyclic: bool,
    ) -> *mut IIOBuffer;
    // fn iio_device_create_buffer(dev: *const IIODevice, idx: libc::c_uint, mask: *const IIOChannelsMask) -> *mut IIOBuffer;
    fn iio_channel_get_id(chn: *const IIOChannel) -> *const libc::c_char;
    fn iio_channel_get_name(chn: *const IIOChannel) -> *const libc::c_char;
    fn iio_channel_is_output(chn: *const IIOChannel) -> bool;
    fn iio_channel_is_scan_element(chn: *const IIOChannel) -> bool;
    fn iio_channel_get_attrs_count(chn: *const IIOChannel) -> libc::c_uint;
    fn iio_channel_get_attr(chn: *const IIOChannel, index: libc::c_uint) -> *const libc::c_char;
    fn iio_channel_attr_get_filename(
        chn: *const IIOChannel,
        name: *const libc::c_char,
    ) -> *const libc::c_char;
    fn iio_channel_attr_read(
        chn: *const IIOChannel,
        name: *const libc::c_char,
        dst: *mut libc::c_char,
        len: libc::size_t,
    ) -> libc::ssize_t;
    fn iio_channel_attr_write(
        chn: *const IIOChannel,
        name: *const libc::c_char,
        src: *const libc::c_char,
    ) -> libc::ssize_t;
    fn iio_channel_enable(chn: *const IIOChannel);
    fn iio_channel_disable(chn: *const IIOChannel);
    fn iio_channel_is_enabled(chn: *const IIOChannel) -> bool;
    fn iio_channel_read(
        chn: *const IIOChannel,
        buffer: *const IIOBuffer,
        dst: *mut libc::c_void,
        len: libc::size_t,
        raw: bool,
    ) -> libc::size_t;
    fn iio_channel_read_raw(
        chn: *const IIOChannel,
        buffer: *const IIOBuffer,
        dst: *mut libc::c_void,
        len: libc::size_t,
        raw: bool,
    ) -> libc::size_t; // NOT FOUND
    fn iio_channel_write(
        chn: *const IIOChannel,
        buffer: *mut IIOBuffer,
        src: *const libc::c_void,
        len: libc::size_t,
        raw: bool,
    ) -> libc::size_t;
    fn iio_channel_write_raw(
        chn: *const IIOChannel,
        buffer: *mut IIOBuffer,
        src: *const libc::c_void,
        len: libc::size_t,
        raw: bool,
    ) -> libc::size_t; // NOT FOUND
    fn iio_channel_get_device(chn: *const IIOChannel) -> *const IIODevice;
    fn iio_channel_get_index(chn: *const IIOChannel) -> libc::c_long;
    fn iio_channel_get_data_format(chn: *const IIOChannel) -> *const IIODataFormat;
    fn iio_channel_get_modifier(chn: *const IIOChannel) -> IIOModifier;
    fn iio_channel_get_type(chn: *const IIOChannel) -> IIOChanType;
    fn iio_buffer_destroy(buf: *mut IIOBuffer);
    fn iio_buffer_refill(buf: *mut IIOBuffer) -> libc::ssize_t;
    fn iio_buffer_push_partial(buf: *mut IIOBuffer, samples_count: libc::size_t) -> libc::ssize_t;
    fn iio_buffer_start(buf: *const IIOBuffer) -> *mut libc::c_void;
    fn iio_buffer_end(buf: *const IIOBuffer) -> *mut libc::c_void;
    fn iio_buffer_cancel(buf: *mut IIOBuffer);
    fn iio_buffer_get_device(buf: *const IIOBuffer) -> *const IIODevice;
    fn iio_buffer_get_poll_fd(buf: *mut IIOBuffer) -> libc::c_int;
    fn iio_buffer_step(buf: *const IIOBuffer) -> libc::c_long;
    fn iio_buffer_set_blocking_mode(buf: *mut IIOBuffer, blocking: bool) -> libc::c_int;
}
