#![crate_type = "lib"]
#![feature(globs, macro_rules, unsafe_destructor)]

extern crate libc;

use std::ptr;

#[allow(dead_code, unused_attribute, non_camel_case_types)]
mod ffi;

#[cfg(test)]
mod test;

#[link(name = "asound")]
extern { }

macro_rules! alsa_ok(
    ($e:expr) => (
        {
            let err = $e;
            if err < 0 {
                return Err(err as int)
            }
            err
        }
    )
)

pub struct PCM {
    i: *mut ffi::snd_pcm_t,
    channels: Option<uint>
}

pub enum Stream {
    Playback,
    Capture
}

impl Stream {
    fn to_ffi(self) -> ffi::snd_pcm_stream_t {
        match self {
            Playback => ffi::SND_PCM_STREAM_PLAYBACK,
            Capture  => ffi::SND_PCM_STREAM_CAPTURE
        }
    }
}

pub enum Mode {
    Blocking,
    Nonblocking,
    Asynchronous
}

impl Mode {
    fn to_ffi(self) -> i32 {
        match self {
            Blocking => 0,
            Nonblocking => ffi::SND_PCM_NONBLOCK,
            Asynchronous => ffi::SND_PCM_ASYNC
        }
    }
}

pub enum Access {
    Interleaved,
    Noninterleaved
}

impl Access {
    fn to_ffi(self) -> ffi::snd_pcm_access_t {
        match self {
            Interleaved => ffi::SND_PCM_ACCESS_RW_INTERLEAVED,
            Noninterleaved => ffi::SND_PCM_ACCESS_RW_NONINTERLEAVED
        }
    }
}

pub enum Format {
    Unsigned8,
    Signed16,
    FloatLE
}

impl Format {
    fn to_ffi(self) -> ffi::snd_pcm_format_t {
        match self {
            Unsigned8 => ffi::SND_PCM_FORMAT_U8,
            Signed16 => ffi::SND_PCM_FORMAT_S16,
            FloatLE => ffi::SND_PCM_FORMAT_FLOAT_LE
        }
    }
}

impl PCM {
    pub fn open(name: &str, stream: Stream, mode: Mode) -> Result<PCM, int> {
        let mut pcm = PCM {
            i: ptr::mut_null(),
            channels: None
        };

        unsafe {
            alsa_ok!(
                name.to_c_str().with_ref(|name| {
                    ffi::snd_pcm_open(&mut pcm.i, name, stream.to_ffi(), mode.to_ffi())
                })
            );
        }

        Ok(pcm)
    }

    pub fn prepare(&mut self) -> Result<(), int> {
        unsafe {
            alsa_ok!(ffi::snd_pcm_prepare(self.i));
        }
        Ok(())
    }

    pub fn set_parameters(&mut self, format: Format, access: Access, channels: uint, rate: uint)
        -> Result<(), int>
    {
        unsafe {
            alsa_ok!(ffi::snd_pcm_set_params(self.i, format.to_ffi(), access.to_ffi(),
                                             channels as u32, rate as u32, 1i32, 500000u32));
        }

        self.channels = Some(channels);

        Ok(())
    }

    pub fn write_interleaved<T: Copy>(&mut self, buffer: &[T]) -> Result<uint, int> {
        let channels = self.channels.expect("write_interleaved called but parameters not set");
        assert_eq!(buffer.len() % channels, 0);

        let n_written = unsafe {
            alsa_ok!(ffi::snd_pcm_writei(self.i, buffer.as_ptr() as *const libc::c_void,
                                         buffer.len() as u64 / channels as u64))
        };

        Ok(n_written as uint)
    }
}
