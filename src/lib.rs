#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::os::raw::{c_char, c_int, c_void};
#[cfg(test)] use std::ptr;
#[cfg(test)] use std::mem::{size_of, align_of};

pub const ONI_MAX_STR: usize = 256;
pub const ONI_MAX_SENSORS: usize = 10;

/// Possible failure values
pub type OniStatus = u32;
pub const OniStatus_ONI_STATUS_OK: OniStatus = 0;
pub const OniStatus_ONI_STATUS_ERROR: OniStatus = 1;
pub const OniStatus_ONI_STATUS_NOT_IMPLEMENTED: OniStatus = 2;
pub const OniStatus_ONI_STATUS_NOT_SUPPORTED: OniStatus = 3;
pub const OniStatus_ONI_STATUS_BAD_PARAMETER: OniStatus = 4;
pub const OniStatus_ONI_STATUS_OUT_OF_FLOW: OniStatus = 5;
pub const OniStatus_ONI_STATUS_NO_DEVICE: OniStatus = 6;
pub const OniStatus_ONI_STATUS_TIME_OUT: OniStatus = 102;

/// The source of the stream
pub type OniSensorType = u32;
pub const OniSensorType_ONI_SENSOR_IR: OniSensorType = 1;
pub const OniSensorType_ONI_SENSOR_COLOR: OniSensorType = 2;
pub const OniSensorType_ONI_SENSOR_DEPTH: OniSensorType = 3;

/// All available formats of the output of a stream
pub type OniPixelFormat = u32;
pub const OniPixelFormat_ONI_PIXEL_FORMAT_DEPTH_1_MM: OniPixelFormat = 100;
pub const OniPixelFormat_ONI_PIXEL_FORMAT_DEPTH_100_UM: OniPixelFormat = 101;
pub const OniPixelFormat_ONI_PIXEL_FORMAT_SHIFT_9_2: OniPixelFormat = 102;
pub const OniPixelFormat_ONI_PIXEL_FORMAT_SHIFT_9_3: OniPixelFormat = 103;
pub const OniPixelFormat_ONI_PIXEL_FORMAT_RGB888: OniPixelFormat = 200;
pub const OniPixelFormat_ONI_PIXEL_FORMAT_YUV422: OniPixelFormat = 201;
pub const OniPixelFormat_ONI_PIXEL_FORMAT_GRAY8: OniPixelFormat = 202;
pub const OniPixelFormat_ONI_PIXEL_FORMAT_GRAY16: OniPixelFormat = 203;
pub const OniPixelFormat_ONI_PIXEL_FORMAT_JPEG: OniPixelFormat = 204;
pub const OniPixelFormat_ONI_PIXEL_FORMAT_YUYV: OniPixelFormat = 205;

pub type OniDeviceState = u32;
pub const OniDeviceState_ONI_DEVICE_STATE_OK: OniDeviceState = 0;
pub const OniDeviceState_ONI_DEVICE_STATE_ERROR: OniDeviceState = 1;
pub const OniDeviceState_ONI_DEVICE_STATE_NOT_READY: OniDeviceState = 2;
pub const OniDeviceState_ONI_DEVICE_STATE_EOF: OniDeviceState = 3;

pub type OniImageRegistrationMode = u32;
pub const OniImageRegistrationMode_ONI_IMAGE_REGISTRATION_OFF: OniImageRegistrationMode = 0;
pub const OniImageRegistrationMode_ONI_IMAGE_REGISTRATION_DEPTH_TO_COLOR : OniImageRegistrationMode = 1 ;

pub type _bindgen_ty_1 = i32;
pub const ONI_TIMEOUT_NONE: _bindgen_ty_1 = 0;
pub const ONI_TIMEOUT_FOREVER: _bindgen_ty_1 = -1;

