// This whole file is a kludge that will panic all over if its inputs are
// bad. caveat haxx0r.

use std::fmt::Display;
use std::io::{self, prelude::*};
use std::iter::FromIterator;
use std::{env, fs};

/// Return CEIL(a/b) without using floats.
fn ceil_div(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}

/// A quick-and-dirty implementation of a bit array implemented as vec
/// of u64.  I first tried to use bitvec for this, but it was too slow
/// for some operations.
#[derive(Clone, Hash, Eq, PartialEq)]
struct BitArray {
    bits: Vec<u64>,
    sz: usize,
}

impl BitArray {
    /// Create a new BitArray of len bits, all of them set to 'val'.
    fn new(len: usize, val: bool) -> Self {
        let fill = if val { !0u64 } else { 0u64 };
        let mut bits = Vec::new();
        bits.resize(ceil_div(len, 64), fill);
        BitArray { sz: len, bits }
    }
    /// Return the size of 'self' in bits.
    fn len(&self) -> usize {
        self.sz
    }
    /// Set all bits between 'lo' and 'hi' inclusive to 'val'.
    fn set_range(&mut self, lo: usize, hi: usize, val: bool) {
        for bit in lo..=hi {
            self.set_bit(bit as usize, val);
        }
    }
    /// Sets the bit at position 'bit' to 'val'.
    fn set_bit(&mut self, bit: usize, val: bool) {
        assert!(bit < self.sz);
        let mask: u64 = 1 << ((bit as u64) & 0x3f);
        if val {
            self.bits[bit / 64] |= mask;
        } else {
            self.bits[bit / 64] &= !mask;
        }
    }
    /// Returns the bit at position 'bit'.
    fn is_set(&self, bit: usize) -> bool {
        assert!(bit < self.sz);
        self.bits[bit / 64] & 1 << ((bit as u64) & 0x3f) != 0
    }
    /// Returns true iff every bit set here is also set in b.
    ///
    /// Requires self.len() == b.len()
    fn contains(&self, b: &BitArray) -> bool {
        !self
            .bits
            .iter()
            .zip(b.bits.iter())
            .any(|(x, y)| (!x & y) != 0)
    }
    /// Returns true iff every bit that is _not_ set here is also set in b.
    fn negation_contains(&self, b: &BitArray) -> bool {
        !self
            .bits
            .iter()
            .zip(b.bits.iter())
            .any(|(x, y)| (x & y) != 0)
    }
    /// Return a BitArray containing false whereever 'self' contains
    /// true, and vice versa.
    fn negate(&self) -> BitArray {
        BitArray {
            bits: self.bits.iter().map(|x| !x).collect(),
            sz: self.sz,
        }
    }
    /// Return a BitArray containing the bitwise operation 'f' applied
    /// to every corresponding pair of u64 elements in 'self' and 'p'.
    ///
    /// (This is a helper used to implement set operations.)
    ///
    /// Requires self.len() == p.len()
    fn combine<F>(&self, p: &BitArray, f: F) -> BitArray
    where
        F: Fn((&u64, &u64)) -> u64,
    {
        assert!(self.len() == p.len());
        let bits: Vec<u64> = self.bits.iter().zip(p.bits.iter()).map(f).collect();
        BitArray { bits, sz: self.sz }
    }
    /// Return the set intersection of two bitarrays.
    ///
    /// Requires self.len() == p.len()
    fn intersect(&self, p: &BitArray) -> BitArray {
        self.combine(p, |(a, b)| a & b)
    }
    /// Return the set subtaction of two bitarrays.
    ///
    /// Requires self.len() == p.len()
    fn subtract(&self, p: &BitArray) -> BitArray {
        self.combine(p, |(a, b)| a & !b)
    }
    /// Return the set subtaction of two bitarrays.
    ///
    /// Requires self.len() == p.len()
    fn count(&self) -> u32 {
        self.bits.iter().fold(0, |acc, i| acc + i.count_ones())
    }
    /// Grow this bit array so that it can hold at least 'sz'
    /// elements.  Fill new bits with false.
    fn grow(&mut self, sz: usize) {
        if sz < self.sz {
            return;
        }
        let sz = ceil_div(sz, 64);
        self.bits.resize(sz, 0u64);
        self.sz = sz * 64
    }
}

// Lets us use "collect()" on an iter of bool to produce a BitArray
impl FromIterator<bool> for BitArray {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = bool>,
    {
        let mut result = BitArray::new(64, false);
        for (i, bit) in iter.into_iter().enumerate() {
            result.grow(i + 1);
            result.set_bit(i, bit);
        }
        result
    }
}

/// A Policy is set of accepted ports, implemented as a bitarray.
#[derive(Clone, Hash, Eq, PartialEq)]
struct Policy {
    allow: BitArray,
}

const N_PORTS: usize = 65536;

impl Default for Policy {
    fn default() -> Self {
        Policy::new(false)
    }
}

