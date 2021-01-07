use std::collections::{HashMap, HashSet};

// common data structures

#[derive(Debug)]
pub struct Task {
    pub id: String,
    pub prio: u8,
    pub deadline: u32,
    pub inter_arrival: u32,
    pub trace: Trace,
}

//#[derive(Debug, Clone)]
#[derive(Debug)]
pub struct Trace {
    pub id: String,
    pub start: u32,
    pub end: u32,
    pub inner: Vec<Trace>,
}

// useful types

// Our task set
pub type Tasks = Vec<Task>;

// A map from Task/Resource identifiers to priority
pub type IdPrio = HashMap<String, u8>;

// A map from Task identifiers to a set of Resource identifiers
pub type TaskResources = HashMap<String, HashSet<String>>;

// Derives the above maps from a set of tasks
pub fn pre_analysis(tasks: &Tasks) -> (IdPrio, TaskResources) {
    let mut ip = HashMap::new();
    let mut tr: TaskResources = HashMap::new();
    for t in tasks {
        update_prio(t.prio, &t.trace, &mut ip);
        for i in &t.trace.inner {
            update_tr(t.id.clone(), i, &mut tr);
        }
    }
    (ip, tr)
}

// helper functions
fn update_prio(prio: u8, trace: &Trace, hm: &mut IdPrio) {
    if let Some(old_prio) = hm.get(&trace.id) {
        if prio > *old_prio {
            hm.insert(trace.id.clone(), prio);
        }
    } else {
        hm.insert(trace.id.clone(), prio);
    }
    for cs in &trace.inner {
        update_prio(prio, cs, hm);
    }
}

fn update_tr(s: String, trace: &Trace, trmap: &mut TaskResources) {
    if let Some(seen) = trmap.get_mut(&s) {
        seen.insert(trace.id.clone());
    } else {
        let mut hs = HashSet::new();
        hs.insert(trace.id.clone());
        trmap.insert(s.clone(), hs);
    }
    for trace in &trace.inner {
        update_tr(s.clone(), trace, trmap);
    }
}




//-------------------------------

fn execution_time(task: &Task) -> u32{
    let execution = task.trace.end - task.trace.start;
    execution
}

pub fn cpu_load(tasks: &Vec<Task>) -> f64 {
    let mut load: f64 = 0.0;

    for t in tasks {
        load += (t.trace.end as f64 - t.trace.start as f64) / (t.inter_arrival as f64);
    }
    load
}

fn blocking_time(task: &Task, tasks: &Vec<Task>, ip: &HashMap<String, u8>, tr: &HashMap<String, HashSet<String>>) -> u32 {
    let mut block = 0;
    let mut resources = &HashSet::new();

    match tr.get(&task.id) {
        Some(r) => resources = r,
        None => (),
    }
    
    for r in resources {
        for t in tasks {
            if t.prio < task.prio && ip.get(r).unwrap() >= &task.prio {
                let calculated_block = fetch_block(&t.trace, r);
                if calculated_block > block {
                    block = calculated_block;
                }
            }
        }
    }
    block
}

fn fetch_block(trace: &Trace, resource: &str) -> u32 {
    let mut block: u32 = 0;

    if trace.id == resource {
        block = trace.end - trace.start;
    } else if trace.inner.len() != 0 {
        for i in &trace.inner {
            let calculated_block = fetch_block(&i, resource);
            if calculated_block > block {
                block = calculated_block;
            }
        }
    }
    block
}

fn preemption_time(task: &Task, tasks: &Vec<Task>, ip: &HashMap<String, u8>, tr: &HashMap<String, HashSet<String>>, exact: bool) -> u32 {
    let preemption;
  
    if exact {
        preemption = preemption_revisited(task, tasks, ip, tr);
    } else {
        preemption = preemption_approximation(task, tasks);
    }
    preemption
}

fn preemption_approximation(task: &Task, tasks: &Vec<Task>) -> u32 {
    let mut preemption = 0;
    for t in tasks {
        if t.prio > task.prio {
            preemption += (t.trace.end - t.trace.start) * ((task.deadline + t.inter_arrival - 1) / t.inter_arrival);
        }
    }
    preemption
}

fn preemption_revisited(task: &Task, tasks: &Vec<Task>, ip: &HashMap<String, u8>, tr: &HashMap<String, HashSet<String>>) -> u32 {
    let base_case = blocking_time(task, tasks, ip, tr) + task.trace.end - task.trace.start;
    let mut preemption: u32 = 0;
    for t in tasks{
       if t.prio > task.prio{
           preemption += (base_case + t.inter_arrival - 1)/( t.inter_arrival) * (t.trace.end - t.trace.start);
       }
    }
    if (preemption + base_case) == base_case{
        return 0;
    } else{
        preemption = preemption + base_case;
        preemption
    }
}

fn response_time(task: &Task, tasks: &Vec<Task>, ip: &HashMap<String, u8>, tr: &HashMap<String, HashSet<String>>, exact: bool) -> u32{
    let response = blocking_time(task, tasks, ip, tr) + execution_time(task) + preemption_time(task, tasks, ip, tr, exact);
    response
}

pub fn srp_analysis(tasks: &Vec<Task>, ip: &HashMap<String, u8>, tr: &HashMap<String, HashSet<String>>, exact: bool,) -> Vec<(String, u32, u32, u32, u32)> {
    let mut results = Vec::new();
    for t in tasks {
        results.push((
            t.id.to_string(),
            response_time(t, tasks, ip, tr, exact),
            execution_time(t),
            blocking_time(t, tasks, ip, tr),
            preemption_time(t, tasks, ip, tr, exact),
        ))
    }

    return results;
}
