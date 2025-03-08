pub mod commands;
pub mod events;

use crate::video_player::commands::StreamInfo;
use crate::video_player::events::{emit_player_event, PlayerEvent};
use anyhow::Result;
use gstreamer as gst;
use gstreamer::bus::BusWatchGuard;
use gstreamer::glib;
use gstreamer::prelude::*;
use gstreamer::Element;
use std::fmt;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Runtime};

pub struct VideoPlayer<R: Runtime> {
    pipeline: Arc<Mutex<Option<gst::Pipeline>>>,
    state: Arc<Mutex<PlayerState>>,
    _bus_watch: Arc<Mutex<Option<BusWatchGuard>>>,
    volume: Arc<Mutex<f64>>,
    playbin: Arc<Mutex<Option<Element>>>,
    app_handle: AppHandle<R>,
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerState {
    Stopped,
    Playing,
    Paused,
    Error,
}

impl fmt::Display for PlayerState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlayerState::Stopped => write!(f, "stopped"),
            PlayerState::Playing => write!(f, "playing"),
            PlayerState::Paused => write!(f, "paused"),
            PlayerState::Error => write!(f, "error"),
        }
    }
}

impl<R: Runtime> VideoPlayer<R> {
    pub fn new(app_handle: AppHandle<R>) -> Result<Self> {
        // Initialize GStreamer
        gst::init()?;

        Ok(Self {
            pipeline: Arc::new(Mutex::new(None)),
            state: Arc::new(Mutex::new(PlayerState::Stopped)),
            _bus_watch: Arc::new(Mutex::new(None)),
            volume: Arc::new(Mutex::new(1.0)),
            playbin: Arc::new(Mutex::new(None)),
            app_handle,
        })
    }

    pub fn play(&self, uri: &str) -> Result<()> {
        // Create a new pipeline for playback
        let pipeline = gst::Pipeline::new();

        // Create elements
        let playbin = gst::ElementFactory::make("playbin")
            .name("playbin")
            .build()?;

        // Set the URI property
        playbin.set_property("uri", uri);

        // Set initial volume
        let current_volume = *self.volume.lock().unwrap();
        playbin.set_property("volume", current_volume);

        // Add elements to pipeline
        pipeline.add(&playbin)?;

        // Set up bus message handling
        let pipeline_weak = pipeline.downgrade();
        let app_handle = self.app_handle.clone();
        let bus = pipeline.bus().unwrap();
        let watch = bus.add_watch(move |_, msg| {
            if let Some(pipeline) = pipeline_weak.upgrade() {
                Self::handle_bus_message(&app_handle, &pipeline, msg);
            }
            glib::ControlFlow::Continue
        })?;

        // Store watch guard, pipeline, and playbin
        *self._bus_watch.lock().unwrap() = Some(watch);
        *self.pipeline.lock().unwrap() = Some(pipeline.clone());
        *self.playbin.lock().unwrap() = Some(playbin);

        pipeline.set_state(gst::State::Playing)?;
        *self.state.lock().unwrap() = PlayerState::Playing;

        // Emit state change event
        emit_player_event(
            &self.app_handle,
            PlayerEvent::StateChanged {
                state: "playing".to_string(),
            },
        );

        Ok(())
    }

    pub fn pause(&self) -> Result<()> {
        if let Some(pipeline) = &*self.pipeline.lock().unwrap() {
            pipeline.set_state(gst::State::Paused)?;
            *self.state.lock().unwrap() = PlayerState::Paused;
            emit_player_event(
                &self.app_handle,
                PlayerEvent::StateChanged {
                    state: "paused".to_string(),
                },
            );
        }
        Ok(())
    }

    pub fn resume(&self) -> Result<()> {
        if let Some(pipeline) = &*self.pipeline.lock().unwrap() {
            pipeline.set_state(gst::State::Playing)?;
            *self.state.lock().unwrap() = PlayerState::Playing;
            emit_player_event(
                &self.app_handle,
                PlayerEvent::StateChanged {
                    state: "playing".to_string(),
                },
            );
        }
        Ok(())
    }

    pub fn stop(&self) -> Result<()> {
        if let Some(pipeline) = &*self.pipeline.lock().unwrap() {
            pipeline.set_state(gst::State::Null)?;
            *self.state.lock().unwrap() = PlayerState::Stopped;
            emit_player_event(
                &self.app_handle,
                PlayerEvent::StateChanged {
                    state: "stopped".to_string(),
                },
            );
        }
        Ok(())
    }

    pub fn seek(&self, position: f64) -> Result<()> {
        if let Some(pipeline) = &*self.pipeline.lock().unwrap() {
            let position_nanos = (position * 1_000_000_000.0) as i64;
            pipeline.seek_simple(
                gst::SeekFlags::FLUSH | gst::SeekFlags::KEY_UNIT,
                gst::ClockTime::from_nseconds(position_nanos as u64),
            )?;
        }
        Ok(())
    }

