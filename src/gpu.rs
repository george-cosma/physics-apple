use rustacuda::prelude::*;
use std::error::Error;
use std::ffi::CString;

use crate::gui::{HEIGHT, WIDTH};

// WIDTH  = THREADS_X * BLOCKS.X
// HEIGHT = THREADS_Y * BLOCKS.Y
// THREADS_X * THREADS_Y should be divisible by 32, and between 256 and 512
// WIDTH = 480
// HEIGHT = 360
const THREADS_X: u32 = 32;
const BLOCKS_X: u32 = (WIDTH / THREADS_X) + 1; // 45
const THREADS_Y: u32 = 32;
const BLOCKS_Y: u32 = (HEIGHT / THREADS_Y) + 1; // 15

const VALUES: usize = (WIDTH * HEIGHT) as usize;

pub fn cuda_generate_static_field(
    mass_product: f32,
    att_x: Vec<i32>,
    att_y: Vec<i32>,
) -> Result<(Vec<f32>, Vec<f32>), Box<dyn Error>> {
    // Set up the context, load the module, and create a stream to run kernels in.
    rustacuda::init(CudaFlags::empty())?;
    let device = Device::get_device(0)?;
    println!("{}", device.name().unwrap());
    let _ctx = Context::create_and_push(ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device)?;

    let ptx = CString::new(include_str!("./shaders/static-field.ptx"))?;
    let module = Module::load_from_string(&ptx)?;
    let stream = Stream::new(StreamFlags::DEFAULT, None)?;

    println!("[CUDA] Copying data to device");
    // Create buffers for data
    let mut in_x2 = DeviceBuffer::from_slice(att_x.as_slice())?;
    let mut in_y2 = DeviceBuffer::from_slice(att_y.as_slice())?;
    let mut out_y = DeviceBuffer::from_slice(&vec![0.0f32; VALUES])?;
    let mut out_x = DeviceBuffer::from_slice(&vec![0.0f32; VALUES])?;
    println!("[CUDA] Copying data to device ... DONE");
    println!("[CUDA] Running kernel for {} attractors", att_x.len());

    // This kernel adds each element in `in_x` and `in_y` and writes the result into `out`.
    unsafe {
        // gravity(const double mass_product, const int* x2, const int* y2, const int attractors, double* out_x, double* out_y) {
        launch!(module.gravity<<<(BLOCKS_X, BLOCKS_Y, 1), (THREADS_X, THREADS_Y, 1), 0, stream>>>(
            mass_product,
            in_x2.as_device_ptr(),
            in_y2.as_device_ptr(),
            in_x2.len() as i32,
            out_x.as_device_ptr(),
            out_y.as_device_ptr(),
            WIDTH as i32,
            HEIGHT as i32
        ))?;
    }

    // Kernel launches are asynchronous, so we wait for the kernels to finish executing.
    stream.synchronize()?;

    println!("[CUDA] Copying data to host");
    // Copy the results back to host memory
    let mut out_host_x = vec![0.0f32; VALUES]; //[0.0f32; THREADS];
    let mut out_host_y = vec![0.0f32; VALUES]; //[0.0f32; THREADS];
    out_y.copy_to(&mut out_host_y)?;
    out_x.copy_to(&mut out_host_x)?;
    println!("[CUDA] Copying data to host ... DONE");

    Ok((out_host_x, out_host_y))
}
