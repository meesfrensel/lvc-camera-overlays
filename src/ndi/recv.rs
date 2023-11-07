use super::*;
use std::{ffi::CString, fmt::Display, mem, sync::Mutex};

/// Current performance levels of the receiving.
///
/// This allows you determine whether frames have been dropped.
#[derive(Debug, Clone, Copy, Hash, Default)]
pub struct RecvPerformance {
    /// number of video frames
    pub video_frames: i64,
    /// number of audio frames
    pub audio_frames: i64,
    /// number of metadata frames
    pub metadata_frames: i64,
}

impl RecvPerformance {
    fn from_binding(perf: NDIlib_recv_performance_t) -> Self {
        Self {
            video_frames: perf.video_frames,
            audio_frames: perf.audio_frames,
            metadata_frames: perf.metadata_frames,
        }
    }
}

impl Display for RecvPerformance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Video frames: {}", self.video_frames)?;
        writeln!(f, "Audio frames: {}", self.audio_frames)?;
        writeln!(f, "Metadata frames: {}", self.metadata_frames)
    }
}

/// Determines what color formats you are passed when a frame is received.
///
/// In general, there are two color formats used in any scenario: one that
/// exists when the source has an alpha channel, and another when it does not.
/// See [`FourCCVideoType`] for details on individual FourCC types.
///

///
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum RecvColorFormat {
    /// BGRX or BGRA
    BGRX_BGRA = NDIlib_recv_color_format_e_NDIlib_recv_color_format_BGRX_BGRA as _,
    /// UYVY or BGRA
    UYVY_BGRA = NDIlib_recv_color_format_e_NDIlib_recv_color_format_UYVY_BGRA as _,
    /// RGBX or RGBA
    RGBX_RGBA = NDIlib_recv_color_format_e_NDIlib_recv_color_format_RGBX_RGBA as _,
    /// UYVY or RGBA
    UYVY_RGBA = NDIlib_recv_color_format_e_NDIlib_recv_color_format_UYVY_RGBA as _,
    /// Normally UYVY, see [`RecvColorFormat`]
    ///
    /// If you specify the color option [`RecvColorFormat::Fastest`], the SDK will provide buffers in the format
    /// that it processes internally without performing any conversions before they are passed to you.
    /// The `allow_video_fields` option is assumed to be true in this mode.
    Fastest = NDIlib_recv_color_format_e_NDIlib_recv_color_format_fastest as _,
    /// Varies, see [`RecvColorFormat`]
    ///
    /// If you specify the color option [`RecvColorFormat::Best`], the SDK will provide you buffers in the format
    /// closest to the native precision of the video codec being used.
    /// In many cases this is both high-performance and high-quality and results in the best quality.
    /// Like [`RecvColorFormat::Fastest`] this format will always deliver individual fields,
    /// implicitly assuming the `allow_video_fields option` as true.
    Best = NDIlib_recv_color_format_e_NDIlib_recv_color_format_best as _,
}

/// Specify whether this connection is in high or low bandwidth mode.
///
/// For most uses you should specify [`RecvBandwidth::Highest`],
/// which will result in the same stream that is being sent from the up-stream source to you.
/// You may specify [`RecvBandwidth::Lowest`], which will provide you with a
/// medium quality stream that takes significantly reduced bandwidth.

#[derive(Debug, Clone, Copy)]
pub enum RecvBandwidth {
    /// Receive metadata only.
    MetadataOnly = NDIlib_recv_bandwidth_e_NDIlib_recv_bandwidth_metadata_only as _,
    /// Receive metadata + audio.
    AudioOnly = NDIlib_recv_bandwidth_e_NDIlib_recv_bandwidth_audio_only as _,
    /// Receive metadata, audio, video at a lower bandwidth and resolution.
    Lowest = NDIlib_recv_bandwidth_e_NDIlib_recv_bandwidth_lowest as _,
    /// Receive metadata, audio, video at full resolution.
    Highest = NDIlib_recv_bandwidth_e_NDIlib_recv_bandwidth_highest as _,
}

/// Builder struct for [`Recv`]
#[derive(Debug, Clone)]
pub struct RecvBuilder {
    source_to_connect_to: Option<Source>,
    color_format: Option<RecvColorFormat>,
    bandwidth: Option<RecvBandwidth>,
    allow_video_fields: Option<bool>,
    ndi_recv_name: Option<String>,
}

