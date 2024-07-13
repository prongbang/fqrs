use jni::JNIEnv;
use jni::objects::{JClass, JByteArray};
use jni::sys::{jstring, jint};
use quircs::Quirc;
use std::os::raw::{c_char, c_int};
use std::ffi::CString;

pub fn decode_qr(width: usize, height: usize, frame_data: &[u8]) -> Option<String> {
    // Assuming frame_data is in RGB or RGBA format
    let img = image::load_from_memory(frame_data).unwrap();
    let img_gray = img.into_luma8();

    let mut decoder = Quirc::default();
    let codes = decoder.identify(width, height, &img_gray);

    for code in codes {
        if let Ok(decoded) = code {
            if let Ok(data) = decoded.decode() {
                if let Ok(payload) = String::from_utf8(data.payload) {
                    return Some(payload);
                }
            }
        }
    }

    None
}

#[no_mangle]
pub extern "system" fn Java_com_inteniquetic_fqrs_decoder_QRDecoder_decodeQRCode(
    env: JNIEnv,
    _class: JClass,
    frame_data: JByteArray,
    width: jint,
    height: jint,
) -> jstring {
    let frame_data: Vec<u8> = env.convert_byte_array(frame_data).unwrap_or(Vec::new());

    let result = decode_qr(width as usize, height as usize, &frame_data);

    match result {
        Some(payload) => env.new_string(payload).unwrap().into_raw(),
        None => env.new_string("").unwrap().into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn decode_qr_code(
    frame_data: *const u8,
    width: c_int,
    height: c_int,
    data_length: c_int,
) -> *mut c_char {
    let frame_data = unsafe {
        std::slice::from_raw_parts(frame_data, data_length as usize)
    };

    let result = decode_qr(width as usize, height as usize, frame_data);

    match result {
        Some(payload) => CString::new(payload).unwrap().into_raw(),
        None => CString::new("").unwrap().into_raw(),
    }
}

#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    unsafe {
        if s.is_null() { return; }
        let _ = CString::from_raw(s);
    };
}