/// Basic types
pub type OniBool = c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OniCallbackHandleImpl {
    _unused: [u8; 0],
}
pub type OniCallbackHandle = *mut OniCallbackHandleImpl;
/// Holds an OpenNI version number, which consists of four separate numbers in the format: @c major.minor.maintenance.build. For example: 2.0.0.20.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OniVersion {
    /// Major version number, incremented for major API restructuring.
    pub major: c_int,
    /// Minor version number, incremented when significant new features added.
    pub minor: c_int,
    /// Maintenance build number, incremented for new releases that primarily provide minor bug fixes.
    pub maintenance: c_int,
    /// Build number. Incremented for each new API build. Generally not shown on the installer and download site.
    pub build: c_int,
}
#[test]
fn bindgen_test_layout_OniVersion() {
    assert_eq!(size_of::<OniVersion>(),
               16usize,
               concat!("Size of: ", stringify!(OniVersion)));
    assert_eq!(align_of::<OniVersion>(),
               4usize,
               concat!("Alignment of ", stringify!(OniVersion)));
    assert_eq!(unsafe { &(*(ptr::null::<OniVersion>())).major as *const _ as usize },
               0usize,
               concat!("Offset of field: ",
                       stringify!(OniVersion),
                       "::",
                       stringify!(major)));
    assert_eq!(unsafe { &(*(ptr::null::<OniVersion>())).minor as *const _ as usize },
               4usize,
               concat!("Offset of field: ",
                       stringify!(OniVersion),
                       "::",
                       stringify!(minor)));
    assert_eq!(unsafe { &(*(ptr::null::<OniVersion>())).maintenance as *const _ as usize },
               8usize,
               concat!("Offset of field: ",
                       stringify!(OniVersion),
                       "::",
                       stringify!(maintenance)));
    assert_eq!(unsafe { &(*(ptr::null::<OniVersion>())).build as *const _ as usize },
               12usize,
               concat!("Offset of field: ",
                       stringify!(OniVersion),
                       "::",
                       stringify!(build)));
}
pub type OniHardwareVersion = c_int;
/// Description of the output: format and resolution
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OniVideoMode {
    pub pixelFormat: OniPixelFormat,
    pub resolutionX: c_int,
    pub resolutionY: c_int,
    pub fps: c_int,
}
#[test]
fn bindgen_test_layout_OniVideoMode() {
    assert_eq!(size_of::<OniVideoMode>(),
               16usize,
               concat!("Size of: ", stringify!(OniVideoMode)));
    assert_eq!(align_of::<OniVideoMode>(),
               4usize,
               concat!("Alignment of ", stringify!(OniVideoMode)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<OniVideoMode>())).pixelFormat as *const _ as usize
               },
               0usize,
               concat!("Offset of field: ",
                       stringify!(OniVideoMode),
                       "::",
                       stringify!(pixelFormat)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<OniVideoMode>())).resolutionX as *const _ as usize
               },
               4usize,
               concat!("Offset of field: ",
                       stringify!(OniVideoMode),
                       "::",
                       stringify!(resolutionX)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<OniVideoMode>())).resolutionY as *const _ as usize
               },
               8usize,
               concat!("Offset of field: ",
                       stringify!(OniVideoMode),
                       "::",
                       stringify!(resolutionY)));
    assert_eq!(unsafe { &(*(ptr::null::<OniVideoMode>())).fps as *const _ as usize },
               12usize,
               concat!("Offset of field: ",
                       stringify!(OniVideoMode),
                       "::",
                       stringify!(fps)));
}
/// List of supported video modes by a specific source
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OniSensorInfo {
    pub sensorType: OniSensorType,
    pub numSupportedVideoModes: c_int,
    pub pSupportedVideoModes: *mut OniVideoMode,
}
#[test]
fn bindgen_test_layout_OniSensorInfo() {
    assert_eq!(size_of::<OniSensorInfo>(),
               16usize,
               concat!("Size of: ", stringify!(OniSensorInfo)));
    assert_eq!(align_of::<OniSensorInfo>(),
               8usize,
               concat!("Alignment of ", stringify!(OniSensorInfo)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<OniSensorInfo>())).sensorType as *const _ as usize
               },
               0usize,
               concat!("Offset of field: ",
                       stringify!(OniSensorInfo),
                       "::",
                       stringify!(sensorType)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<OniSensorInfo>())).numSupportedVideoModes as *const _ as
                   usize
               },
               4usize,
               concat!("Offset of field: ",
                       stringify!(OniSensorInfo),
                       "::",
                       stringify!(numSupportedVideoModes)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<OniSensorInfo>())).pSupportedVideoModes as *const _ as
                   usize
               },
               8usize,
               concat!("Offset of field: ",
                       stringify!(OniSensorInfo),
                       "::",
                       stringify!(pSupportedVideoModes)));
}
/// Basic description of a device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OniDeviceInfo {
    pub uri: [c_char; ONI_MAX_STR],
    pub vendor: [c_char; ONI_MAX_STR],
    pub name: [c_char; ONI_MAX_STR],
    pub usbVendorId: u16,
    pub usbProductId: u16,
}
#[test]
fn bindgen_test_layout_OniDeviceInfo() {
    assert_eq!(size_of::<OniDeviceInfo>(),
               772usize,
               concat!("Size of: ", stringify!(OniDeviceInfo)));
    assert_eq!(align_of::<OniDeviceInfo>(),
               2usize,
               concat!("Alignment of ", stringify!(OniDeviceInfo)));
    assert_eq!(unsafe { &(*(ptr::null::<OniDeviceInfo>())).uri as *const _ as usize },
               0usize,
               concat!("Offset of field: ",
                       stringify!(OniDeviceInfo),
                       "::",
                       stringify!(uri)));
    assert_eq!(unsafe { &(*(ptr::null::<OniDeviceInfo>())).vendor as *const _ as usize },
               256usize,
               concat!("Offset of field: ",
                       stringify!(OniDeviceInfo),
                       "::",
                       stringify!(vendor)));
    assert_eq!(unsafe { &(*(ptr::null::<OniDeviceInfo>())).name as *const _ as usize },
               512usize,
               concat!("Offset of field: ",
                       stringify!(OniDeviceInfo),
                       "::",
                       stringify!(name)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<OniDeviceInfo>())).usbVendorId as *const _ as usize
               },
               768usize,
               concat!("Offset of field: ",
                       stringify!(OniDeviceInfo),
                       "::",
                       stringify!(usbVendorId)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<OniDeviceInfo>())).usbProductId as *const _ as usize
               },
               770usize,
               concat!("Offset of field: ",
                       stringify!(OniDeviceInfo),
                       "::",
                       stringify!(usbProductId)));
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _OniDevice {
    _unused: [u8; 0],
}
pub type OniDeviceHandle = *mut _OniDevice;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _OniStream {
    _unused: [u8; 0],
}
pub type OniStreamHandle = *mut _OniStream;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _OniRecorder {
    _unused: [u8; 0],
}
pub type OniRecorderHandle = *mut _OniRecorder;
/// All information of the current frame
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OniFrame {
    pub dataSize: c_int,
    pub data: *mut c_void,
    pub sensorType: OniSensorType,
    pub timestamp: u64,
    pub frameIndex: c_int,
    pub width: c_int,
    pub height: c_int,
    pub videoMode: OniVideoMode,
    pub croppingEnabled: OniBool,
    pub cropOriginX: c_int,
    pub cropOriginY: c_int,
    pub stride: c_int,
}
#[test]
fn bindgen_test_layout_OniFrame() {
    assert_eq!(size_of::<OniFrame>(),
               80usize,
               concat!("Size of: ", stringify!(OniFrame)));
    assert_eq!(align_of::<OniFrame>(),
               8usize,
               concat!("Alignment of ", stringify!(OniFrame)));
    assert_eq!(unsafe { &(*(ptr::null::<OniFrame>())).dataSize as *const _ as usize },
               0usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(dataSize)));
    assert_eq!(unsafe { &(*(ptr::null::<OniFrame>())).data as *const _ as usize },
               8usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(data)));
    assert_eq!(unsafe { &(*(ptr::null::<OniFrame>())).sensorType as *const _ as usize },
               16usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(sensorType)));
    assert_eq!(unsafe { &(*(ptr::null::<OniFrame>())).timestamp as *const _ as usize },
               24usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(timestamp)));
    assert_eq!(unsafe { &(*(ptr::null::<OniFrame>())).frameIndex as *const _ as usize },
               32usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(frameIndex)));
    assert_eq!(unsafe { &(*(ptr::null::<OniFrame>())).width as *const _ as usize },
               36usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(width)));
    assert_eq!(unsafe { &(*(ptr::null::<OniFrame>())).height as *const _ as usize },
               40usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(height)));
    assert_eq!(unsafe { &(*(ptr::null::<OniFrame>())).videoMode as *const _ as usize },
               44usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(videoMode)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<OniFrame>())).croppingEnabled as *const _ as usize
               },
               60usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(croppingEnabled)));
    assert_eq!(unsafe { &(*(ptr::null::<OniFrame>())).cropOriginX as *const _ as usize },
               64usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(cropOriginX)));
    assert_eq!(unsafe { &(*(ptr::null::<OniFrame>())).cropOriginY as *const _ as usize },
               68usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(cropOriginY)));
    assert_eq!(unsafe { &(*(ptr::null::<OniFrame>())).stride as *const _ as usize },
               72usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(stride)));
}
pub type OniNewFrameCallback =
    ::std::option::Option<unsafe extern "C" fn(stream: OniStreamHandle,
                                               pCookie: *mut c_void)>;
