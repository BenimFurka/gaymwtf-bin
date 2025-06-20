use std::sync::mpsc;
use macroquad::time::get_time;
use sysinfo::{System, SystemExt, ProcessExt, Pid};
use std::thread;
use num_cpus;
use macroquad::prelude::*;

#[derive(Debug)]
pub enum SystemError {
    ProcessNotFound,
    MemoryReadError,
    CpuReadError,
}

pub struct SystemInfo {
    pub process_memory: u32,
    pub cpu_usage: f32,
    pub fps: u32,              
    frames_count: u32,    
    last_fps_update: f64,   
    receiver: mpsc::Receiver<Result<(u32, f32), SystemError>>,
    last_system_update: f64, 
    cpu_count: usize,        
    show_debug: bool,
}

impl SystemInfo {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let pid = std::process::id() as usize;
        let cpu_count = num_cpus::get();

        thread::spawn(move || {
            let mut sys = System::new_all();
            loop {
                sys.refresh_all();
                let result = sys.process(Pid::from(pid)).map_or(
                    Err(SystemError::ProcessNotFound),
                    |process| {
                        let memory = (process.memory() / 1024 / 1024) as u32;
                        let cpu = process.cpu_usage().min(100.0 * cpu_count as f32) / cpu_count as f32;
                        Ok((memory, cpu))
                    },
                );
                let _ = sender.send(result);
                thread::sleep(std::time::Duration::from_millis(1000));
            }
        });

        Self {
            process_memory: 0,
            cpu_usage: 0.0,
            fps: 0,

            frames_count: 0,
            last_fps_update: get_time(),
            receiver,
            last_system_update: get_time(),
            cpu_count,
            show_debug: false,
        }
    }

    pub fn toggle_debug(&mut self) {
        self.show_debug = !self.show_debug;
    }

    pub fn is_debug_visible(&self) -> bool {
        self.show_debug
    }

    pub fn update(&mut self) {
        self.frames_count += 1;
        let current_time = get_time();

        if current_time - self.last_fps_update >= 1.0 {
            self.fps = self.frames_count;
            self.frames_count = 0;
            self.last_fps_update = current_time;
        }

        if current_time - self.last_system_update >= 1.0 {
            if let Ok(result) = self.receiver.try_recv() {
                match result {
                    Ok((memory, cpu)) => {
                        self.process_memory = memory;
                        self.cpu_usage = cpu / self.cpu_count as f32;
                    }
                    Err(_) => {
                        self.process_memory = 0;
                        self.cpu_usage = 0.0;
                    }
                }
            }
            self.last_system_update = current_time;
        }
    }
}