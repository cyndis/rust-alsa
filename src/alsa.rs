#![crate_type = "lib"]

extern crate libc;

use std::ptr;

#[allow(dead_code, unused_attributes, bad_style)]
mod ffi;

#[link(name = "asound")]
extern { }

macro_rules! alsa_ok {
    ($e:expr) => (
        {
            let err = $e;
            if err < 0 {
                return Err(err as isize)
            }
            err
        }
    )
}

pub struct PCM<State> {
    i: *mut ffi::snd_pcm_t,
    data: State
}

#[allow(missing_copy_implementations)]
pub struct Open { #[allow(dead_code)] no_constr: () }

#[allow(missing_copy_implementations)]
pub struct Prepared {
    channels: usize,
    sample_fmt: Format
}

#[derive(Copy, PartialEq, Eq, Show)]
pub enum Stream {
    Playback,
    Capture
}

impl Stream {
    fn to_ffi(self) -> ffi::snd_pcm_stream_t {
        match self {
            Stream::Playback => ffi::SND_PCM_STREAM_PLAYBACK,
            Stream::Capture  => ffi::SND_PCM_STREAM_CAPTURE
        }
    }
}

#[derive(Copy, PartialEq, Eq, Show)]
pub enum Mode {
    Blocking,
    Nonblocking,
    Asynchronous
}

impl Mode {
    fn to_ffi(self) -> i32 {
        match self {
            Mode::Blocking => 0,
            Mode::Nonblocking => ffi::SND_PCM_NONBLOCK,
            Mode::Asynchronous => ffi::SND_PCM_ASYNC
        }
    }
}

#[derive(Copy, PartialEq, Eq, Show)]
pub enum Access {
    Interleaved,
    Noninterleaved
}

impl Access {
    fn to_ffi(self) -> ffi::snd_pcm_access_t {
        match self {
            Access::Interleaved => ffi::SND_PCM_ACCESS_RW_INTERLEAVED,
            Access::Noninterleaved => ffi::SND_PCM_ACCESS_RW_NONINTERLEAVED
        }
    }
}

#[derive(Copy, PartialEq, Eq, Show)]
pub enum Format {
    Unsigned8,
    Signed16,
    FloatLE
}

impl Format {
    fn to_ffi(self) -> ffi::snd_pcm_format_t {
        match self {
            Format::Unsigned8 => ffi::SND_PCM_FORMAT_U8,
            Format::Signed16 => ffi::SND_PCM_FORMAT_S16,
            Format::FloatLE => ffi::SND_PCM_FORMAT_FLOAT_LE
        }
    }

    fn size(self) -> usize {
        use std::mem::size_of;
        match self {
            Format::Unsigned8 => 1,
            Format::Signed16 => 2,
            Format::FloatLE => size_of::<libc::c_float>()
        }
    }
}

impl PCM<Open> {
    pub fn open(name: &str, stream: Stream, mode: Mode) -> Result<PCM<Open>, isize> {
        let mut pcm = PCM {
            i: ptr::null_mut(),
            data: Open { no_constr: () }
        };

        unsafe {
            let name = std::ffi::CString::new(name).unwrap();
            alsa_ok!(
                ffi::snd_pcm_open(&mut pcm.i, name.as_ptr(), stream.to_ffi(), mode.to_ffi())
            );
        }

        Ok(pcm)
    }
}

impl PCM<Open> {
    pub fn set_parameters(self, format: Format, access: Access, channels: usize, rate: usize)
        -> Result<PCM<Prepared>, (PCM<Open>, isize)>
    {
        unsafe {
            let err = ffi::snd_pcm_set_params(self.i, format.to_ffi(), access.to_ffi(),
                                              channels as u32, rate as u32, 1i32, 500000u32);
            if err < 0 {
                return Err((self, err as isize))
            }
        }

        Ok(
            PCM {
                i: self.i,
                data: Prepared {
                    channels: channels,
                    sample_fmt: format
                }
            }
        )
    }

}

impl PCM<Prepared> {
    pub fn write_interleaved<T: Copy>(&mut self, buffer: &[T]) -> Result<usize, isize> {
        let channels = self.data.channels;

        assert_eq!(buffer.len() % channels, 0);
        assert_eq!(::std::mem::size_of::<T>(), self.data.sample_fmt.size());

        let n_written = unsafe {
            alsa_ok!(ffi::snd_pcm_writei(self.i, buffer.as_ptr() as *const libc::c_void,
                                         buffer.len() as u64 / channels as u64))
        };

        Ok(n_written as usize)
    }
}
