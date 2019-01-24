

/// 进程 线程  调度 相关数据结构
/// 



/// kernel/src/arch_rv32/context.rs
pub struct Context {
    /// The stack pointer of the suspended thread.
    /// A `ContextData` is stored here.
    sp: usize,
}

/// kernel/src/memory.rs
pub struct KernelStack(usize);


/// kernel/src/process.rs/context
    pub struct Process {
        pub arch: ArchContext,
        pub memory_set: MemorySet,
        pub kstack: KernelStack,
        pub files: BTreeMap<usize, Arc<Mutex<File>>>,
        pub cwd: String,
    }



/// crate/process/src/processor.rs
pub struct Processor {
    inner: UnsafeCell<Option<ProcessorInner>>,
}

struct ProcessorInner {
    id: usize,
    proc: Option<(Pid, Box<Context>)>,
    loop_context: Box<Context>,
    manager: Arc<ProcessManager>,
}


/// crate/process/src/process_manager.rs
pub trait Context {
    unsafe fn switch_to(&mut self, target: &mut Context);
}

struct Process {
    #[allow(dead_code)]
    id: Pid,
    status: Status,
    status_after_stop: Status,
    context: Option<Box<Context>>,
    parent: Pid,
    children: Vec<Pid>,
}

pub struct ProcessManager {
    procs: Vec<Mutex<Option<Process>>>,
    scheduler: Mutex<Box<Scheduler>>,
    event_hub: Mutex<EventHub<Event>>,
}


///crate/process/src/scheduler.rs
pub trait Scheduler {
    fn insert(&mut self, pid: Pid);
    fn remove(&mut self, pid: Pid);
    fn select(&mut self) -> Option<Pid>;
    fn tick(&mut self, current: Pid) -> bool;   // need reschedule?

    fn set_priority(&mut self, pid: Pid, priority: u8);
    fn move_to_head(&mut self, pid: Pid);
}



/// 1.用户态单进程多线程模型尚未实现？
/// 
/// 2.进程线程部分数据结构命名较为混乱？
/// 
/// 3.进程内存部分以文件夹形式放回到kernel中？