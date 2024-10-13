use winapi::um::winuser::{FindWindowW, GetDC, GetDesktopWindow, GetWindowDC, GetWindowRect, IsIconic, PrintWindow, ReleaseDC, SetForegroundWindow, ShowWindow, PW_CLIENTONLY};
use winapi::um::wingdi::{BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDIBits, GetObjectW, SelectObject, BITMAP, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, SRCCOPY};
use winapi::shared::windef::{HBITMAP, HDC, HGDIOBJ, HWND, RECT};
use std::ptr::{null, from_mut};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::mem::{zeroed, size_of};
use image::{ImageBuffer, DynamicImage};

fn to_wide_str(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect::<Vec<u16>>()
}

pub fn find_window(class_name: Option<&str>, window_name: &str) -> Option<HWND> {
    let mut cls_name = None;
    if let Some(name) = class_name {
        cls_name = Some(to_wide_str(name));
    }
    let window_name = to_wide_str(window_name);

    let hwnd = unsafe {
        FindWindowW(match cls_name {
            Some(name) => name.as_ptr(),
            None => null()
        }, window_name.as_ptr())
    };
    
    if hwnd.is_null() {
        None
    }
    else {
        Some(hwnd)
    }
}

pub fn get_desktop_window() -> Option<HWND> {
    let hwnd = unsafe { GetDesktopWindow() };

    if hwnd.is_null() {
        None
    }
    else {
        Some(hwnd)
    }
}

pub fn is_iconic(hwnd: HWND) -> bool {
    unsafe { IsIconic(hwnd) != 0 }
}

pub fn set_foreground_window(hwnd: HWND) -> bool {
    unsafe { SetForegroundWindow(hwnd) != 0 }
}

pub fn show_window(hwnd: HWND, cmd: i32) -> bool {
    unsafe { ShowWindow(hwnd, cmd) != 0 }
}

pub fn get_window_rect(hwnd: HWND) -> Option<RECT> {
    let mut rect: RECT = unsafe { zeroed() };
    let success = unsafe { GetWindowRect(hwnd, &mut rect) };

    if success == 0 {
        None
    }
    else {
        Some(rect)
    }
}

pub fn get_dc(hwnd: HWND) -> Option<HDC> {
    let hdc = unsafe { GetDC(hwnd) };

    if hdc.is_null() {
        None
    }
    else {
        Some(hdc)
    }
}

pub fn get_window_dc(hwnd: HWND) -> Option<HDC> {
    let hdc = unsafe { GetWindowDC(hwnd) };

    if hdc.is_null() {
        None
    }
    else {
        Some(hdc)
    }
}

pub fn release_dc(hwnd: HWND, hdc: HDC) -> bool {
    unsafe { ReleaseDC(hwnd, hdc) != 0 }
}

pub fn delete_dc(hdc: HDC) -> bool {
    unsafe { DeleteDC(hdc) != 0 }
}

pub fn select_object(hdc: HDC, hgdiobj: HGDIOBJ) -> Option<HGDIOBJ> {
    let h = unsafe { SelectObject(hdc, hgdiobj) };

    if h.is_null() {
        None
    }
    else {
        Some(h)
    }
}

pub fn delete_object(hgdiobj: HGDIOBJ) -> bool {
    unsafe { DeleteObject(hgdiobj) != 0 }
}

pub fn get_object(hbitmap: HBITMAP, size: i32, bitmap: &mut BITMAP) -> bool {
    unsafe { GetObjectW(hbitmap as *mut _, size, from_mut(bitmap) as *mut _) != 0 }
}

pub fn create_compatible_dc(hdc: HDC) -> Option<HDC> {
    let hdc_mem = unsafe { CreateCompatibleDC(hdc) };

    if hdc_mem.is_null() {
        None
    }
    else {
        Some(hdc_mem)
    }
}