impl Policy {
    /// Make a new policy whose default value for every port is 'val'
    fn new(val: bool) -> Self {
        Policy {
            allow: BitArray::new(N_PORTS, val),
        }
    }
    /// Set the status for ports between lo and hi inclusive to 'val'.
    /// Clips 'lo' and 'hi' so that they are in range and port 0 is
    /// ignored.
    fn set_range(&mut self, mut lo: u16, mut hi: u16, val: bool) {
        if lo < 1 {
            lo = 1;
        }
        if lo as usize >= N_PORTS {
            return;
        }
        if hi as usize >= N_PORTS {
            hi = (N_PORTS - 1) as u16;
        }
        self.allow.set_range(lo as usize, hi as usize, val);
    }

    // (these functions just delegate to BitArray)

    fn allows(&self, port: u16) -> bool {
        self.allow.is_set(port as usize)
    }
    fn contains(&self, p: &Policy) -> bool {
        self.allow.contains(&p.allow)
    }
    fn negation_contains(&self, p: &Policy) -> bool {
        self.allow.negation_contains(&p.allow)
    }
    fn negate(&self) -> Policy {
        Policy {
            allow: self.allow.negate(),
        }
    }
    fn intersect(&self, p: &Policy) -> Policy {
        Policy {
            allow: self.allow.intersect(&p.allow),
        }
    }
}

// Write a policy in a human-readable form.
impl Display for Policy {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn dump(
            first: bool,
            lo: u16,
            hi: u16,
            fmt: &mut std::fmt::Formatter<'_>,
        ) -> std::fmt::Result {
            if !first {
                write!(fmt, ",")?;
            }
            if lo == hi {
                write!(fmt, "{}", lo)
            } else {
                write!(fmt, "{}-{}", lo, hi)
            }
        }

        let mut span: Option<(u16, u16)> = None;
        let mut first: bool = true;
        for port in 1..=(N_PORTS - 1) as u16 {
            if self.allows(port) {
                span = match span {
                    None => Some((port, port)),
                    Some((lo, hi)) => {
                        if hi == port - 1 {
                            Some((lo, port))
                        } else {
                            dump(first, lo, hi, fmt)?;
                            first = false;
                            Some((port, port))
                        }
                    }
                }
            }
        }
        if let Some((lo, hi)) = span {
            dump(first, lo, hi, fmt)?;
        }
        Ok(())
    }
}

// Given a list of policies, find a set of disjoint "covering"
// policies such that every policy in the input can be made as the
// union of some subset of the covering policies.
//
// Return a vector of (policy,count) tuples where each "count" is the
// number of input policies that use the covering policy.
fn find_cover(policies: &[Policy]) -> Vec<(Policy, u32)> {
    // based on ian goldberg's perl code in the walkingonions pape repo
    let mut classes = Vec::new();

    classes.push((read_policy_from_line("p accept 1-65535").unwrap(), 0u32));

    for pol in policies.iter() {
        let mut newclasses = Vec::new();
        for (class, count) in classes.into_iter() {
            if pol.contains(&class) {
                newclasses.push((class, count + 1));
            } else if pol.negation_contains(&class) {
                newclasses.push((class, count));
            } else {
                let not_pol = pol.negate();
                let c1 = class.intersect(&pol);
                let c2 = class.intersect(&not_pol);
                newclasses.push((c1, count + 1));
                newclasses.push((c2, count));
            }
        }
        classes = newclasses;
    }

    classes
}

/// Dump out a set of covering sets for some input policies.
fn print_cover(policies: &[Policy]) {
    let classes = find_cover(policies);
    for (cl, ct) in classes {
        println!("{}: {}", ct, cl);
    }
}

/// For a given covering policy, return a bitarray representing which
/// members of 'policies' support every one of its ports.
fn find_supporting_policies(covering: &Policy, policies: &[Policy]) -> BitArray {
    policies.iter().map(|p| p.contains(covering)).collect()
}

/// Calculate the "loss" from pset1 (a bitarray representing a group
/// of policies what support some covering-policy) that would result
/// from combining pset1 with pset2.
///
/// The "loss" is the number of relays in pset1 that we wouldn't be
/// able to list any more if we could only list the arrays supporting
/// both pset1 and pset2.
fn combining_loss(pset1: &BitArray, pset2: &BitArray) -> u32 {
    pset1.subtract(pset2).count()
}

