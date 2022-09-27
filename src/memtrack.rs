use std::process;
use log::{  debug, };
use sysinfo::{Pid, ProcessExt, System, SystemExt};



#[derive(Debug)]
pub struct MemTrack {
    process_id:  u32,
    s_pid: Pid,
    system: System,
    total_mem:  u64,
    used_mem: u64,
    total_swap: u64,
    used_swap: u64,
}


impl MemTrack {

    pub fn new() -> MemTrack {

        let process_id = process::id();

        let system = System::new_all();
        // Display system information:
        debug!("System name:             {:?}", system.name());
        debug!("System kernel version:   {:?}", system.kernel_version());
        debug!("System OS version:       {:?}", system.os_version());
        debug!("System host name:        {:?}", system.host_name());

        let s_pid = sysinfo::get_current_pid().unwrap();
        if let Some(proc) = system.process(s_pid) {
            debug!("PROC {}", proc.pid());
        }
        else {
            debug!("Process not found");
        }
        MemTrack {  
                process_id,
                s_pid,
                system,
                total_mem:  0,
                used_mem: 0,
                total_swap: 0,
                used_swap: 0,

        }
    }


    pub fn debug_mem_change(&mut self,description: &String) {
        self.system.refresh_all();
        let total = self.system.total_memory();
        let used = self.system.used_memory();
        let total_swap = self.system.total_swap();
        let used_swap = self.system.used_swap();
        if total != self.total_mem {
            self.total_mem = total;
            debug!("CHG: total memory: {} bytes @ {}", self.total_mem,description);
        }
        if used != self.used_mem {
            self.used_mem = used;
            debug!("CHG: used memory: {} bytes @ {}", self.used_mem,description);
        }
        if total_swap != self.total_swap {
            self.total_swap = total_swap;
            debug!("CHG: total swap: {} bytes @ {}", self.total_mem,description);
        }
        if used_swap != self.used_swap {
            self.used_swap = used_swap;
            debug!("CHG: used swap: {} bytes @ {}", self.used_swap,description);
        }
    }

    pub fn debug_mem_info(&mut self,description: &String) {

        self.system.refresh_all();
        debug!("Memory Info at {}",description);
        debug!("Process {:?}",self);
        debug!("=> system:");
        // RAM and swap information:
        debug!("total memory: {} bytes", self.system.total_memory());
        debug!("used memory : {} bytes", self.system.used_memory());
        debug!("total swap  : {} bytes", self.system.total_swap());
        debug!("used swap   : {} bytes", self.system.used_swap());
        if let Some(proc) = self.system.process(self.s_pid) {
            debug!("PROC {}", proc.pid());
            debug!("memory   {}", proc.memory());
            debug!("virt mem {}", proc.virtual_memory());
        }
    }

}
