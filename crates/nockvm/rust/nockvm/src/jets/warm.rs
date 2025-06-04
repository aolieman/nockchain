use crate::hamt::Hamt;
use crate::jets::cold::{Batteries, Cold};
use crate::jets::names::JET_NAME_MAP;
use crate::jets::hot::Hot;
use crate::jets::Jet;
use crate::mem::{NockStack, Preserve};
use crate::noun::{Noun, Slots};
use std::ptr::{copy_nonoverlapping, null_mut};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};


static JET_COUNTERS: OnceLock<Mutex<HashMap<Jet, usize>>> = OnceLock::new();
// TODO: incorporate into trace system
fn increment_rsjet_counter(jet: Jet) {
    let map = JET_COUNTERS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = map.lock().unwrap();
    *map.entry(jet).or_insert(0) += 1;
}

pub fn reset_and_print_rsjet_counters() {
    let map = JET_COUNTERS.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = map.lock().unwrap();

    // Only print if at least one jet was used >= 100 times
    if map.values().any(|&count| count >= 100) {
        println!("Jet usage since last report:");
        for (jet_fn, count) in map.iter() {
            if *count >= 100 {
                println!("  {:30} {}", rsjet_name(*jet_fn), count);
            }
        }
        map.clear();
    } else {
        map.clear(); // Still reset to avoid unbounded growth
    }
}



fn rsjet_name(jet: Jet) -> &'static str {
    let map =JET_NAME_MAP.lock().expect("JET_NAME_MAP is locked during lookup");
    map.get(&jet).copied().unwrap_or("unknown_jet")
}


fn noun_to_label_path(mut n: Noun) -> Vec<String> {
    let mut labels = Vec::new();

    // Walk the cons‐list until we hit a non‐cell (i.e. an atom or “empty”).
    while n.is_cell() {
        let cell = n.as_cell().unwrap();            // unwrap Cell
        let head = cell.head();                     // the first element of this cons cell
        if head.is_atom() {
            let a = head.as_atom().unwrap();        // unwrap Atom         
            match std::str::from_utf8(a.as_ne_bytes()) {  // raw “@tas” bytes
                Ok(a_str) => labels.push(a_str.trim_end_matches('\0').to_string()),
                Err(_) => labels.push(format!("{:?}", head)),
            }
        } else {
            // If for some reason it isn’t a pure atom (shouldn't happen
            // if Hot::init only conses raw “tas” atoms), we fallback:
            labels.push(format!("{:?}", head));
        }
        n = cell.tail(); // move down the list
    }

    labels.reverse(); // because Hot::init built it in reverse order
    labels
}


/// key = formula
#[derive(Copy, Clone)]
pub struct Warm(Hamt<WarmEntry>);

impl Preserve for Warm {
    unsafe fn assert_in_stack(&self, stack: &NockStack) {
        self.0.assert_in_stack(stack);
    }
    unsafe fn preserve(&mut self, stack: &mut NockStack) {
        self.0.preserve(stack);
    }
}

#[derive(Copy, Clone)]
struct WarmEntry(*mut WarmEntryMem);

const WARM_ENTRY_NIL: WarmEntry = WarmEntry(null_mut());

struct WarmEntryMem {
    batteries: Batteries,
    jet: Jet,
    path: Noun, // useful for profiling/debugging
    next: WarmEntry,
}

impl Preserve for WarmEntry {
    unsafe fn assert_in_stack(&self, stack: &NockStack) {
        if self.0.is_null() {
            return;
        };
        let mut cursor = *self;
        loop {
            stack.assert_struct_is_in(cursor.0, 1);
            (*cursor.0).batteries.assert_in_stack(stack);
            (*cursor.0).path.assert_in_stack(stack);
            if (*cursor.0).next.0.is_null() {
                break;
            };
            cursor = (*cursor.0).next;
        }
    }
    unsafe fn preserve(&mut self, stack: &mut NockStack) {
        if self.0.is_null() {
            return;
        }
        let mut ptr: *mut *mut WarmEntryMem = &mut self.0;
        loop {
            if stack.is_in_frame(*ptr) {
                (**ptr).batteries.preserve(stack);
                (**ptr).path.preserve(stack);
                let dest_mem: *mut WarmEntryMem = stack.struct_alloc_in_previous_frame(1);
                copy_nonoverlapping(*ptr, dest_mem, 1);
                *ptr = dest_mem;
                ptr = &mut ((*dest_mem).next.0);
                if (*dest_mem).next.0.is_null() {
                    break;
                };
            } else {
                break;
            }
        }
    }
}

impl Iterator for WarmEntry {
    type Item = (Noun, Batteries, Jet);
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_null() {
            return None;
        }
        unsafe {
            let res = ((*(self.0)).path, (*(self.0)).batteries, (*(self.0)).jet);
            *self = (*(self.0)).next;
            Some(res)
        }
    }
}

impl Warm {
    #[allow(clippy::new_without_default)]
    pub fn new(stack: &mut NockStack) -> Self {
        Warm(Hamt::new(stack))
    }

    fn insert(
        &mut self,
        stack: &mut NockStack,
        formula: &mut Noun,
        path: Noun,
        batteries: Batteries,
        jet: Jet,
    ) {
        let current_warm_entry = self.0.lookup(stack, formula).unwrap_or(WARM_ENTRY_NIL);
        unsafe {
            let warm_entry_mem_ptr: *mut WarmEntryMem = stack.struct_alloc(1);
            *warm_entry_mem_ptr = WarmEntryMem {
                batteries,
                jet,
                path,
                next: current_warm_entry,
            };
            self.0 = self.0.insert(stack, formula, WarmEntry(warm_entry_mem_ptr));
        }
    }

    pub fn init(stack: &mut NockStack, cold: &mut Cold, hot: &Hot) -> Self {
        let mut warm = Self::new(stack);
        for (mut path, axis, jet) in *hot {
            let batteries_list = cold.find(stack, &mut path);
            for batteries in batteries_list {
                let mut batteries_tmp = batteries;
                let (battery, _parent_axis) = batteries_tmp
                    .next()
                    .expect("IMPOSSIBLE: empty battery entry in cold state");
                if let Ok(mut formula) = unsafe { (*battery).slot_atom(axis) } {
                    warm.insert(stack, &mut formula, path, batteries, jet);
                    println!(
                        "Registered jet: {} at path: {:?}",
                        rsjet_name(jet),
                        noun_to_label_path(path)
                    );
                } else {
                    //  XX: need NockStack allocated string interpolation
                    eprintln!(
                        "Failed to register jet: {} at path: {:?} (bad axis {:?} into battery {:?})",
                        rsjet_name(jet),
                        noun_to_label_path(path),
                        axis, 
                        battery
                    );
                    continue;
                }
            }
        }
        warm
    }

    /// Walk through the linked list of WarmEntry objects and do a partial check
    /// against the subject using Batteries (walk to root of parent batteries).
    /// If there's a match, then we've found a valid jet.
    pub fn find_jet(
        &mut self,
        stack: &mut NockStack,
        s: &mut Noun,
        f: &mut Noun,
    ) -> Option<(Jet, Noun)> {
        let warm_it = self.0.lookup(stack, f)?;
        for (path, batteries, jet) in warm_it {
            if batteries.matches(stack, *s) {
                // increment_rsjet_counter(jet);
                return Some((jet, path));
            }
        }
        println!(
            "No jet matched for formula: {:?}",
            f
        );
        None
    }
}
