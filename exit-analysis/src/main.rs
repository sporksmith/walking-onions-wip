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
    /// Return the set unionn of two bitarrays.
    ///
    /// Requires self.len() == p.len()
    fn union(&self, p: &BitArray) -> BitArray {
        self.combine(p, |(a, b)| a | b)
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
        self.bits.iter().map(|n| n.count_ones()).sum()
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
    fn port_count(&self) -> u32 {
        self.allow.count()
    }
    fn union(&self, p: &Policy) -> Policy {
        Policy {
            allow: self.allow.union(&p.allow),
        }
    }
}

// Write a policy in a human-readable form, as a list of accepted port-ranges
impl Display for Policy {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Helper: dump a range.
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

    // TODO: we don't actually use these 'count' values, and a policy
    // bitmap would be more appropriate.

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

/// For a given covering policy, return a bitarray representing which
/// members of 'policies' support every one of its ports.
fn find_supporting_policies(covering: &Policy, policies: &[Policy]) -> BitArray {
    policies.iter().map(|p| p.contains(covering)).collect()
}

/// A pair containing a set of accepted ports (the policy), and a bitarray
/// representing which relays' policies support all of those ports.
#[derive(Clone, Eq, PartialEq)]
struct PolSupporters {
    ports: Policy,
    relays: BitArray,
}

impl PolSupporters {
    /// Construct a PolSupporters from a set of `ports`, and from a list
    /// of all the relays.  Note that PolSupporters objects constructed
    /// from differerent lists of relays are not interoperable.
    fn from_port_set(ports: Policy, all_policies: &[Policy]) -> Self {
        let relays = find_supporting_policies(&ports, all_policies);
        PolSupporters { ports, relays }
    }
    /// How many ports does this policy support?
    fn port_count(&self) -> u32 {
        self.ports.port_count()
    }
    /// How many relay support this policy?
    fn relay_count(&self) -> u32 {
        self.relays.count()
    }
    /// What is the "value" of this policy?  This is whatever we are
    /// are trying to maximize when we combine policy groups together.
    fn value(&self) -> u32 {
        // Here we define the value of a policy simply as the number of
        // exit ports it contains, times the number of relays supporting it.
        //
        // TODO: In production we should probably take bandwidth into
        // account, and we should probably treat some ports
        // (e.g. https) as more important than others (e.g., chargen, echo,
        // random high-ports not assigned to any service).
        self.ports.port_count() * self.relays.count()
    }
    /// What is the "cost" of combining this policy with `ps`?  We
    /// define the cost as the amount of value() we would lose when we
    /// combine the two sets into one.  This function must therefore
    /// be consistent with value() and combine().
    fn combining_cost(&self, ps: &PolSupporters) -> u32 {
        // number of relays supporting us but not ps.
        let r1 = self.relays.subtract(&ps.relays).count();
        // number of relays supporting ps but not us.
        let r2 = ps.relays.subtract(&self.relays).count();

        r1 * self.port_count() + r2 * ps.port_count()
    }
    /// Join two policy sets into one: the new set contains more ports
    /// (all the ports from the two sets that were combined) and fewer
    /// relays (only those relays that were in both parent sets).
    ///
    /// Return a tuple of the new set and of the value lost by doing the
    /// combination.
    fn combine(&self, ps: &PolSupporters) -> (PolSupporters, u32) {
        let ports = self.ports.union(&ps.ports);
        let relays = self.relays.intersect(&ps.relays);
        let result = PolSupporters { ports, relays };
        let cc = self.combining_cost(ps);
        assert_eq!((self.value() + ps.value()) - result.value(), cc);
        (result, cc)
    }
}

/// Given an interator returning (Policy,u32) pairs, construct a
/// vector containing, for each port, the number of policies that
/// suppport it, weighted by the paired u32 values.
fn get_portcount<'a, I1>(weighted_policies: I1) -> Vec<u32>
where
    I1: IntoIterator<Item = (&'a Policy, u32)>,
{
    let mut portcount = vec![0u32; N_PORTS];
    for (p, w) in weighted_policies.into_iter() {
        for (portno, val) in portcount.iter_mut().enumerate() {
            if p.allows(portno as u16) {
                *val += w;
            }
        }
    }
    portcount
}

/// Use a greedy algorithm to look for a good partition of ports, given
/// a list of relay `policies`.  Uses PolSupporters::value() to decide
/// how "good" a partition is.  Tries to make a partition into `target` sets.
/// Prints the partition and its cost.
fn greedy_combine_coverage(policies: &[Policy], target: usize) {
    assert!(target >= 1);
    // how many relays are there in total?
    let n_relays = policies.len();
    // How many relays support each port in the original set of policies?
    let orig_portcount = get_portcount(policies.iter().map(|p| (p, 1)));
    // Start by using find_cover() to get a minimal set of policies
    // such that every relay's policy can be built as a combination of
    // some policies in the set.  This will have the highest possible
    // "value" (since every relay's policy is reflected here in full),
    // but will have too many different policies for convenient use
    // with Walking Onions.
    let mut combined: Vec<_> = find_cover(policies)
        .into_iter()
        .map(|(c, _)| PolSupporters::from_port_set(c, policies))
        .collect();

    // Total "value" of the original port partition.
    let orig_value: u32 = combined.iter().map(|ps| ps.value()).sum();
    // Keep track of how much "value" we've paid, to double-check.
    let mut total_cost = 0;

    while combined.len() > target {
        // Here's the greedy algorithm: with each step, we look at all
        // distinct pairs of policies, and find the pair that would
        // have the lowest cost to combine.  Then we combine that pair
        // into one, and continue.

        // 1. Find the best policy.
        //
        // (TODO this recalculates a lot of values in every iteration.
        // We don't need to actually need to recalculate the ones that didn't
        // change.)
        let mut best_cost = std::u32::MAX;
        let mut best_idx = None;
        for (idx1, psup1) in combined.iter().enumerate() {
            'inner: for (idx2, psup2) in combined.iter().enumerate() {
                if idx1 >= idx2 {
                    continue 'inner;
                }
                let cost = psup1.combining_cost(psup2);
                if cost < best_cost {
                    best_cost = cost;
                    best_idx = Some((idx1, idx2));
                }
            }
        }
        // 2. We found a pair to combine.
        if let Some((idx1, idx2)) = best_idx {
            assert!(idx1 < idx2);
            // remove idx2 before idx1, to avoid index problems.
            let psup2 = combined.remove(idx2);
            let psup1 = combined.remove(idx1);
            let (newval, cost) = psup1.combine(&psup2);
            assert_eq!(cost, best_cost);
            combined.push(newval);
            println!(
                "[{}] Cost {}: Combined {} and {}",
                combined.len(),
                cost,
                psup1.ports,
                psup2.ports
            );
            total_cost += cost;
        } else {
            // Somehow we didn't find anything to combine. Probably an error.
            break;
        }
    }
    // Double-check total_cost is correct according to our value function.
    let cur_value = combined.iter().fold(0, |acc, ps| acc + ps.value());
    assert_eq!(orig_value - cur_value, total_cost);

    // Recalculate a new version of orig_portcount (how many relays
    // now support each port).
    let final_portcount = get_portcount(
        combined
            .iter()
            .map(|ps| (&ps.ports, ps.relay_count() as u32)),
    );

    // Compare orig_portcount and final_portcount, to get the
    // percentage of relays supporting each port that are no longer
    // listed.
    let mut port_loss: Vec<_> = orig_portcount
        .into_iter()
        .zip(final_portcount.into_iter().enumerate())
        .map(|(a, (idx, b))| (idx, a, (100 * (a - b)) as f64 / (a as f64)))
        .collect();

    println!("===================== DONE.");
    // Print the final port partition.
    for (idx, psup) in combined.iter().enumerate() {
        println!(
            "Set {} [{}/{} relays]: {}",
            idx + 1,
            psup.relay_count(),
            n_relays,
            psup.ports
        );
    }

    // Describe the cost of achieving this partition.
    println!(
        "We lost {}/{} ports getting down to {} sets. [{:.5}%]",
        total_cost,
        orig_value,
        target,
        (total_cost * 100) as f64 / (orig_value as f64)
    );

    println!("Worst fraction lost by port:");
    port_loss.sort_by(|(_, _, x), (_, _, y)| x.partial_cmp(y).unwrap());
    for (port, orig, loss) in port_loss.iter().rev().take(10) {
        println!("Port {}: {:.2}% of {}", port, loss, orig);
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

/// Dump out a set of covering sets for some input policies.
fn print_cover(policies: &[Policy]) {
    let classes = find_cover(policies);
    for (cl, ct) in classes {
        println!("{}: {}", ct, cl);
    }
}

/// For each port, print the number of policies supporting that port.
fn print_portcount(policies: &[Policy]) {
    let portcount = get_portcount(policies.iter().zip(std::iter::repeat(1u32)));

    for (port, count) in portcount.iter().enumerate() {
        println!("{}: {}", port, count);
    }
}

enum Command {
    Portcount,
    Cover,
    Greedy(usize),
}

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        println!("I want an command and a filename (like cached-microdescs)");
        return Ok(());
    }
    let cmd = args.get(1).unwrap();

    use Command::*;
    let cmd = match &cmd[..] {
        "portcount" => Portcount,
        "cover" => Cover,
        "greedy-16" => Greedy(16),
        "greedy-8" => Greedy(8),
        "greedy-4" => Greedy(4),
        _ => {
            println!("Recognized commands: portcount, cover, greedy-{{16,8,4}}");
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
        Greedy(n) => greedy_combine_coverage(&policies, n),
    }

    Ok(())
}