/// Find covering policy sets, and dump out the impact of combining
/// each set with each other set, if the result isn't "too bad".
fn emit_loss_graph(policies: &[Policy]) {
    let cover0 = find_cover(policies);
    let covering_sets: Vec<_> = cover0
        .iter()
        .map(|(c, _)| (c, find_supporting_policies(c, policies)))
        .collect();

    println!("PORTS = [");
    for (c1, _) in covering_sets.iter() {
        println!("    '{}',", c1);
    }
    println!("]");

    println!("GRAPH=[");
    for (_c1, pol1) in covering_sets.iter() {
        let count1 = pol1.count();
        print!("    [");
        for (_c2, pol2) in covering_sets.iter() {
            /*
            // this weighing treats all (port-relay) units as equally
            // important, so the first port 25 is just as important as
            // the 100th port 443.  Other possibilities are possible.
            let loss = combining_loss(pol1, pol2);
             */

            // This weighting treats losing all ports as equally
            // important, but treats losing 10% of port 24 as being
            // just as bad as losing 10% of port 443.
            let loss = combining_loss(pol1, pol2) as f64 / count1 as f64;

            print!("{},", loss);
        }
        println!("],");
    }
    println!("]");
}

/// Find covering policy sets, and dump out the impact of combining
/// each set with each other set, if the result isn't "too bad".
fn analyze_coverage_combination(policies: &[Policy]) {
    let cover0 = find_cover(policies);
    let covering_sets: Vec<_> = cover0
        .iter()
        .map(|(c, _)| (c, find_supporting_policies(c, policies)))
        .collect();

    for (c1, pol1) in covering_sets.iter() {
        let count1 = pol1.count();
        println!("Analyzing set {} ({})", c1, pol1.count());

        for (c2, pol2) in covering_sets.iter() {
            let c1loss = combining_loss(pol1, pol2);
            let c2loss = combining_loss(pol2, pol1);
            let count2 = pol2.count();
            // A combination is "acceptable" if it gives less than 5%
            // loss from either set.
            let acceptable = c1loss * 20 <= count1 && c2loss * 20 <= pol2.count();
            if acceptable && c1 != c2 {
                println!(
                    "\t {}: {}/{} lost [{}/{}]",
                    c2, c1loss, count1, c2loss, count2
                );
            }
        }
    }
}


/// If "line" is well-formed "p accept" or "p reject" line, parse it into a
/// Policy. Otherwise, return None.
fn read_policy_from_line(line: &str) -> Option<Policy> {
    /// Helper: Parse a single 'a-b' or 'a' range.
    fn set_item_in_policy(policy: &mut Policy, item: &str, val: bool) {
        let parts: Vec<_> = item.split('-').collect();
        let (s_lo, s_hi) = match parts.len() {
            1 => (parts[0], parts[0]),
            2 => (parts[0], parts[1]),
            _ => {
                return;
            }
        };
        let lo: u16 = s_lo.parse().unwrap(); // panic
        let hi: u16 = s_hi.parse().unwrap(); // panic
        policy.set_range(lo, hi, val);
    }

    if !line.starts_with("p ") {
        return None;
    }

    let line = &line[2..];
    let spacepos = line.find(' ')?;
    let kwd = &line[..spacepos];
    let line = &line[spacepos + 1..];

    let is_accept = match kwd {
        "accept" => true,
        "reject" => false,
        _ => return None,
    };

    let mut result = Policy::new(!is_accept);
    for item in line.split(',') {
        set_item_in_policy(&mut result, item, is_accept);
    }

    Some(result)
}

/// Fetch all the policies from a cached-microdescs* file.
fn read_policies<R: io::Read>(r: io::BufReader<R>) -> io::Result<Vec<Policy>> {
    let mut policies = Vec::new();
    for line in r.lines() {
        let line = line?;
        if let Some(p) = read_policy_from_line(&line[..]) {
            policies.push(p);
        }
    }
    // println!("Found {} policies", policies.len());
    Ok(policies)
}

/// For each port, print the number of policies supporting that port.
fn print_portcount(policies: &[Policy]) {
    let mut portcount = vec![0u32; N_PORTS];
    for p in policies.iter() {
        for (portno, val) in portcount.iter_mut().enumerate() {
            if p.allows(portno as u16) {
                *val += 1;
            }
        }
    }

    for (port, count) in portcount.iter().enumerate() {
        println!("{}: {}", port, count);
    }
}

enum Command {
    Portcount,
    Cover,
    CoverLoss,
    LossGraph,
}

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        println!("I want an command and a filename.");
        return Ok(());
    }
    let cmd = args.get(1).unwrap();

    use Command::*;
    let cmd = match &cmd[..] {
        "portcount" => Portcount,
        "cover" => Cover,
        "cover-loss" => CoverLoss,
        "loss-graph" => LossGraph,
        _ => {
            println!("Recognized commands: portcount, cover, cover-loss");
            return Ok(());
        }
    };
    let fname = args.get(2).unwrap();

    let policies = {
        let mut file = fs::File::open(fname).unwrap();
        let r = io::BufReader::new(&mut file);
        read_policies(r)
    }?;

    match cmd {
        Portcount => print_portcount(&policies),
        Cover => print_cover(&policies),
        CoverLoss => analyze_coverage_combination(&policies),
        LossGraph => emit_loss_graph(&policies),
    }

    Ok(())
}
