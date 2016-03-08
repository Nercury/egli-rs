//! Error and Result types.

use std::result;
use std::str;

#[derive(Copy, Clone, Debug)]
pub enum EglCallError {
    GetConfigs,
    GetCurrentContext,
    GetCurrentDisplay,
    GetDisplay,
    GetCurrentSurface,
    GetConfigAttrib,
    DestroySurface,
    Initialize,
    MakeCurrent,
    DestroyContext,
    CreateWindowSurface,
    CreatePlatformWindowSurface,
    CreatePixmapSurface,
    CreatePbufferSurface,
    CreatePbufferFromClientBuffer,
    CreateContext,
    CopyBuffers,
    ChooseConfig,
    BindTexImage,
    BindAPI,
    Terminate,
    WaitClient,
    WaitGL,
    WaitNative,
    SwapInterval,
    SwapBuffers,
    SurfaceAttrib,
    ReleaseThread,
    ReleaseTexImage,
    QuerySurface,
    QueryString,
    QueryContext,
}

pub type EglCallResult<T> = result::Result<T, EglCallError>;

#[derive(Copy, Clone, Debug)]
pub enum Error {
    Egl(EglCallError),
    NonUtf8StringReceived(str::Utf8Error),
}

pub type Result<T> = result::Result<T, Error>;

impl From<EglCallError> for Error {
    fn from(other: EglCallError) -> Error {
        Error::Egl(other)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(other: str::Utf8Error) -> Error {
        Error::NonUtf8StringReceived(other)
    }
}
