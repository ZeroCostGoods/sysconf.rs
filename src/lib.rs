extern crate libc;

use std::result;
use self::libc::c_int;

pub type Result<T> = result::Result<T, SysconfError>;

#[derive(Debug)]
pub enum SysconfError {
    Invalid
}

pub enum SysconfVariable {
    ScArgMax = 0,
    ScChildMax = 1,
    ScClkTck = 2,
    ScNgroupsMax = 3,
    ScOpenMax = 4,
    ScStreamMax = 5,
    ScTznameMax = 6,
    ScJobControl = 7,
    ScSavedIds = 8,
    ScRealtimeSignals = 9,
    ScPriorityScheduling = 10,
    ScTimers = 11,
    ScAsynchronousIo = 12,
    ScPrioritizedIo = 13,
    ScSynchronizedIo = 14,
    ScFsync = 15,
    ScMappedFiles = 16,
    ScMemlock = 17,
    ScMemlockRange = 18,
    ScMemoryProtection = 19,
    ScMessagePassing = 20,
    ScSemaphores = 21,
    ScSharedMemoryObjects = 22,
    ScAioListioMax = 23,
    ScAioMax = 24,
    ScAioPrioDeltaMax = 25,
    ScDelaytimerMax = 26,
    ScMqOpenMax = 27,
    ScVersion = 29,
    ScPagesize = 30,
    ScRtsigMax = 31,
    ScSemNsemsMax = 32,
    ScSemValueMax = 33,
    ScSigqueueMax = 34,
    ScTimerMax = 35,
    ScBcBaseMax = 36,
    ScBcDimMax = 37,
    ScBcScaleMax = 38,
    ScBcStringMax = 39,
    ScCollWeightsMax = 40,
    ScExprNestMax = 42,
    ScLineMax = 43,
    ScReDupMax = 44,
    Sc2Version = 46,
    Sc2CBind = 47,
    Sc2CDev = 48,
    Sc2FortDev = 49,
    Sc2FortRun = 50,
    Sc2SwDev = 51,
    Sc2Localedef = 52,
    ScNprocessorsOnln = 84,
    Sc2CharTerm = 95,
    Sc2CVersion = 96,
    Sc2Upe = 97,
    ScXbs5Ilp32Off32 = 125,
    ScXbs5Ilp32Offbig = 126,
    ScXbs5LpbigOffbig = 128,
}

pub fn sysconf(name: SysconfVariable) -> Result<i64> {
    match unsafe { libc::sysconf(name as c_int) } {
        -1  => Err(SysconfError::Invalid),
        ret => Ok(ret),
    }
}
