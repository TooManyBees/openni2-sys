#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use std::os::raw::{c_char, c_int, c_void};

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
    assert_eq!(::std::mem::size_of::<OniVersion>(),
               16usize,
               concat!("Size of: ", stringify!(OniVersion)));
    assert_eq!(::std::mem::align_of::<OniVersion>(),
               4usize,
               concat!("Alignment of ", stringify!(OniVersion)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<OniVersion>())).major as *const _ as usize },
               0usize,
               concat!("Offset of field: ",
                       stringify!(OniVersion),
                       "::",
                       stringify!(major)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<OniVersion>())).minor as *const _ as usize },
               4usize,
               concat!("Offset of field: ",
                       stringify!(OniVersion),
                       "::",
                       stringify!(minor)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<OniVersion>())).maintenance as *const _ as usize },
               8usize,
               concat!("Offset of field: ",
                       stringify!(OniVersion),
                       "::",
                       stringify!(maintenance)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<OniVersion>())).build as *const _ as usize },
               12usize,
               concat!("Offset of field: ",
                       stringify!(OniVersion),
                       "::",
                       stringify!(build)));
}
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
    assert_eq!(::std::mem::size_of::<OniVideoMode>(),
               16usize,
               concat!("Size of: ", stringify!(OniVideoMode)));
    assert_eq!(::std::mem::align_of::<OniVideoMode>(),
               4usize,
               concat!("Alignment of ", stringify!(OniVideoMode)));
    assert_eq!(unsafe {
                   &(*(::std::ptr::null::<OniVideoMode>())).pixelFormat as *const _ as usize
               },
               0usize,
               concat!("Offset of field: ",
                       stringify!(OniVideoMode),
                       "::",
                       stringify!(pixelFormat)));
    assert_eq!(unsafe {
                   &(*(::std::ptr::null::<OniVideoMode>())).resolutionX as *const _ as usize
               },
               4usize,
               concat!("Offset of field: ",
                       stringify!(OniVideoMode),
                       "::",
                       stringify!(resolutionX)));
    assert_eq!(unsafe {
                   &(*(::std::ptr::null::<OniVideoMode>())).resolutionY as *const _ as usize
               },
               8usize,
               concat!("Offset of field: ",
                       stringify!(OniVideoMode),
                       "::",
                       stringify!(resolutionY)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<OniVideoMode>())).fps as *const _ as usize },
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
    assert_eq!(::std::mem::size_of::<OniSensorInfo>(),
               16usize,
               concat!("Size of: ", stringify!(OniSensorInfo)));
    assert_eq!(::std::mem::align_of::<OniSensorInfo>(),
               8usize,
               concat!("Alignment of ", stringify!(OniSensorInfo)));
    assert_eq!(unsafe {
                   &(*(::std::ptr::null::<OniSensorInfo>())).sensorType as *const _ as usize
               },
               0usize,
               concat!("Offset of field: ",
                       stringify!(OniSensorInfo),
                       "::",
                       stringify!(sensorType)));
    assert_eq!(unsafe {
                   &(*(::std::ptr::null::<OniSensorInfo>())).numSupportedVideoModes as *const _ as
                   usize
               },
               4usize,
               concat!("Offset of field: ",
                       stringify!(OniSensorInfo),
                       "::",
                       stringify!(numSupportedVideoModes)));
    assert_eq!(unsafe {
                   &(*(::std::ptr::null::<OniSensorInfo>())).pSupportedVideoModes as *const _ as
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
    pub uri: [c_char; 256usize],
    pub vendor: [c_char; 256usize],
    pub name: [c_char; 256usize],
    pub usbVendorId: u16,
    pub usbProductId: u16,
}
#[test]
fn bindgen_test_layout_OniDeviceInfo() {
    assert_eq!(::std::mem::size_of::<OniDeviceInfo>(),
               772usize,
               concat!("Size of: ", stringify!(OniDeviceInfo)));
    assert_eq!(::std::mem::align_of::<OniDeviceInfo>(),
               2usize,
               concat!("Alignment of ", stringify!(OniDeviceInfo)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<OniDeviceInfo>())).uri as *const _ as usize },
               0usize,
               concat!("Offset of field: ",
                       stringify!(OniDeviceInfo),
                       "::",
                       stringify!(uri)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<OniDeviceInfo>())).vendor as *const _ as usize },
               256usize,
               concat!("Offset of field: ",
                       stringify!(OniDeviceInfo),
                       "::",
                       stringify!(vendor)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<OniDeviceInfo>())).name as *const _ as usize },
               512usize,
               concat!("Offset of field: ",
                       stringify!(OniDeviceInfo),
                       "::",
                       stringify!(name)));
    assert_eq!(unsafe {
                   &(*(::std::ptr::null::<OniDeviceInfo>())).usbVendorId as *const _ as usize
               },
               768usize,
               concat!("Offset of field: ",
                       stringify!(OniDeviceInfo),
                       "::",
                       stringify!(usbVendorId)));
    assert_eq!(unsafe {
                   &(*(::std::ptr::null::<OniDeviceInfo>())).usbProductId as *const _ as usize
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
    assert_eq!(::std::mem::size_of::<OniFrame>(),
               80usize,
               concat!("Size of: ", stringify!(OniFrame)));
    assert_eq!(::std::mem::align_of::<OniFrame>(),
               8usize,
               concat!("Alignment of ", stringify!(OniFrame)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<OniFrame>())).dataSize as *const _ as usize },
               0usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(dataSize)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<OniFrame>())).data as *const _ as usize },
               8usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(data)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<OniFrame>())).sensorType as *const _ as usize },
               16usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(sensorType)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<OniFrame>())).timestamp as *const _ as usize },
               24usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(timestamp)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<OniFrame>())).frameIndex as *const _ as usize },
               32usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(frameIndex)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<OniFrame>())).width as *const _ as usize },
               36usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(width)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<OniFrame>())).height as *const _ as usize },
               40usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(height)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<OniFrame>())).videoMode as *const _ as usize },
               44usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(videoMode)));
    assert_eq!(unsafe {
                   &(*(::std::ptr::null::<OniFrame>())).croppingEnabled as *const _ as usize
               },
               60usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(croppingEnabled)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<OniFrame>())).cropOriginX as *const _ as usize },
               64usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(cropOriginX)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<OniFrame>())).cropOriginY as *const _ as usize },
               68usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(cropOriginY)));
    assert_eq!(unsafe { &(*(::std::ptr::null::<OniFrame>())).stride as *const _ as usize },
               72usize,
               concat!("Offset of field: ",
                       stringify!(OniFrame),
                       "::",
                       stringify!(stride)));
}
pub type OniNewFrameCallback =
    ::std::option::Option<unsafe extern "C" fn(stream: OniStreamHandle,
                                               pCookie: *mut c_void)>;
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
    assert_eq!(::std::mem::size_of::<OniDeviceCallbacks>(),
               24usize,
               concat!("Size of: ", stringify!(OniDeviceCallbacks)));
    assert_eq!(::std::mem::align_of::<OniDeviceCallbacks>(),
               8usize,
               concat!("Alignment of ", stringify!(OniDeviceCallbacks)));
    assert_eq!(unsafe {
                   &(*(::std::ptr::null::<OniDeviceCallbacks>())).deviceConnected as *const _ as
                   usize
               },
               0usize,
               concat!("Offset of field: ",
                       stringify!(OniDeviceCallbacks),
                       "::",
                       stringify!(deviceConnected)));
    assert_eq!(unsafe {
                   &(*(::std::ptr::null::<OniDeviceCallbacks>())).deviceDisconnected as *const _ as
                   usize
               },
               8usize,
               concat!("Offset of field: ",
                       stringify!(OniDeviceCallbacks),
                       "::",
                       stringify!(deviceDisconnected)));
    assert_eq!(unsafe {
                   &(*(::std::ptr::null::<OniDeviceCallbacks>())).deviceStateChanged as *const _ as
                   usize
               },
               16usize,
               concat!("Offset of field: ",
                       stringify!(OniDeviceCallbacks),
                       "::",
                       stringify!(deviceStateChanged)));
}
/// Pixel type used to store depth images.
pub type OniDepthPixel = u16;
extern "C" {
    /// Initialize OpenNI2. Use ONI_API_VERSION as the version.
    #[link_name = "\u{1}_oniInitialize"]
    pub fn oniInitialize(apiVersion: c_int) -> OniStatus;

    /// Shutdown OpenNI2
    #[link_name = "\u{1}_oniShutdown"]
    pub fn oniShutdown();

    /// Get the list of currently connected device.
    /// Each device is represented by its OniDeviceInfo.
    /// pDevices will be allocated inside.
    #[link_name = "\u{1}_oniGetDeviceList"]
    pub fn oniGetDeviceList(pDevices: *mut *mut OniDeviceInfo,
                            pNumDevices: *mut c_int)
                            -> OniStatus;

    /// Release previously allocated device list
    #[link_name = "\u{1}_oniReleaseDeviceList"]
    pub fn oniReleaseDeviceList(pDevices: *mut OniDeviceInfo) -> OniStatus;

    #[link_name = "\u{1}_oniRegisterDeviceCallbacks"]
    pub fn oniRegisterDeviceCallbacks(pCallbacks: *mut OniDeviceCallbacks,
                                      pCookie: *mut c_void,
                                      pHandle: *mut OniCallbackHandle)
                                      -> OniStatus;

    #[link_name = "\u{1}_oniUnregisterDeviceCallbacks"]
    pub fn oniUnregisterDeviceCallbacks(handle: OniCallbackHandle);


    /// Wait for any of the streams to have a new frame
    #[link_name = "\u{1}_oniWaitForAnyStream"]
    pub fn oniWaitForAnyStream(pStreams: *mut OniStreamHandle,
                               numStreams: c_int,
                               pStreamIndex: *mut c_int,
                               timeout: c_int)
                               -> OniStatus;


    /// Get the current version of OpenNI2
    #[link_name = "\u{1}_oniGetVersion"]
    pub fn oniGetVersion() -> OniVersion;


    /// Translate from format to number of bytes per pixel. Will return 0 for formats in which the number of bytes per pixel isn't fixed.
    #[link_name = "\u{1}_oniFormatBytesPerPixel"]
    pub fn oniFormatBytesPerPixel(format: OniPixelFormat) -> c_int;


    /// Get internal error
    #[link_name = "\u{1}_oniGetExtendedError"]
    pub fn oniGetExtendedError() -> *const c_char;


    /// Open a device. Uri can be taken from the matching OniDeviceInfo.
    #[link_name = "\u{1}_oniDeviceOpen"]
    pub fn oniDeviceOpen(uri: *const c_char,
                         pDevice: *mut OniDeviceHandle)
                         -> OniStatus;


    /// Close a device
    #[link_name = "\u{1}_oniDeviceClose"]
    pub fn oniDeviceClose(device: OniDeviceHandle) -> OniStatus;


    /// Get the possible configurations available for a specific source, or NULL if the source does not exist.
    #[link_name = "\u{1}_oniDeviceGetSensorInfo"]
    pub fn oniDeviceGetSensorInfo(device: OniDeviceHandle,
                                  sensorType: OniSensorType)
                                  -> *const OniSensorInfo;


    /// Get the OniDeviceInfo of a certain device.
    #[link_name = "\u{1}_oniDeviceGetInfo"]
    pub fn oniDeviceGetInfo(device: OniDeviceHandle, pInfo: *mut OniDeviceInfo) -> OniStatus;


    /// Create a new stream in the device. The stream will originate from the source.
    #[link_name = "\u{1}_oniDeviceCreateStream"]
    pub fn oniDeviceCreateStream(device: OniDeviceHandle,
                                 sensorType: OniSensorType,
                                 pStream: *mut OniStreamHandle)
                                 -> OniStatus;

    #[link_name = "\u{1}_oniDeviceEnableDepthColorSync"]
    pub fn oniDeviceEnableDepthColorSync(device: OniDeviceHandle) -> OniStatus;

    #[link_name = "\u{1}_oniDeviceDisableDepthColorSync"]
    pub fn oniDeviceDisableDepthColorSync(device: OniDeviceHandle);

    #[link_name = "\u{1}_oniDeviceGetDepthColorSyncEnabled"]
    pub fn oniDeviceGetDepthColorSyncEnabled(device: OniDeviceHandle) -> OniBool;

    /// Set property in the device. Use the properties listed in OniTypes.h: ONI_DEVICE_PROPERTY_..., or specific ones supplied by the device.
    #[link_name = "\u{1}_oniDeviceSetProperty"]
    pub fn oniDeviceSetProperty(device: OniDeviceHandle,
                                propertyId: c_int,
                                data: *const c_void,
                                dataSize: c_int)
                                -> OniStatus;

    /// Get property in the device. Use the properties listed in OniTypes.h: ONI_DEVICE_PROPERTY_..., or specific ones supplied by the device.
    #[link_name = "\u{1}_oniDeviceGetProperty"]
    pub fn oniDeviceGetProperty(device: OniDeviceHandle,
                                propertyId: c_int,
                                data: *mut c_void,
                                pDataSize: *mut c_int)
                                -> OniStatus;

    /// Check if the property is supported by the device. Use the properties listed in OniTypes.h: ONI_DEVICE_PROPERTY_..., or specific ones supplied by the device.
    #[link_name = "\u{1}_oniDeviceIsPropertySupported"]
    pub fn oniDeviceIsPropertySupported(device: OniDeviceHandle,
                                        propertyId: c_int)
                                        -> OniBool;

    /// Invoke an internal functionality of the device.
    #[link_name = "\u{1}_oniDeviceInvoke"]
    pub fn oniDeviceInvoke(device: OniDeviceHandle,
                           commandId: c_int,
                           data: *mut c_void,
                           dataSize: c_int)
                           -> OniStatus;

    /// Check if a command is supported, for invoke
    #[link_name = "\u{1}_oniDeviceIsCommandSupported"]
    pub fn oniDeviceIsCommandSupported(device: OniDeviceHandle,
                                       commandId: c_int)
                                       -> OniBool;

    #[link_name = "\u{1}_oniDeviceIsImageRegistrationModeSupported"]
    pub fn oniDeviceIsImageRegistrationModeSupported(device: OniDeviceHandle,
                                                     mode: OniImageRegistrationMode)
                                                     -> OniBool;

    /// @internal
    #[link_name = "\u{1}_oniDeviceOpenEx"]
    pub fn oniDeviceOpenEx(uri: *const c_char,
                           mode: *const c_char,
                           pDevice: *mut OniDeviceHandle)
                           -> OniStatus;

    /// Destroy an existing stream
    #[link_name = "\u{1}_oniStreamDestroy"]
    pub fn oniStreamDestroy(stream: OniStreamHandle);

    /// Get the OniSensorInfo of the certain stream.
    #[link_name = "\u{1}_oniStreamGetSensorInfo"]
    pub fn oniStreamGetSensorInfo(stream: OniStreamHandle) -> *const OniSensorInfo;

    /// Start generating data from the stream.
    #[link_name = "\u{1}_oniStreamStart"]
    pub fn oniStreamStart(stream: OniStreamHandle) -> OniStatus;

    /// Stop generating data from the stream.
    #[link_name = "\u{1}_oniStreamStop"]
    pub fn oniStreamStop(stream: OniStreamHandle);

    /// Get the next frame from the stream. This function is blocking until there is a new frame from the stream. For timeout, use oniWaitForStreams() first
    #[link_name = "\u{1}_oniStreamReadFrame"]
    pub fn oniStreamReadFrame(stream: OniStreamHandle, pFrame: *mut *mut OniFrame) -> OniStatus;

    /// Register a callback to when the stream has a new frame.
    #[link_name = "\u{1}_oniStreamRegisterNewFrameCallback"]
    pub fn oniStreamRegisterNewFrameCallback(stream: OniStreamHandle,
                                             handler: OniNewFrameCallback,
                                             pCookie: *mut c_void,
                                             pHandle: *mut OniCallbackHandle)
                                             -> OniStatus;

    /// Unregister a previously registered callback to when the stream has a new frame.
    #[link_name = "\u{1}_oniStreamUnregisterNewFrameCallback"]
    pub fn oniStreamUnregisterNewFrameCallback(stream: OniStreamHandle,
                                               handle: OniCallbackHandle);

    /// Set property in the stream. Use the properties listed in OniTypes.h: ONI_STREAM_PROPERTY_..., or specific ones supplied by the device for its streams.
    #[link_name = "\u{1}_oniStreamSetProperty"]
    pub fn oniStreamSetProperty(stream: OniStreamHandle,
                                propertyId: c_int,
                                data: *const c_void,
                                dataSize: c_int)
                                -> OniStatus;

    /// Get property in the stream. Use the properties listed in OniTypes.h: ONI_STREAM_PROPERTY_..., or specific ones supplied by the device for its streams.
    #[link_name = "\u{1}_oniStreamGetProperty"]
    pub fn oniStreamGetProperty(stream: OniStreamHandle,
                                propertyId: c_int,
                                data: *mut c_void,
                                pDataSize: *mut c_int)
                                -> OniStatus;

    /// Check if the property is supported the stream. Use the properties listed in OniTypes.h: ONI_STREAM_PROPERTY_..., or specific ones supplied by the device for its streams.
    #[link_name = "\u{1}_oniStreamIsPropertySupported"]
    pub fn oniStreamIsPropertySupported(stream: OniStreamHandle,
                                        propertyId: c_int)
                                        -> OniBool;

    /// Invoke an internal functionality of the stream.
    #[link_name = "\u{1}_oniStreamInvoke"]
    pub fn oniStreamInvoke(stream: OniStreamHandle,
                           commandId: c_int,
                           data: *mut c_void,
                           dataSize: c_int)
                           -> OniStatus;

    /// Check if a command is supported, for invoke
    #[link_name = "\u{1}_oniStreamIsCommandSupported"]
    pub fn oniStreamIsCommandSupported(stream: OniStreamHandle,
                                       commandId: c_int)
                                       -> OniBool;

    /// Sets the stream buffer allocation functions. Note that this function may only be called while stream is not started.
    #[link_name = "\u{1}_oniStreamSetFrameBuffersAllocator"]
    pub fn oniStreamSetFrameBuffersAllocator(stream: OniStreamHandle,
                                             alloc: OniFrameAllocBufferCallback,
                                             free: OniFrameFreeBufferCallback,
                                             pCookie: *mut c_void)
                                             -> OniStatus;

    ///
    /// ** Mark another user of the frame. */
    #[link_name = "\u{1}_oniFrameAddRef"]
    pub fn oniFrameAddRef(pFrame: *mut OniFrame);

    /// Mark that the frame is no longer needed.
    #[link_name = "\u{1}_oniFrameRelease"]
    pub fn oniFrameRelease(pFrame: *mut OniFrame);

    /// Creates a recorder that records to a file.
    /// @param  [in]    fileName    The name of the file that will contain the recording.
    /// @param  [out]   pRecorder   Points to the handle to the newly created recorder.
    /// @retval ONI_STATUS_OK Upon successful completion.
    /// @retval ONI_STATUS_ERROR Upon any kind of failure.
    #[link_name = "\u{1}_oniCreateRecorder"]
    pub fn oniCreateRecorder(fileName: *const c_char,
                             pRecorder: *mut OniRecorderHandle)
                             -> OniStatus;

    /// Attaches a stream to a recorder. The amount of attached streams is virtually
    /// infinite. You cannot attach a stream after you have started a recording, if
    /// you do: an error will be returned by oniRecorderAttachStream.
    /// @param  [in]    recorder                The handle to the recorder.
    /// @param  [in]    stream                  The handle to the stream.
    /// @param  [in]    allowLossyCompression   Allows/denies lossy compression
    /// @retval ONI_STATUS_OK Upon successful completion.
    /// @retval ONI_STATUS_ERROR Upon any kind of failure.
    #[link_name = "\u{1}_oniRecorderAttachStream"]
    pub fn oniRecorderAttachStream(recorder: OniRecorderHandle,
                                   stream: OniStreamHandle,
                                   allowLossyCompression: OniBool)
                                   -> OniStatus;

    /// Starts recording. There must be at least one stream attached to the recorder,
    /// if not: oniRecorderStart will return an error.
    /// @param[in] recorder The handle to the recorder.
    /// @retval ONI_STATUS_OK Upon successful completion.
    /// @retval ONI_STATUS_ERROR Upon any kind of failure.
    #[link_name = "\u{1}_oniRecorderStart"]
    pub fn oniRecorderStart(recorder: OniRecorderHandle) -> OniStatus;

    /// Stops recording. You can resume recording via oniRecorderStart.
    /// @param[in] recorder The handle to the recorder.
    /// @retval ONI_STATUS_OK Upon successful completion.
    /// @retval ONI_STATUS_ERROR Upon any kind of failure.
    #[link_name = "\u{1}_oniRecorderStop"]
    pub fn oniRecorderStop(recorder: OniRecorderHandle);

    /// Stops recording if needed, and destroys a recorder.
    /// @param  [in,out]    recorder    The handle to the recorder, the handle will be
    /// invalidated (nullified) when the function returns.
    /// @retval ONI_STATUS_OK Upon successful completion.
    /// @retval ONI_STATUS_ERROR Upon any kind of failure.
    #[link_name = "\u{1}_oniRecorderDestroy"]
    pub fn oniRecorderDestroy(pRecorder: *mut OniRecorderHandle) -> OniStatus;

    #[link_name = "\u{1}_oniCoordinateConverterDepthToWorld"]
    pub fn oniCoordinateConverterDepthToWorld(depthStream: OniStreamHandle,
                                              depthX: f32,
                                              depthY: f32,
                                              depthZ: f32,
                                              pWorldX: *mut f32,
                                              pWorldY: *mut f32,
                                              pWorldZ: *mut f32)
                                              -> OniStatus;

    #[link_name = "\u{1}_oniCoordinateConverterWorldToDepth"]
    pub fn oniCoordinateConverterWorldToDepth(depthStream: OniStreamHandle,
                                              worldX: f32,
                                              worldY: f32,
                                              worldZ: f32,
                                              pDepthX: *mut f32,
                                              pDepthY: *mut f32,
                                              pDepthZ: *mut f32)
                                              -> OniStatus;

    #[link_name = "\u{1}_oniCoordinateConverterDepthToColor"]
    pub fn oniCoordinateConverterDepthToColor(depthStream: OniStreamHandle,
                                              colorStream: OniStreamHandle,
                                              depthX: c_int,
                                              depthY: c_int,
                                              depthZ: OniDepthPixel,
                                              pColorX: *mut c_int,
                                              pColorY: *mut c_int)
                                              -> OniStatus;

    /// Change the log output folder
    ///
    /// @param const char * strOutputFolder [in]    path to the desirebale folder
    ///
    /// @retval ONI_STATUS_OK Upon successful completion.
    /// @retval ONI_STATUS_ERROR Upon any kind of failure.
    #[link_name = "\u{1}_oniSetLogOutputFolder"]
    pub fn oniSetLogOutputFolder(strOutputFolder: *const c_char) -> OniStatus;

    /// Get the current log file name
    ///
    /// @param  char * strFileName  [out]   hold the returned file name
    /// @param  int nBufferSize [in]    size of strFileName
    ///
    /// @retval ONI_STATUS_OK Upon successful completion.
    /// @retval ONI_STATUS_ERROR Upon any kind of failure.
    #[link_name = "\u{1}_oniGetLogFileName"]
    pub fn oniGetLogFileName(strFileName: *mut c_char,
                             nBufferSize: c_int)
                             -> OniStatus;

    /// Set the Minimum severity for log produce
    ///
    /// @param  const char * strMask    [in]    Name of the logger
    ///
    /// @retval ONI_STATUS_OK Upon successful completion.
    /// @retval ONI_STATUS_ERROR Upon any kind of failure.
    #[link_name = "\u{1}_oniSetLogMinSeverity"]
    pub fn oniSetLogMinSeverity(nMinSeverity: c_int) -> OniStatus;

    /// Configures if log entries will be printed to console.
    ///
    /// @param  OniBool bConsoleOutput  [in]    TRUE to print log entries to console, FALSE otherwise.
    ///
    /// @retval ONI_STATUS_OK Upon successful completion.
    /// @retval ONI_STATUS_ERROR Upon any kind of failure.
    #[link_name = "\u{1}_oniSetLogConsoleOutput"]
    pub fn oniSetLogConsoleOutput(bConsoleOutput: OniBool) -> OniStatus;

    /// Configures if log entries will be printed to a log file.
    ///
    /// @param  OniBool bFileOutput [in]    TRUE to print log entries to the file, FALSE otherwise.
    ///
    /// @retval ONI_STATUS_OK Upon successful completion.
    /// @retval ONI_STATUS_ERROR Upon any kind of failure.
    #[link_name = "\u{1}_oniSetLogFileOutput"]
    pub fn oniSetLogFileOutput(bFileOutput: OniBool) -> OniStatus;
}