impl RecvBuilder {
    /// Create a new instance of the builder
    pub fn new() -> Self {
        Self {
            source_to_connect_to: None,
            color_format: None,
            bandwidth: None,
            allow_video_fields: None,
            ndi_recv_name: None,
        }
    }

    /// Choose the [`Source`] to connect to
    pub fn source_to_connect_to(mut self, source: Source) -> Self {
        self.source_to_connect_to = Some(source);
        self
    }

    /// Choose the color format of preference
    pub fn color_format(mut self, color_format: RecvColorFormat) -> Self {
        self.color_format = Some(color_format);
        self
    }

    /// Select a bandwidth mode
    pub fn bandwidth(mut self, bandwidth: RecvBandwidth) -> Self {
        self.bandwidth = Some(bandwidth);
        self
    }

    /// If your application does not like receiving fielded video data you can specify
    /// `false` to this value, and all video received will be de-interlaced before it is
    /// passed to you.
    pub fn allow_video_fields(mut self, allow_video_fields: bool) -> Self {
        self.allow_video_fields = Some(allow_video_fields);
        self
    }

    /// The name of the NDI receiver to create
    ///
    /// Give your receiver a meaningful, descriptive, and unique name. This will
    /// be the name of the NDI receiver on the network. For instance, if your network
    /// machine name is called “MyMachine” and you specify this parameter as “Video
    /// Viewer”, then the NDI receiver on the network would be “MyMachine (Video Viewer)”.
    pub fn ndi_recv_name(mut self, ndi_recv_name: String) -> Self {
        self.ndi_recv_name = Some(ndi_recv_name);
        self
    }

    /// Build the [`Recv`]
    pub fn build(self) -> Result<Recv, RecvCreateError> {
        // From default C++ constructor in Processing.NDI.Recv.h
        let mut settings: NDIlib_recv_create_v3_t = NDIlib_recv_create_v3_t {
            source_to_connect_to: Source::new().p_instance,
            color_format: RecvColorFormat::UYVY_BGRA as _,
            bandwidth: RecvBandwidth::Highest as _,
            allow_video_fields: true,
            p_ndi_recv_name: null(),
        };

        if let Some(src) = self.source_to_connect_to {
            settings.source_to_connect_to = src.p_instance;
        }
        if let Some(color_format) = self.color_format {
            settings.color_format = color_format as _;
        }
        if let Some(bandwidth) = self.bandwidth {
            settings.bandwidth = bandwidth as _;
        }
        if let Some(allow_video_fields) = self.allow_video_fields {
            settings.allow_video_fields = allow_video_fields;
        }
        if let Some(ndi_recv_name) = self.ndi_recv_name {
            // String shouldn't contain a 0-byte
            let cstr = CString::new(ndi_recv_name).unwrap();

            settings.p_ndi_recv_name = cstr.into_raw();
        }

        Recv::with_settings(settings)
    }
}

/// Check the current queue size
#[derive(Debug, Clone, Copy)]
pub struct RecvQueueSize {
    /// Number of video frames in queue
    pub video_frames: u32,
    /// Number of audio frames in queue
    pub audio_frames: u32,
    /// Number of metadata frames in queue
    pub metadata_frames: u32,
}

impl RecvQueueSize {
    /// Create new queue size
    pub fn new() -> Self {
        Self {
            video_frames: 0,
            audio_frames: 0,
            metadata_frames: 0,
        }
    }

    fn from_binding(queue: NDIlib_recv_queue_t) -> Self {
        Self {
            video_frames: queue.video_frames as _,
            audio_frames: queue.audio_frames as _,
            metadata_frames: queue.metadata_frames as _,
        }
    }
}

/// The NDI receiver struct
pub struct Recv {
    /// whether the Recv is currently connected
    pub connected: bool,
    p_instance: Arc<OnDrop<NDIlib_recv_instance_t>>,
    guard: Mutex<()>,
}

unsafe impl core::marker::Send for Recv {}
unsafe impl core::marker::Sync for Recv {}

