use libc::{c_long, getrusage, rusage, suseconds_t, time_t, timeval, RUSAGE_CHILDREN, RUSAGE_SELF};
use std::fmt;
use std::ops::Sub;

#[derive(Debug, Copy, Clone)]
pub struct Usage {
    utime: f64,
    stime: f64,
    max_rss: isize,
    ix_rss: isize,
    id_rss: isize,
    is_rss: isize,
    min_flt: isize,
    maj_flt: isize,
    nswap: isize,
    in_block: isize,
    out_block: isize,
    msg_send: isize,
    msg_recv: isize,
    nsignals: isize,
    nvcsw: isize,
    nivcsw: isize,
}

impl Usage {
    pub fn new() -> Usage {
        Usage {
            utime: 0.0,
            stime: 0.0,
            max_rss: 0,
            ix_rss: 0,
            id_rss: 0,
            is_rss: 0,
            min_flt: 0,
            maj_flt: 0,
            nswap: 0,
            in_block: 0,
            out_block: 0,
            msg_send: 0,
            msg_recv: 0,
            nsignals: 0,
            nvcsw: 0,
            nivcsw: 0,
        }
    }

    pub fn new_self() -> Usage {
        let mut res = Usage::new();
        res.update_self();
        res
    }

    pub fn new_children() -> Usage {
        let mut res = Usage::new();
        res.update_children();
        res
    }

    fn new_usage() -> rusage {
        rusage {
            ru_utime: timeval {
                tv_sec: 0 as time_t,
                tv_usec: 0 as suseconds_t,
            },
            ru_stime: timeval {
                tv_sec: 0 as time_t,
                tv_usec: 0 as suseconds_t,
            },
            ru_maxrss: 0 as c_long,
            ru_ixrss: 0 as c_long,
            ru_idrss: 0 as c_long,
            ru_isrss: 0 as c_long,
            ru_minflt: 0 as c_long,
            ru_majflt: 0 as c_long,
            ru_nswap: 0 as c_long,
            ru_inblock: 0 as c_long,
            ru_oublock: 0 as c_long,
            ru_msgsnd: 0 as c_long,
            ru_msgrcv: 0 as c_long,
            ru_nsignals: 0 as c_long,
            ru_nvcsw: 0 as c_long,
            ru_nivcsw: 0 as c_long,
        }
    }

    fn tv2f64(a: timeval) -> f64 {
        a.tv_sec as f64 + a.tv_usec as f64 / 1000_000.0
    }

    fn set(&mut self, a: rusage) {
        self.utime = Usage::tv2f64(a.ru_utime);
        self.stime = Usage::tv2f64(a.ru_stime);
        self.max_rss = a.ru_maxrss as isize;
        self.ix_rss = a.ru_ixrss as isize;
        self.id_rss = a.ru_idrss as isize;
        self.min_flt = a.ru_minflt as isize;
        self.maj_flt = a.ru_majflt as isize;
        self.nswap = a.ru_nswap as isize;
        self.in_block = a.ru_inblock as isize;
        self.out_block = a.ru_oublock as isize;
        self.msg_send = a.ru_msgsnd as isize;
        self.msg_recv = a.ru_msgrcv as isize;
        self.nsignals = a.ru_nsignals as isize;
        self.nvcsw = a.ru_nvcsw as isize;
        self.nivcsw = a.ru_nivcsw as isize;
    }

    pub fn update_self(&mut self) {
        let mut usage = Usage::new_usage();
        unsafe {
            getrusage(RUSAGE_SELF, &mut usage);
        }
        self.set(usage);
    }

    pub fn update_children(&mut self) {
        let mut usage = Usage::new_usage();
        unsafe {
            getrusage(RUSAGE_CHILDREN, &mut usage);
        }
        self.set(usage);
    }
}

impl Sub for Usage {
    type Output = Usage;

    fn sub(self, other: Usage) -> Usage {
        let mut res = Usage::new();
        res.utime = self.utime - other.utime;
        res.stime = self.stime - other.stime;
        res.max_rss = self.max_rss - other.max_rss;
        res.ix_rss = self.ix_rss - other.ix_rss;
        res.id_rss = self.id_rss - other.id_rss;
        res.is_rss = self.is_rss - other.is_rss;
        res.min_flt = self.min_flt - other.min_flt;
        res.maj_flt = self.maj_flt - other.maj_flt;
        res.nswap = self.nswap - other.nswap;
        res.in_block = self.in_block - other.in_block;
        res.out_block = self.out_block - other.out_block;
        res.msg_send = self.msg_send - other.msg_send;
        res.msg_recv = self.msg_recv - other.msg_recv;
        res.nsignals = self.nsignals - other.nsignals;
        res.nvcsw = self.nvcsw - other.nvcsw;
        res.nivcsw = self.nivcsw - other.nivcsw;
        res
    }
}

impl fmt::Display for Usage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "usr={:.3}, sys={:.3}", self.utime, self.stime)
    }
}