pub fn create_compatible_bitmap(hdc: HDC, width: i32, height: i32) -> Option<HBITMAP> {
    let hbitmap = unsafe { CreateCompatibleBitmap(hdc, width, height) };

    if hbitmap.is_null() {
        None
    }
    else {
        Some(hbitmap)
    }
}

pub fn bit_blt(hdc: HDC, source_hdc: HDC, x: i32, y: i32, width: i32, height: i32) -> bool {
    unsafe { BitBlt(hdc, 0, 0, width, height, source_hdc, x, y, SRCCOPY) != 0}
}

pub fn print_window(hwnd: HWND, hdc: HDC) -> bool {
    unsafe { PrintWindow(hwnd, hdc, PW_CLIENTONLY) != 0 }
}

pub fn get_window_di_bits(hdc: HDC, hbitmap: HBITMAP, width: i32, height: i32, buf: &mut Vec<u32>) -> bool {
    let mut bitmap_info_header: BITMAPINFOHEADER = unsafe { zeroed() };
    bitmap_info_header.biSize = size_of::<BITMAPINFOHEADER>() as u32;
    bitmap_info_header.biWidth = width;
    bitmap_info_header.biHeight = -height;
    bitmap_info_header.biPlanes = 1;
    bitmap_info_header.biBitCount = 32;
    bitmap_info_header.biCompression = BI_RGB;

    let mut bitmap_info: BITMAPINFO = unsafe { zeroed() };
    bitmap_info.bmiHeader = bitmap_info_header;

    unsafe { GetDIBits(
        hdc, 
        hbitmap, 
        0, 
        height as u32, 
        buf.as_mut_ptr() as *mut _, 
        &mut bitmap_info, 
        DIB_RGB_COLORS
    ) != 0 }
}

pub fn screenshot_on_window(hwnd: HWND) -> Result<DynamicImage, &'static str> {
    let rect = get_window_rect(hwnd).ok_or("cannot get window rect")?;
    let width = rect.right - rect.left;
    let height = rect.bottom - rect.top;

    let hdc_window = get_dc(hwnd).ok_or("cannot get window dc")?;
    #[allow(unused_assignments)]
    let mut hdc_mem: HDC = unsafe { zeroed() };
    let maybe_hdc_mem = create_compatible_dc(hdc_window);
    if let Some(hdc) = maybe_hdc_mem {
        hdc_mem = hdc;
    }
    else {
        release_dc(hwnd, hdc_window);
        return Err("cannot create memory dc");
    }

    #[allow(unused_assignments)]
    let mut hbitmap: HBITMAP = unsafe { zeroed() };
    let maybe_hbitmap = create_compatible_bitmap(hdc_window, width, height);
    if let Some(h) = maybe_hbitmap {
        hbitmap = h;
    }
    else {
        delete_dc(hdc_mem);
        release_dc(hwnd, hdc_window);
        return Err("cannot create memory bitmap");
    }

    let old_bitmap = select_object(hdc_mem, hbitmap as HGDIOBJ).unwrap();
    
    if !bit_blt(hdc_mem, hdc_window, 0, 0, width, height) {
        select_object(hdc_mem, old_bitmap).unwrap();
        delete_object(hbitmap as HGDIOBJ);
        delete_dc(hdc_mem);
        release_dc(hwnd, hdc_window);
        return Err("cannot print window");
    }

    let mut buf = vec![0u32; (width * height) as usize];
    let success = get_window_di_bits(hdc_mem, hbitmap, width, height, &mut buf);

    select_object(hdc_mem, old_bitmap).unwrap();
    delete_object(hbitmap as HGDIOBJ);
    delete_dc(hdc_mem);
    release_dc(hwnd, hdc_window);

    if success {
        let img_buf = ImageBuffer::from_raw(
            width as u32, 
            height as u32, 
            bytemuck::cast_slice(&buf).to_vec()
        ).ok_or("cannot create image buffer")?;
        Ok(DynamicImage::ImageRgba8(img_buf))
    }
    else {
        Err("cannot get di bits")
    }
}