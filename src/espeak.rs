
use std::ptr::{null, null_mut};
use std::ffi::CString;

extern crate libc;
use libc::{c_void, c_char, c_int, c_uint, size_t};

#[repr(C)]
enum espeak_AUDIO_OUTPUT {
	AUDIO_OUTPUT_PLAYBACK,
	AUDIO_OUTPUT_RETRIEVAL,
	AUDIO_OUTPUT_SYNCHRONOUS,
	AUDIO_OUTPUT_SYNCH_PLAYBACK
}
#[repr(C)]
enum espeak_ERROR {
	EE_OK = 0,
	EE_INTERNAL_ERROR = -1,
	EE_BUFFER_FULL = 1,
	EE_NOT_FOUND = 2
}
#[derive(Debug)]
pub enum Error {
    Internal,
    BufferFull,
    NotFound,
}
impl From<espeak_ERROR> for Error {
    fn from(err: espeak_ERROR) -> Self {
        match err {
            espeak_ERROR::EE_BUFFER_FULL => Error::BufferFull,
            espeak_ERROR::EE_NOT_FOUND => Error::NotFound,
            _ => Error::Internal,
        }
    }
}

#[repr(C)]
enum espeak_POSITION_TYPE {
	POS_CHARACTER = 1,
	POS_WORD,
	POS_SENTENCE
}

#[repr(C)]
pub enum espeak_PARAMETER {
	SILENCE = 0,
	RATE,
	VOLUME,
	PITCH,
	RANGE,
	PUNCTUATION,
	CAPITALS,
	WORDGAP,
	OPTIONS,
	INTONATION,
	RESERVED1,
	RESERVED2,
	EMPHASIS,
	VOICETYPE,
	N_SPEECH_PARAM
}

#[repr(C)]
pub enum Ponctuation {
	None = 0,
	All = 1,
	Some = 2,
}

#[link(name = "espeak")]
extern "C" {
    fn espeak_Initialize(output: espeak_AUDIO_OUTPUT, buflength: c_int, path: *const c_char, options: c_int) -> c_int;
	fn espeak_Synth(text: *const c_void,
		size: size_t,
		position: c_uint,
		position_type: espeak_POSITION_TYPE,
		end_position: c_uint,
		flags: c_uint,
		unique_identifier: *mut c_uint,
user_data: *mut c_void) -> espeak_ERROR;
	fn espeak_Key(key_name: *const c_char) -> espeak_ERROR;
	fn espeak_Synchronize() -> espeak_ERROR;
	fn espeak_SetParameter(parameter: espeak_PARAMETER, value: c_int, relative: c_int) -> espeak_ERROR;
	fn espeak_Cancel() -> espeak_ERROR;
}

pub struct ESpeak {
    sample_rate: i32,
}

impl ESpeak {
    pub fn sample_rate(&self) -> i32 { self.sample_rate }

    pub fn set_ponctuation(&mut self, p: Ponctuation) -> Result<(), Error>{
        unsafe {
        match espeak_SetParameter(espeak_PARAMETER::PUNCTUATION,
                                p as i32,
                                0) {

            espeak_ERROR::EE_OK => Ok(()),
            err => Err(Error::from(err)),
        }
        }
    }

    pub fn new() -> Result<Self, ()> {
        let rate = unsafe {
            espeak_Initialize(espeak_AUDIO_OUTPUT::AUDIO_OUTPUT_PLAYBACK,
                              0,
                              null(),
                              0)
        };
        if rate == (espeak_ERROR::EE_INTERNAL_ERROR as i32) {
            Err(())
        } else {
            Ok(ESpeak { sample_rate: rate })
        }
    }

    /// Speak the given text
    ///
    /// The input argument is a sequence of bytes, espeak tries
    /// to autodected the contents as either ISO8859 or UTF8
    pub fn say(&self, text: &[u8]) -> Result<(), Error> {
        let c_str = CString::new(text).unwrap();
        let size = c_str.as_bytes().len();
        unsafe {
        match espeak_Synth(c_str.as_ptr() as *const c_void,
                            size,
                            0,
                            espeak_POSITION_TYPE::POS_CHARACTER,
                            0,
                            0, null_mut(), null_mut()) {
            espeak_ERROR::EE_OK => Ok(()),
            err => Err(Error::from(err)),
        }
        }
    }

    pub fn say_key(&self, key_name: &[u8]) -> Result<(), Error> {
        let c_str = CString::new(key_name).unwrap();
        match unsafe { espeak_Key(c_str.as_ptr()) } {
            espeak_ERROR::EE_OK => Ok(()),
            err => Err(Error::from(err)),
        }
    }

    pub fn synchronize(&self) -> Result<(), Error> {
        match unsafe { espeak_Synchronize() } {
            espeak_ERROR::EE_OK => Ok(()),
            err => Err(Error::from(err)),
        }
    }

    pub fn cancel(&self) -> Result<(), Error> {
        match unsafe { espeak_Cancel() } {
            espeak_ERROR::EE_OK => Ok(()),
            err => Err(Error::from(err)),
        }
    }
}

