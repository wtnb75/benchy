#[macro_use]
extern crate log;

use std::fmt;
use std::time::{Duration, Instant};

mod rusage;

const LOW_BITS: usize = 6;
const LOW_MASK: usize = (1 << LOW_BITS) - 1;
const HIGH_BITS: usize = 25 - LOW_BITS;

#[derive(Copy, Clone)]
pub struct Data {
    cnt: usize,
    s: u128,
    s2: u128,
    max: u128,
    min: u128,
    percentile: [[u32; 1 << LOW_BITS]; HIGH_BITS],
}

pub struct Benchy {
    title: String,
    iter: Option<usize>,
    start_at: Instant,
    duration: Option<Duration>,
    prev: Instant,
    usage: rusage::Usage,
    data: Data,
}

impl Data {
    pub fn new() -> Data {
        Data {
            cnt: 0,
            s: 0,
            s2: 0,
            max: 0,
            min: std::u128::MAX,
            percentile: [[0; 1 << LOW_BITS]; HIGH_BITS],
        }
    }

    fn idx2val(idx: usize, slot: usize) -> Option<usize> {
        if slot == 0 {
            Some(idx)
        } else {
            Some((idx + (1 << LOW_BITS)) << (slot - 1))
        }
    }

    pub fn nth_from_low<'a>(&'a self, n: usize) -> Option<u128> {
        debug!("getting nth {}", n);
        let mut c = 0;
        for i in 0..HIGH_BITS {
            for j in 0..(1 << LOW_BITS) {
                let v = self.percentile[i][j] as usize;
                if c + v > n {
                    // return value
                    debug!("{}-th: got c/v={}/{}, i={}, j={}", n, c, v, i, j);
                    if i == 0 {
                        return Some(j as u128);
                    } else {
                        let v1 = Data::idx2val(j, i).unwrap();
                        let v2 = Data::idx2val(j + 1, i).unwrap();
                        let n1 = c;
                        let n2 = c + v;
                        let d =
                            v1 + ((v2 - v1) as f64 / (n2 - n1) as f64 * (n - n1) as f64) as usize;
                        return Some(d as u128);
                    }
                }
                c += v;
            }
        }
        error!("not found?");
        None
    }

    pub fn nth_from_high<'a>(&'a self, n: usize) -> Option<u128> {
        self.nth_from_low(self.cnt - n)
    }

    pub fn percentile<'a>(&'a self, percent: f64) -> Option<u128> {
        debug!("getting percentile {}", percent);
        if percent < 0.0 || 100.0 < percent {
            return None;
        }
        let nth = (self.cnt as f64 / 100.0 * percent) as usize;
        self.nth_from_low(nth)
    }

    pub fn mean(self) -> Option<f64> {
        if self.cnt < 2 {
            None
        } else {
            Some(self.s as f64 / self.cnt as f64)
        }
    }

    pub fn median(self) -> Option<u128> {
        debug!("getting median");
        self.percentile(50.0)
    }

    pub fn mode(self) -> Option<u128> {
        let mut slot = 0usize;
        let mut idx = 0usize;
        let mut maxv = 0usize;
        for i in 0..HIGH_BITS {
            for j in 0..(1 << LOW_BITS) {
                let v = self.percentile[i][j] as usize;
                let v2 = match i {
                    0 => v << HIGH_BITS,
                    1 => v << HIGH_BITS,
                    _ => v << (HIGH_BITS - (i - 1)),
                };
                if maxv < v2 {
                    maxv = v2;
                    slot = i;
                    idx = j;
                }
            }
        }
        if maxv == 0 {
            None
        } else {
            debug!("mode: idx={}, slot={}", idx, slot);
            Some(Data::idx2val(idx, slot).unwrap() as u128)
        }
    }

    pub fn register_val(&mut self, v: u128) {
        self.s += v;
        self.s2 += v * v;
        if v > self.max {
            self.max = v;
        }
        if v < self.min {
            self.min = v;
        }
        self.cnt += 1;
        let msb = match v {
            0 => 0,
            _ => 128 - v.leading_zeros() as usize - 1,
        };
        if msb < LOW_BITS {
            let slot = 0;
            let idx = v as usize;
            debug!("low: v={}, slot={}, idx={}", v, slot, idx);
            self.percentile[slot][idx] += 1;
        } else {
            let slot = msb - LOW_BITS + 1;
            // remove top bit and tail bits
            let idx = (v >> (slot - 1)) as usize & LOW_MASK;
            debug!("high: v={}, slot={}, idx={}", v, slot, idx);
            if slot > HIGH_BITS {
                error!("out of bounds: v={}, slot={}, idx={}", v, slot, idx);
            } else {
                self.percentile[slot][idx] += 1;
            }
        }
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.cnt == 0 {
            return write!(f, "(initialized)");
        } else if self.cnt == 1 {
            return write!(f, "(no data)");
        }
        let total = self.s;
        let cnt = self.cnt - 1; // XXX!
        let avg = total as f64 / cnt as f64;
        let stdev = (self.s2 as f64 / cnt as f64 - (avg * avg)).sqrt();
        let median = match self.median() {
            Some(n) => n,
            None => 0,
        };
        write!(
            f,
            "cnt={cnt}, avg={avg:.3}, median={median}, mode={mode}, stdev={stdev:.3}, min/max={min}/{max}",
            cnt = cnt,
            avg = avg,
            median = median,
            mode=self.mode().unwrap(),
            stdev = stdev,
            min = self.min,
            max = self.max
        )
    }
}

impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{{ cnt+1={}, s/s2={}/{}, min/max={}/{} }}",
            self.cnt, self.s, self.s2, self.min, self.max
        )
        .unwrap();
        self.percentile.iter().enumerate().for_each(|(i, v)| {
            write!(f, "{}:", i).unwrap();
            if *v.iter().max().unwrap() == 0 {
                write!(f, "[0]").unwrap();
            } else {
                v[..].fmt(f).unwrap();
                write!(f, "\n").unwrap()
            }
        });
        Ok(())
    }
}

impl Benchy {
    pub fn new_n(title: &str, n: usize) -> Benchy {
        Benchy {
            title: title.to_string(),
            iter: Some(n),
            start_at: Instant::now(),
            duration: None,
            prev: Instant::now(),
            usage: rusage::Usage::new_self(),
            data: Data::new(),
        }
    }
    pub fn new_duration(title: &str, duration: Duration) -> Benchy {
        Benchy {
            title: title.to_string(),
            iter: None,
            start_at: Instant::now(),
            duration: Some(duration),
            prev: Instant::now(),
            usage: rusage::Usage::new_self(),
            data: Data::new(),
        }
    }

    pub fn finish<'a>(&'a self) {
        println!("{}", self);
    }
}

impl Iterator for Benchy {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let now = Instant::now();
        match self.data.cnt {
            0 => self.data.cnt += 1, // XXX!
            _ => self.data.register_val((now - self.prev).as_micros()),
        }
        self.prev = now;
        match self.iter {
            Some(n) => {
                if n < self.data.cnt {
                    self.finish();
                    None
                } else {
                    Some(self.data.cnt - 1)
                }
            }
            None => match self.duration {
                Some(n) => {
                    if now - self.start_at < n {
                        Some(self.data.cnt - 1)
                    } else {
                        self.finish();
                        None
                    }
                }
                None => {
                    self.finish();
                    None
                }
            },
        }
    }
}

impl fmt::Display for Benchy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total = (self.prev - self.start_at).as_micros();
        let cur_usage = rusage::Usage::new_self();
        let rst_usage = cur_usage - self.usage;
        writeln!(f, "usage: {:?}", rst_usage).unwrap();
        writeln!(
            f,
            "{title}: usecs={total}, {data}",
            title = self.title,
            total = total,
            data = self.data
        )
        .unwrap();
        debug!("hist={:?}", self.data);
        Ok(())
    }
}

#[cfg(test)]
mod tests;
