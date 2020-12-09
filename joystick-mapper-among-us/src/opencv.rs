use opencv_sys as ffi;
pub use opencv_sys::Point;
pub use opencv_sys::Moment;

//Bastardised subset of opencv I need

pub const CV_8UC1: i32 = 0;
pub const CV_8UC4: i32 = 24;
pub const RETR_LIST: i32 = 1;
pub const CHAIN_APPROX_SIMPLE: i32 = 2;

#[derive(Debug)]
pub struct Mat {
    pub(crate) inner: ffi::Mat,
}

impl Clone for Mat {
    fn clone(&self) -> Self {
        Mat {
            inner: unsafe { ffi::Mat_Clone(self.inner) },
        }
    }
}

impl Drop for Mat {
    fn drop(&mut self) {
        unsafe { ffi::Mat_Close(self.inner) }
    }
}

impl Default for Mat {
    fn default() -> Mat {
        Mat { inner: unsafe { ffi::Mat_New() } }
    }
}

impl Mat {
    pub fn new_from_bytes(rows: i32, cols: i32, type_: i32, buffer: &[u8]) -> Mat {
        let bytearray = ffi::ByteArray {
            data: buffer.as_ptr() as *mut i8,
            length: buffer.len() as i32
        };
        Mat { 
            inner: unsafe { ffi::Mat_NewFromBytes(rows, cols, type_, bytearray) }
        }
    }
}

pub fn bitwise_or(src1: &Mat, src2: &Mat, dst: &mut Mat) {
    unsafe { ffi::Mat_BitwiseOr(src1.inner, src2.inner, dst.inner) }
}

pub fn in_range(src: &Mat, lowerb: &Mat, upperb: &Mat, dst: &mut Mat) {
    unsafe { ffi::Mat_InRange(src.inner, lowerb.inner, upperb.inner, dst.inner) }
}

pub fn find_contours(src: &Mat, mode: i32, method: i32) -> Vec<Vec<Point>> {
    let contours_raw = unsafe { ffi::FindContours(src.inner, mode, method) };
    let contours_array = unsafe { std::slice::from_raw_parts(contours_raw.contours, contours_raw.length as usize) };
    let mut contours = Vec::default();
    for contour in contours_array {
        let points_array = unsafe { std::slice::from_raw_parts(contour.points, contour.length as usize) };
        let mut points = Vec::default();
        for point in points_array {
            points.push(*point);
        }
        contours.push(points);
    }
    contours
}

pub fn contour_area(contour: &mut Vec<Point>) -> f64 {
    let points = ffi::Points {
        points: contour.as_mut_ptr(),
        length: contour.len() as i32,
    };
    unsafe { ffi::ContourArea(points) }
}

// pub fn imwrite(path: String, src: &Mat) -> bool {
//     unsafe { opencv_sys::Image_IMWrite(std::ffi::CString::new(path).unwrap().as_ptr(), src.inner) }
// }