    pub fn set_volume(&self, volume: f64) -> Result<()> {
        let volume = volume.max(0.0).min(1.0);
        *self.volume.lock().unwrap() = volume;

        if let Some(playbin) = &*self.playbin.lock().unwrap() {
            playbin.set_property("volume", volume);
        }
        Ok(())
    }

    pub fn get_stream_info(&self) -> Result<StreamInfo> {
        let mut info = StreamInfo {
            duration: None,
            position: 0.0,
            video_codec: None,
            audio_codec: None,
            resolution: None,
            bitrate: None,
        };

        if let Some(pipeline) = &*self.pipeline.lock().unwrap() {
            // Get duration and position
            if let Some(duration) = pipeline.query_duration::<gst::ClockTime>() {
                let duration_nsec = duration.nseconds();
                info.duration = Some(duration_nsec as f64 / 1_000_000_000.0);
            }

            if let Some(position) = pipeline.query_position::<gst::ClockTime>() {
                let position_nsec = position.nseconds();
                info.position = position_nsec as f64 / 1_000_000_000.0;
            }

            // Get stream information from playbin
            if let Some(playbin) = &*self.playbin.lock().unwrap() {
                // Get video stream info
                let n_video: i32 = playbin.property("n-video");
                if n_video > 0 {
                    if let Some(video_pad) = playbin.static_pad("video_sink") {
                        if let Some(caps) = video_pad.current_caps() {
                            if let Some(s) = caps.structure(0) {
                                // Get video codec
                                if let Ok(codec_name) = s.get::<String>("codec_name") {
                                    info.video_codec = Some(codec_name);
                                }

                                // Get resolution
                                if let (Ok(width), Ok(height)) =
                                    (s.get::<i32>("width"), s.get::<i32>("height"))
                                {
                                    info.resolution = Some(format!("{}x{}", width, height));
                                }
                            }
                        }
                    }
                }

                // Get audio codec information
                let n_audio: i32 = playbin.property("n-audio");
                if n_audio > 0 {
                    if let Some(audio_pad) = playbin.static_pad("audio_sink") {
                        if let Some(caps) = audio_pad.current_caps() {
                            if let Some(s) = caps.structure(0) {
                                if let Ok(codec_name) = s.get::<String>("codec_name") {
                                    info.audio_codec = Some(codec_name);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(info)
    }

    pub fn get_state(&self) -> PlayerState {
        *self.state.lock().unwrap()
    }

    fn handle_bus_message(
        app_handle: &AppHandle<R>,
        _pipeline: &gst::Pipeline,
        msg: &gst::Message,
    ) {
        match msg.view() {
            gst::MessageView::Error(err) => {
                eprintln!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );

                emit_player_event(
                    app_handle,
                    PlayerEvent::Error {
                        code: -1, // Generic error code since GStreamer errors are strings
                        message: err.error().to_string(),
                    },
                );
            }
            gst::MessageView::Eos(_) => {
                println!("End of stream");
                emit_player_event(app_handle, PlayerEvent::EndOfStream);
            }
            gst::MessageView::StateChanged(state) => {
                println!(
                    "State changed from {:?} to {:?}",
                    state.old(),
                    state.current()
                );
            }
            gst::MessageView::Buffering(buffer) => {
                emit_player_event(
                    app_handle,
                    PlayerEvent::BufferingProgress {
                        percent: buffer.percent(),
                    },
                );
            }
            gst::MessageView::StreamStatus(status) => {
                if let Some(element) = status.src() {
                    if element.is::<gst::Element>() {
                        if let Some(video_pad) = element
                            .downcast_ref::<gst::Element>()
                            .and_then(|e| e.static_pad("video_sink"))
                        {
                            if let Some(caps) = video_pad.current_caps() {
                                if let Some(s) = caps.structure(0) {
                                    if let (Ok(width), Ok(height)) =
                                        (s.get::<i32>("width"), s.get::<i32>("height"))
                                    {
                                        let framerate = s
                                            .get::<gst::Fraction>("framerate")
                                            .ok()
                                            .map(|f| f.numer() as f64 / f.denom() as f64);
                                        let bitrate = None; // Could be added if available in the stream

                                        emit_player_event(
                                            app_handle,
                                            PlayerEvent::QualityChanged {
                                                width,
                                                height,
                                                framerate,
                                                bitrate,
                                            },
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }
}

impl<R: Runtime> Drop for VideoPlayer<R> {
    fn drop(&mut self) {
        if let Some(pipeline) = &*self.pipeline.lock().unwrap() {
            let _ = pipeline.set_state(gst::State::Null);
        }
    }
}