impl Recv {
    fn with_settings(settings: NDIlib_recv_create_v3_t) -> Result<Self, RecvCreateError> {
        let p_instance = unsafe { NDIlib_recv_create_v3(&settings) };
        if p_instance.is_null() {
            return Err(RecvCreateError);
        }

        let guard = Mutex::new(());
        let mut this = Self {
            p_instance: Arc::new(OnDrop::new(p_instance, |s| unsafe {
                NDIlib_recv_destroy(s)
            })),
            connected: false,
            guard,
        };

        this.connected = this.get_no_connections() > 0;
        Ok(this)
    }

    /// Create new receiver which isn't connected to any sources
    ///
    /// It is recommended that you use [`RecvBuilder`] instead if possible
    pub fn new() -> Result<Self, RecvCreateError> {
        let p_instance = unsafe { NDIlib_recv_create_v3(null()) };

        if p_instance.is_null() {
            return Err(RecvCreateError);
        }

        let guard = Mutex::new(());
        Ok(Self {
            p_instance: Arc::new(OnDrop::new(p_instance, |s| unsafe {
                NDIlib_recv_destroy(s)
            })),
            connected: false,
            guard,
        })
    }

    /// Connect to a source
    pub fn connect(&mut self, source: &Source) {
        let instance: *const NDIlib_source_t = &source.p_instance;
        unsafe { NDIlib_recv_connect(**self.p_instance, instance) };
    }

    /// Disconnect from all sources
    pub fn disconnect(&mut self) {
        unsafe {
            NDIlib_recv_connect(**self.p_instance, null());
        }
    }

    /// Receive video, audio and metadata frames.
    ///
    /// This call can be called simultaneously on separate threads,
    /// so it is entirely possible to receive audio, video, metadata all on separate threads.
    /// This function will return [`FrameType::None`] if no data is received within the specified timeout
    /// and [`FrameType::ErrorFrame`] if the connection is lost.
    /// Buffers captured with this must be freed with the appropriate free function.
    pub fn capture_all(
        &self,
        video_data: &mut Option<VideoData>,
        audio_data: &mut Option<AudioData>,
        meta_data: &mut Option<MetaData>,
        timeout_ms: u32,
    ) -> FrameType {
        let mut video = if let Some(x) = video_data {
            mem::MaybeUninit::new(x.p_instance)
        } else {
            mem::MaybeUninit::zeroed()
        };

        let mut audio = if let Some(x) = audio_data {
            mem::MaybeUninit::new(x.p_instance)
        } else {
            mem::MaybeUninit::zeroed()
        };

        let mut metadata = if let Some(x) = meta_data {
            mem::MaybeUninit::new(x.p_instance)
        } else {
            mem::MaybeUninit::zeroed()
        };

        let response = unsafe {
            NDIlib_recv_capture_v3(
                **self.p_instance,
                video.as_mut_ptr(),
                audio.as_mut_ptr(),
                metadata.as_mut_ptr(),
                timeout_ms,
            )
        };

        if !video.as_ptr().is_null() {
            *video_data = Some(VideoData::from_binding_recv(
                self.p_instance.clone(),
                unsafe { video.assume_init() },
            ));
        }

        if !audio.as_ptr().is_null() {
            *audio_data = Some(AudioData::from_binding_recv(
                self.p_instance.clone(),
                unsafe { audio.assume_init() },
            ));
        }

        if !metadata.as_ptr().is_null() {
            *meta_data = Some(MetaData::from_binding_recv(
                self.p_instance.clone(),
                unsafe { metadata.assume_init() },
            ));
        }

        FrameType::try_from(response).unwrap()
    }

    /// Receive video frame
    pub fn capture_video(&self, video_data: &mut Option<VideoData>, timeout_ms: u32) -> FrameType {
        unsafe {
            let mut video = if let Some(x) = video_data {
                mem::MaybeUninit::new(x.p_instance)
            } else {
                mem::MaybeUninit::zeroed()
            };

            let response = NDIlib_recv_capture_v3(
                **self.p_instance,
                video.as_mut_ptr(),
                null_mut(),
                null_mut(),
                timeout_ms,
            );

            if !video.as_ptr().is_null() {
                *video_data = Some(VideoData::from_binding_recv(
                    self.p_instance.clone(),
                    video.assume_init(),
                ));
            }

            FrameType::try_from(response).unwrap()
        }
    }

