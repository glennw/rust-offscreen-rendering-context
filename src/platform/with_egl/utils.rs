use std::mem;
use euclid::Size2D;
use super::NativeGLContext;

use egl;
use egl::types::{EGLNativeDisplayType, EGLDisplay, EGLConfig, EGLSurface, EGLint};

fn create_pbuffer_surface(display: EGLDisplay, config: EGLConfig, size: Size2D<i32>) -> Result<EGLSurface, &'static str> {
    let mut attrs = [
        egl::WIDTH as EGLint, size.width as EGLint,
        egl::HEIGHT as EGLint, size.height as EGLint,
        egl::NONE as EGLint, 0, 0, 0, // see mod.rs
    ];

    let surface = unsafe { egl::CreatePbufferSurface(display, config, &mut *attrs.as_mut_ptr()) };

    if surface == (egl::NO_SURFACE as EGLSurface) {
        return Err("egl::CreatePBufferSurface");
    }

    Ok(surface)
}

pub fn create_pixel_buffer_backed_offscreen_context(size: Size2D<i32>) -> Result<NativeGLContext, &'static str> {
    let attributes = [
        egl::SURFACE_TYPE as EGLint, egl::PBUFFER_BIT as EGLint,
        egl::RENDERABLE_TYPE as EGLint, egl::OPENGL_ES2_BIT as EGLint,
        egl::RED_SIZE as EGLint, 8,
        egl::GREEN_SIZE as EGLint, 8,
        egl::BLUE_SIZE as EGLint, 8,
        egl::ALPHA_SIZE as EGLint, 0,
        egl::NONE as EGLint, 0, 0, 0, // see mod.rs
    ];

    // TODO: Check if we should use `egl::GetCurrentDisplay` instead
    let display = unsafe { egl::GetDisplay(egl::DEFAULT_DISPLAY as EGLNativeDisplayType) };

    if display == (egl::NO_DISPLAY as EGLDisplay) {
        return Err("egl::GetDisplay");
    }


    let mut config : EGLConfig = unsafe { mem::uninitialized() };
    let mut found_configs : EGLint = 0;

    unsafe {
        if egl::ChooseConfig(display, attributes.as_ptr(), &mut config, 1, &mut found_configs) == 0 {
            return Err("egl::ChooseConfig");
        }
    }

    if found_configs == 0 {
        return Err("No EGL config for pBuffer");
    }

    let surface = try!(create_pbuffer_surface(display, config, size));

    NativeGLContext::new(None, display, surface, config)
}
