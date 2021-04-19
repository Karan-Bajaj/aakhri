use rustacuda::launch;
use rustacuda::memory::DeviceBox;
use rustacuda::prelude::*;
use std::error::Error;
use std::ffi::CString;
use crate::{Voter, ElectionOutcome};

const num_runs: u32 = 500000;

pub struct CudaContext {
    module: Module,
    stream: Stream,
    _context: Context,
}

impl CudaContext {
    pub fn init() -> Result<Self, Box<dyn Error>> {
        println!("Initializing CudaContext...");
        // Initialize the CUDA API
        rustacuda::init(CudaFlags::empty()).unwrap();

        println!("Init OK");
        // Get the first device
        let device = Device::get_device(0)?;

        println!("Got Device");
        // Create a context associated to this device
        let context =
            Context::create_and_push(ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device)?;

        println!("Ready to load Kernel...");
        // Load the module containing the function we want to call
        let module_data = CString::new(include_str!("../kernel/kernel.ptx"))?;
        let module = Module::load_from_string(&module_data)?;

        // Create a stream to submit work to
        let stream = Stream::new(StreamFlags::NON_BLOCKING, None)?;

        Ok(Self {
            module,
            stream,
            _context: context,
        })
    }

    pub fn compute(&mut self, input: Vec<Voter>) -> Result<Vec<ElectionOutcome>, Box<dyn Error>> {
    }
}