    /// Receive audio frame
    pub fn capture_audio(&self, audio_data: &mut Option<AudioData>, timeout_ms: u32) -> FrameType {
        unsafe {
            let mut audio = if let Some(x) = audio_data {
                mem::MaybeUninit::new(x.p_instance)
            } else {
                mem::MaybeUninit::zeroed()
            };
            let response = NDIlib_recv_capture_v3(
                **self.p_instance,
                null_mut(),
                audio.as_mut_ptr(),
                null_mut(),
                timeout_ms,
            );

            if !audio.as_ptr().is_null() {
                *audio_data = Some(AudioData::from_binding_recv(
                    self.p_instance.clone(),
                    audio.assume_init(),
                ));
            }
            FrameType::try_from(response).unwrap()
        }
    }

    /// Receive metadata frame
    pub fn capture_metadata(&self, meta_data: &mut Option<MetaData>, timeout_ms: u32) -> FrameType {
        unsafe {
            let mut metadata = if let Some(x) = meta_data {
                mem::MaybeUninit::new(x.p_instance)
            } else {
                mem::MaybeUninit::zeroed()
            };
            let response = NDIlib_recv_capture_v3(
                **self.p_instance,
                null_mut(),
                null_mut(),
                metadata.as_mut_ptr(),
                timeout_ms,
            );

            if !metadata.as_ptr().is_null() {
                *meta_data = Some(MetaData::from_binding_recv(
                    Arc::clone(&self.p_instance),
                    metadata.assume_init(),
                ));
            }
            FrameType::try_from(response).unwrap()
        }
    }

    /// Get the performance metrics (total, dropped)
    pub fn get_performance(&self) -> (RecvPerformance, RecvPerformance) {
        let _lock = self.guard.lock().unwrap();
        let mut p_total: mem::MaybeUninit<NDIlib_recv_performance_t> = mem::MaybeUninit::uninit();
        let mut p_dropped: mem::MaybeUninit<NDIlib_recv_performance_t> = mem::MaybeUninit::uninit();
        unsafe {
            NDIlib_recv_get_performance(
                **self.p_instance,
                p_total.as_mut_ptr(),
                p_dropped.as_mut_ptr(),
            )
        };

        let total_perf = RecvPerformance::from_binding(unsafe { p_total.assume_init() });
        let dropped_perf = RecvPerformance::from_binding(unsafe { p_dropped.assume_init() });

        (total_perf, dropped_perf)
    }

    /// Get the current queue sizes
    pub fn get_queue(&self) -> RecvQueueSize {
        let _lock = self.guard.lock().unwrap();
        let mut p_total: mem::MaybeUninit<NDIlib_recv_queue_t> = mem::MaybeUninit::uninit();
        unsafe {
            NDIlib_recv_get_queue(**self.p_instance, p_total.as_mut_ptr());
            let queue = RecvQueueSize::from_binding(p_total.assume_init());

            queue
        }
    }

    /// Get the current number of sources connected to
    pub fn get_no_connections(&self) -> u32 {
        let _lock = self.guard.lock().unwrap();
        unsafe { NDIlib_recv_get_no_connections(**self.p_instance) as _ }
    }

    /// Set tally info for sender
    pub fn set_tally(&mut self, tally: Tally) {
        let _lock = self.guard.lock().unwrap();
        unsafe {
            NDIlib_recv_set_tally(**self.p_instance, &tally.into());
        }
    }

    /// Add a connection metadata string to the list of what is sent on each new connection.
    ///
    /// If someone is already connected then this string will be sent to them immediately.
    /// Connection based metadata is data that is sent automatically each time a new connection is received.
    /// To reset them you need to clear them all and set them up again using [`Recv::recv_clear_connection_metadata()`]
    pub fn add_connection_metadata(&self, metadata: &MetaData) {
        let _lock = self.guard.lock().unwrap();
        unsafe {
            NDIlib_recv_add_connection_metadata(**self.p_instance, &metadata.p_instance);
        }
    }

    /// Send metadata to sender
    ///
    /// This returns `false` if we are not currently connected to anything.
    pub fn send_metadata(&self, metadata: &MetaData) -> bool {
        let _lock = self.guard.lock().unwrap();
        unsafe { NDIlib_recv_send_metadata(**self.p_instance, &metadata.p_instance) }
    }

    /// Clear all connection metadata
    pub fn recv_clear_connection_metadata(&self) {
        let _lock = self.guard.lock().unwrap();
        unsafe {
            NDIlib_recv_clear_connection_metadata(**self.p_instance);
        }
    }
}