pub type OniGeneralCallback =
    ::std::option::Option<unsafe extern "C" fn(pCookie: *mut c_void)>;
pub type OniDeviceInfoCallback =
    ::std::option::Option<unsafe extern "C" fn(pInfo: *const OniDeviceInfo,
                                               pCookie: *mut c_void)>;
pub type OniDeviceStateCallback =
    ::std::option::Option<unsafe extern "C" fn(pInfo: *const OniDeviceInfo,
                                               deviceState: OniDeviceState,
                                               pCookie: *mut c_void)>;
pub type OniFrameAllocBufferCallback =
    ::std::option::Option<unsafe extern "C" fn(size: c_int,
                                               pCookie: *mut c_void)
                                               -> *mut c_void>;
pub type OniFrameFreeBufferCallback =
    ::std::option::Option<unsafe extern "C" fn(data: *mut c_void,
                                               pCookie: *mut c_void)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OniDeviceCallbacks {
    pub deviceConnected: OniDeviceInfoCallback,
    pub deviceDisconnected: OniDeviceInfoCallback,
    pub deviceStateChanged: OniDeviceStateCallback,
}
#[test]
fn bindgen_test_layout_OniDeviceCallbacks() {
    assert_eq!(size_of::<OniDeviceCallbacks>(),
               24usize,
               concat!("Size of: ", stringify!(OniDeviceCallbacks)));
    assert_eq!(align_of::<OniDeviceCallbacks>(),
               8usize,
               concat!("Alignment of ", stringify!(OniDeviceCallbacks)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<OniDeviceCallbacks>())).deviceConnected as *const _ as
                   usize
               },
               0usize,
               concat!("Offset of field: ",
                       stringify!(OniDeviceCallbacks),
                       "::",
                       stringify!(deviceConnected)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<OniDeviceCallbacks>())).deviceDisconnected as *const _ as
                   usize
               },
               8usize,
               concat!("Offset of field: ",
                       stringify!(OniDeviceCallbacks),
                       "::",
                       stringify!(deviceDisconnected)));
    assert_eq!(unsafe {
                   &(*(ptr::null::<OniDeviceCallbacks>())).deviceStateChanged as *const _ as
                   usize
               },
               16usize,
               concat!("Offset of field: ",
                       stringify!(OniDeviceCallbacks),
                       "::",
                       stringify!(deviceStateChanged)));
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OniCropping {
    pub enabled: c_int,
    pub originX: c_int,
    pub originY: c_int,
    pub width: c_int,
    pub height: c_int,
}
#[test]
fn bindgen_test_layout_OniCropping() {
    assert_eq!(size_of::<OniCropping>(),
               20usize,
               concat!("Size of: ", stringify!(OniCropping)));
    assert_eq!(align_of::<OniCropping>(),
               4usize,
               concat!("Alignment of ", stringify!(OniCropping)));
    assert_eq!(unsafe { &(*(ptr::null::<OniCropping>())).enabled as *const _ as usize },
               0usize,
               concat!("Offset of field: ",
                       stringify!(OniCropping),
                       "::",
                       stringify!(enabled)));
    assert_eq!(unsafe { &(*(ptr::null::<OniCropping>())).originX as *const _ as usize },
               4usize,
               concat!("Offset of field: ",
                       stringify!(OniCropping),
                       "::",
                       stringify!(originX)));
    assert_eq!(unsafe { &(*(ptr::null::<OniCropping>())).originY as *const _ as usize },
               8usize,
               concat!("Offset of field: ",
                       stringify!(OniCropping),
                       "::",
                       stringify!(originY)));
    assert_eq!(unsafe { &(*(ptr::null::<OniCropping>())).width as *const _ as usize },
               12usize,
               concat!("Offset of field: ",
                       stringify!(OniCropping),
                       "::",
                       stringify!(width)));
    assert_eq!(unsafe { &(*(ptr::null::<OniCropping>())).height as *const _ as usize },
               16usize,
               concat!("Offset of field: ",
                       stringify!(OniCropping),
                       "::",
                       stringify!(height)));
}
/// Pixel type used to store depth images.
pub type OniDepthPixel = u16;
/// Pixel type used to store 16-bit grayscale images
pub type OniGrayscale16Pixel = u16;
/// Pixel type used to store 8-bit grayscale/bayer images
pub type OniGrayscale8Pixel = u8;
/// Holds the value of a single color image pixel in 24-bit RGB format.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OniRGB888Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
#[test]
fn bindgen_test_layout_OniRGB888Pixel() {
    assert_eq!(size_of::<OniRGB888Pixel>(),
               3usize,
               concat!("Size of: ", stringify!(OniRGB888Pixel)));
    assert_eq!(align_of::<OniRGB888Pixel>(),
               1usize,
               concat!("Alignment of ", stringify!(OniRGB888Pixel)));
    assert_eq!(unsafe { &(*(ptr::null::<OniRGB888Pixel>())).r as *const _ as usize },
               0usize,
               concat!("Offset of field: ",
                       stringify!(OniRGB888Pixel),
                       "::",
                       stringify!(r)));
    assert_eq!(unsafe { &(*(ptr::null::<OniRGB888Pixel>())).g as *const _ as usize },
               1usize,
               concat!("Offset of field: ",
                       stringify!(OniRGB888Pixel),
                       "::",
                       stringify!(g)));
    assert_eq!(unsafe { &(*(ptr::null::<OniRGB888Pixel>())).b as *const _ as usize },
               2usize,
               concat!("Offset of field: ",
                       stringify!(OniRGB888Pixel),
                       "::",
                       stringify!(b)));
}
/// Holds the value of two pixels in YUV422 format (Luminance/Chrominance,16-bits/pixel).
/// The first pixel has the values y1, u, v.
/// The second pixel has the values y2, u, v.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OniYUV422DoublePixel {
    /// First chrominance value for two pixels, stored as blue luminance difference signal.
    pub u: u8,
    /// Overall luminance value of first pixel.
    pub y1: u8,
    /// Second chrominance value for two pixels, stored as red luminance difference signal.
    pub v: u8,
    /// Overall luminance value of second pixel.
    pub y2: u8,
}
#[test]
fn bindgen_test_layout_OniYUV422DoublePixel() {
    assert_eq!(size_of::<OniYUV422DoublePixel>(),
               4usize,
               concat!("Size of: ", stringify!(OniYUV422DoublePixel)));
    assert_eq!(align_of::<OniYUV422DoublePixel>(),
               1usize,
               concat!("Alignment of ", stringify!(OniYUV422DoublePixel)));
    assert_eq!(unsafe { &(*(ptr::null::<OniYUV422DoublePixel>())).u as *const _ as usize },
               0usize,
               concat!("Offset of field: ",
                       stringify!(OniYUV422DoublePixel),
                       "::",
                       stringify!(u)));
    assert_eq!(unsafe { &(*(ptr::null::<OniYUV422DoublePixel>())).y1 as *const _ as usize },
               1usize,
               concat!("Offset of field: ",
                       stringify!(OniYUV422DoublePixel),
                       "::",
                       stringify!(y1)));
    assert_eq!(unsafe { &(*(ptr::null::<OniYUV422DoublePixel>())).v as *const _ as usize },
               2usize,
               concat!("Offset of field: ",
                       stringify!(OniYUV422DoublePixel),
                       "::",
                       stringify!(v)));
    assert_eq!(unsafe { &(*(ptr::null::<OniYUV422DoublePixel>())).y2 as *const _ as usize },
               3usize,
               concat!("Offset of field: ",
                       stringify!(OniYUV422DoublePixel),
                       "::",
                       stringify!(y2)));
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OniSeek {
    pub frameIndex: c_int,
    pub stream: OniStreamHandle,
}
#[test]
fn bindgen_test_layout_OniSeek() {
    assert_eq!(size_of::<OniSeek>(),
               16usize,
               concat!("Size of: ", stringify!(OniSeek)));
    assert_eq!(align_of::<OniSeek>(),
               8usize,
               concat!("Alignment of ", stringify!(OniSeek)));
    assert_eq!(unsafe { &(*(ptr::null::<OniSeek>())).frameIndex as *const _ as usize },
               0usize,
               concat!("Offset of field: ",
                       stringify!(OniSeek),
                       "::",
                       stringify!(frameIndex)));
    assert_eq!(unsafe { &(*(ptr::null::<OniSeek>())).stream as *const _ as usize },
               8usize,
               concat!("Offset of field: ",
                       stringify!(OniSeek),
                       "::",
                       stringify!(stream)));
}
pub const ONI_DEVICE_PROPERTY_FIRMWARE_VERSION: _bindgen_ty_2 = 0;
pub const ONI_DEVICE_PROPERTY_DRIVER_VERSION: _bindgen_ty_2 = 1;
pub const ONI_DEVICE_PROPERTY_HARDWARE_VERSION: _bindgen_ty_2 = 2;
pub const ONI_DEVICE_PROPERTY_SERIAL_NUMBER: _bindgen_ty_2 = 3;
pub const ONI_DEVICE_PROPERTY_ERROR_STATE: _bindgen_ty_2 = 4;
pub const ONI_DEVICE_PROPERTY_IMAGE_REGISTRATION: _bindgen_ty_2 = 5;
pub const ONI_DEVICE_PROPERTY_PLAYBACK_SPEED: _bindgen_ty_2 = 100;
pub const ONI_DEVICE_PROPERTY_PLAYBACK_REPEAT_ENABLED: _bindgen_ty_2 = 101;
pub type _bindgen_ty_2 = u32;
pub const ONI_STREAM_PROPERTY_CROPPING: _bindgen_ty_3 = 0;
pub const ONI_STREAM_PROPERTY_HORIZONTAL_FOV: _bindgen_ty_3 = 1;
pub const ONI_STREAM_PROPERTY_VERTICAL_FOV: _bindgen_ty_3 = 2;
pub const ONI_STREAM_PROPERTY_VIDEO_MODE: _bindgen_ty_3 = 3;
pub const ONI_STREAM_PROPERTY_MAX_VALUE: _bindgen_ty_3 = 4;
pub const ONI_STREAM_PROPERTY_MIN_VALUE: _bindgen_ty_3 = 5;
pub const ONI_STREAM_PROPERTY_STRIDE: _bindgen_ty_3 = 6;
pub const ONI_STREAM_PROPERTY_MIRRORING: _bindgen_ty_3 = 7;
pub const ONI_STREAM_PROPERTY_NUMBER_OF_FRAMES: _bindgen_ty_3 = 8;
pub const ONI_STREAM_PROPERTY_AUTO_WHITE_BALANCE: _bindgen_ty_3 = 100;
pub const ONI_STREAM_PROPERTY_AUTO_EXPOSURE: _bindgen_ty_3 = 101;
pub const ONI_STREAM_PROPERTY_EXPOSURE: _bindgen_ty_3 = 102;
pub const ONI_STREAM_PROPERTY_GAIN: _bindgen_ty_3 = 103;
pub type _bindgen_ty_3 = u32;
pub const ONI_DEVICE_COMMAND_SEEK: _bindgen_ty_4 = 1;
pub type _bindgen_ty_4 = u32;
extern "C" {
    /// Initialize OpenNI2. Use ONI_API_VERSION as the version.
    pub fn oniInitialize(apiVersion: c_int) -> OniStatus;
}
extern "C" {
    /// Shutdown OpenNI2
    pub fn oniShutdown();
}
extern "C" {
    /// Get the list of currently connected device.
    /// Each device is represented by its OniDeviceInfo.
    /// pDevices will be allocated inside.
    pub fn oniGetDeviceList(pDevices: *mut *mut OniDeviceInfo,
                            pNumDevices: *mut c_int)
                            -> OniStatus;
}
extern "C" {
    /// Release previously allocated device list
    pub fn oniReleaseDeviceList(pDevices: *mut OniDeviceInfo) -> OniStatus;
}
extern "C" {
    pub fn oniRegisterDeviceCallbacks(pCallbacks: *mut OniDeviceCallbacks,
                                      pCookie: *mut c_void,
                                      pHandle: *mut OniCallbackHandle)
                                      -> OniStatus;
}
extern "C" {
    pub fn oniUnregisterDeviceCallbacks(handle: OniCallbackHandle);
}
extern "C" {
    /// Wait for any of the streams to have a new frame
    pub fn oniWaitForAnyStream(pStreams: *mut OniStreamHandle,
                               numStreams: c_int,
                               pStreamIndex: *mut c_int,
                               timeout: c_int)
                               -> OniStatus;
}
extern "C" {
    /// Get the current version of OpenNI2
    pub fn oniGetVersion() -> OniVersion;
}
extern "C" {
    /// Translate from format to number of bytes per pixel. Will return 0 for formats in which the number of bytes per pixel isn't fixed.
    pub fn oniFormatBytesPerPixel(format: OniPixelFormat) -> c_int;
}
extern "C" {
    /// Get internal error
    pub fn oniGetExtendedError() -> *const c_char;
}
extern "C" {
    /// Open a device. Uri can be taken from the matching OniDeviceInfo.
    pub fn oniDeviceOpen(uri: *const c_char,
                         pDevice: *mut OniDeviceHandle)
                         -> OniStatus;
}
extern "C" {
    /// Close a device
    pub fn oniDeviceClose(device: OniDeviceHandle) -> OniStatus;
}
extern "C" {
    /// Get the possible configurations available for a specific source, or NULL if the source does not exist.
    pub fn oniDeviceGetSensorInfo(device: OniDeviceHandle,
                                  sensorType: OniSensorType)
                                  -> *const OniSensorInfo;
}
extern "C" {
    /// Get the OniDeviceInfo of a certain device.
    pub fn oniDeviceGetInfo(device: OniDeviceHandle, pInfo: *mut OniDeviceInfo) -> OniStatus;
}
extern "C" {
    /// Create a new stream in the device. The stream will originate from the source.
    pub fn oniDeviceCreateStream(device: OniDeviceHandle,
                                 sensorType: OniSensorType,
                                 pStream: *mut OniStreamHandle)
                                 -> OniStatus;
}
extern "C" {
    pub fn oniDeviceEnableDepthColorSync(device: OniDeviceHandle) -> OniStatus;
}
extern "C" {
    pub fn oniDeviceDisableDepthColorSync(device: OniDeviceHandle);
}
extern "C" {
    pub fn oniDeviceGetDepthColorSyncEnabled(device: OniDeviceHandle) -> OniBool;
}
extern "C" {
    /// Set property in the device. Use the properties listed in OniTypes.h: ONI_DEVICE_PROPERTY_..., or specific ones supplied by the device.
    pub fn oniDeviceSetProperty(device: OniDeviceHandle,
                                propertyId: c_int,
                                data: *const c_void,
                                dataSize: c_int)
                                -> OniStatus;
}
extern "C" {
    /// Get property in the device. Use the properties listed in OniTypes.h: ONI_DEVICE_PROPERTY_..., or specific ones supplied by the device.
    pub fn oniDeviceGetProperty(device: OniDeviceHandle,
                                propertyId: c_int,
                                data: *mut c_void,
                                pDataSize: *mut c_int)
                                -> OniStatus;
}
extern "C" {
    /// Check if the property is supported by the device. Use the properties listed in OniTypes.h: ONI_DEVICE_PROPERTY_..., or specific ones supplied by the device.
    pub fn oniDeviceIsPropertySupported(device: OniDeviceHandle,
                                        propertyId: c_int)
                                        -> OniBool;
}
extern "C" {
    /// Invoke an internal functionality of the device.
    pub fn oniDeviceInvoke(device: OniDeviceHandle,
                           commandId: c_int,
                           data: *mut c_void,
                           dataSize: c_int)
                           -> OniStatus;
}
extern "C" {
    /// Check if a command is supported, for invoke
    pub fn oniDeviceIsCommandSupported(device: OniDeviceHandle,
                                       commandId: c_int)
                                       -> OniBool;
}
extern "C" {
    pub fn oniDeviceIsImageRegistrationModeSupported(device: OniDeviceHandle,
                                                     mode: OniImageRegistrationMode)
                                                     -> OniBool;
}
extern "C" {
    /// @internal
    pub fn oniDeviceOpenEx(uri: *const c_char,
                           mode: *const c_char,
                           pDevice: *mut OniDeviceHandle)
                           -> OniStatus;
}
extern "C" {
    /// Destroy an existing stream
    pub fn oniStreamDestroy(stream: OniStreamHandle);
}
extern "C" {
    /// Get the OniSensorInfo of the certain stream.
    pub fn oniStreamGetSensorInfo(stream: OniStreamHandle) -> *const OniSensorInfo;
}
extern "C" {
    /// Start generating data from the stream.
    pub fn oniStreamStart(stream: OniStreamHandle) -> OniStatus;
}
extern "C" {
    /// Stop generating data from the stream.
    pub fn oniStreamStop(stream: OniStreamHandle);
}
extern "C" {
    /// Get the next frame from the stream. This function is blocking until there is a new frame from the stream. For timeout, use oniWaitForStreams() first
    pub fn oniStreamReadFrame(stream: OniStreamHandle, pFrame: *mut *mut OniFrame) -> OniStatus;
}
extern "C" {
    /// Register a callback to when the stream has a new frame.
    pub fn oniStreamRegisterNewFrameCallback(stream: OniStreamHandle,
                                             handler: OniNewFrameCallback,
                                             pCookie: *mut c_void,
                                             pHandle: *mut OniCallbackHandle)
                                             -> OniStatus;
}
extern "C" {
    /// Unregister a previously registered callback to when the stream has a new frame.
    pub fn oniStreamUnregisterNewFrameCallback(stream: OniStreamHandle,
                                               handle: OniCallbackHandle);
}
extern "C" {
    /// Set property in the stream. Use the properties listed in OniTypes.h: ONI_STREAM_PROPERTY_..., or specific ones supplied by the device for its streams.
    pub fn oniStreamSetProperty(stream: OniStreamHandle,
                                propertyId: c_int,
                                data: *const c_void,
                                dataSize: c_int)
                                -> OniStatus;
}
extern "C" {
    /// Get property in the stream. Use the properties listed in OniTypes.h: ONI_STREAM_PROPERTY_..., or specific ones supplied by the device for its streams.
    pub fn oniStreamGetProperty(stream: OniStreamHandle,
                                propertyId: c_int,
                                data: *mut c_void,
                                pDataSize: *mut c_int)
                                -> OniStatus;
}
extern "C" {
    /// Check if the property is supported the stream. Use the properties listed in OniTypes.h: ONI_STREAM_PROPERTY_..., or specific ones supplied by the device for its streams.
    pub fn oniStreamIsPropertySupported(stream: OniStreamHandle,
                                        propertyId: c_int)
                                        -> OniBool;
}
extern "C" {
    /// Invoke an internal functionality of the stream.
    pub fn oniStreamInvoke(stream: OniStreamHandle,
                           commandId: c_int,
                           data: *mut c_void,
                           dataSize: c_int)
                           -> OniStatus;
}
extern "C" {
    /// Check if a command is supported, for invoke
    pub fn oniStreamIsCommandSupported(stream: OniStreamHandle,
                                       commandId: c_int)
                                       -> OniBool;
}
extern "C" {
    /// Sets the stream buffer allocation functions. Note that this function may only be called while stream is not started.
    pub fn oniStreamSetFrameBuffersAllocator(stream: OniStreamHandle,
                                             alloc: OniFrameAllocBufferCallback,
                                             free: OniFrameFreeBufferCallback,
                                             pCookie: *mut c_void)
                                             -> OniStatus;
}
extern "C" {
    ///
    /// ** Mark another user of the frame. */
    pub fn oniFrameAddRef(pFrame: *mut OniFrame);
}
extern "C" {
    /// Mark that the frame is no longer needed.
    pub fn oniFrameRelease(pFrame: *mut OniFrame);
}
extern "C" {
    /// Creates a recorder that records to a file.
    /// @param	[in]	fileName	The name of the file that will contain the recording.
    /// @param	[out]	pRecorder	Points to the handle to the newly created recorder.
    /// @retval ONI_STATUS_OK Upon successful completion.
    /// @retval ONI_STATUS_ERROR Upon any kind of failure.
    pub fn oniCreateRecorder(fileName: *const c_char,
                             pRecorder: *mut OniRecorderHandle)
                             -> OniStatus;
}
extern "C" {
    /// Attaches a stream to a recorder. The amount of attached streams is virtually
    /// infinite. You cannot attach a stream after you have started a recording, if
    /// you do: an error will be returned by oniRecorderAttachStream.
    /// @param	[in]	recorder				The handle to the recorder.
    /// @param	[in]	stream					The handle to the stream.
    /// @param	[in]	allowLossyCompression	Allows/denies lossy compression
    /// @retval ONI_STATUS_OK Upon successful completion.
    /// @retval ONI_STATUS_ERROR Upon any kind of failure.
    pub fn oniRecorderAttachStream(recorder: OniRecorderHandle,
                                   stream: OniStreamHandle,
                                   allowLossyCompression: OniBool)
                                   -> OniStatus;
}
extern "C" {
    /// Starts recording. There must be at least one stream attached to the recorder,
    /// if not: oniRecorderStart will return an error.
    /// @param[in] recorder The handle to the recorder.
    /// @retval ONI_STATUS_OK Upon successful completion.
    /// @retval ONI_STATUS_ERROR Upon any kind of failure.
    pub fn oniRecorderStart(recorder: OniRecorderHandle) -> OniStatus;
}
extern "C" {
    /// Stops recording. You can resume recording via oniRecorderStart.
    /// @param[in] recorder The handle to the recorder.
    /// @retval ONI_STATUS_OK Upon successful completion.
    /// @retval ONI_STATUS_ERROR Upon any kind of failure.
    pub fn oniRecorderStop(recorder: OniRecorderHandle);
}
extern "C" {
    /// Stops recording if needed, and destroys a recorder.
    /// @param	[in,out]	recorder	The handle to the recorder, the handle will be
    /// invalidated (nullified) when the function returns.
    /// @retval ONI_STATUS_OK Upon successful completion.
    /// @retval ONI_STATUS_ERROR Upon any kind of failure.
    pub fn oniRecorderDestroy(pRecorder: *mut OniRecorderHandle) -> OniStatus;
}
extern "C" {
    pub fn oniCoordinateConverterDepthToWorld(depthStream: OniStreamHandle,
                                              depthX: f32,
                                              depthY: f32,
                                              depthZ: f32,
                                              pWorldX: *mut f32,
                                              pWorldY: *mut f32,
                                              pWorldZ: *mut f32)
                                              -> OniStatus;
}
extern "C" {
    pub fn oniCoordinateConverterWorldToDepth(depthStream: OniStreamHandle,
                                              worldX: f32,
                                              worldY: f32,
                                              worldZ: f32,
                                              pDepthX: *mut f32,
                                              pDepthY: *mut f32,
                                              pDepthZ: *mut f32)
                                              -> OniStatus;
}
extern "C" {
    pub fn oniCoordinateConverterDepthToColor(depthStream: OniStreamHandle,
                                              colorStream: OniStreamHandle,
                                              depthX: c_int,
                                              depthY: c_int,
                                              depthZ: OniDepthPixel,
                                              pColorX: *mut c_int,
                                              pColorY: *mut c_int)
                                              -> OniStatus;
}
extern "C" {
    /// Change the log output folder
    ///
    /// @param const char * strOutputFolder	[in]	path to the desirebale folder
    ///
    /// @retval ONI_STATUS_OK Upon successful completion.
    /// @retval ONI_STATUS_ERROR Upon any kind of failure.
    pub fn oniSetLogOutputFolder(strOutputFolder: *const c_char) -> OniStatus;
}
extern "C" {
    /// Get the current log file name
    ///
    /// @param	char * strFileName	[out]	hold the returned file name
    /// @param	int nBufferSize	[in]	size of strFileName
    ///
    /// @retval ONI_STATUS_OK Upon successful completion.
    /// @retval ONI_STATUS_ERROR Upon any kind of failure.
    pub fn oniGetLogFileName(strFileName: *mut c_char,
                             nBufferSize: c_int)
                             -> OniStatus;
}
extern "C" {
    /// Set the Minimum severity for log produce
    ///
    /// @param	const char * strMask	[in]	Name of the logger
    ///
    /// @retval ONI_STATUS_OK Upon successful completion.
    /// @retval ONI_STATUS_ERROR Upon any kind of failure.
    pub fn oniSetLogMinSeverity(nMinSeverity: c_int) -> OniStatus;
}
extern "C" {
    /// Configures if log entries will be printed to console.
    ///
    /// @param	OniBool bConsoleOutput	[in]	TRUE to print log entries to console, FALSE otherwise.
    ///
    /// @retval ONI_STATUS_OK Upon successful completion.
    /// @retval ONI_STATUS_ERROR Upon any kind of failure.
    pub fn oniSetLogConsoleOutput(bConsoleOutput: OniBool) -> OniStatus;
}
extern "C" {
    /// Configures if log entries will be printed to a log file.
    ///
    /// @param	OniBool bFileOutput	[in]	TRUE to print log entries to the file, FALSE otherwise.
    ///
    /// @retval ONI_STATUS_OK Upon successful completion.
    /// @retval ONI_STATUS_ERROR Upon any kind of failure.
    pub fn oniSetLogFileOutput(bFileOutput: OniBool) -> OniStatus;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_oniGetVersion() {
        let version: OniVersion = unsafe { oniGetVersion() };
        assert_eq!(version.major, 2);
        assert_eq!(version.minor, 2);
        assert_eq!(version.maintenance, 0);
        assert_eq!(version.build, 33);
    }
}
