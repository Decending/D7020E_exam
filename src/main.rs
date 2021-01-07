mod common;
use common::*;
mod html_generate;
use html_generate::*;
use structopt::StructOpt;

#[macro_use]
extern crate serde_derive;
extern crate open;
extern crate serde_json;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(short, long)]
    exact: bool,
}

fn main() {
    let opts = Opts::from_args();
    let t1 = Task {
        id: "T1".to_string(),
        prio: 1,
        deadline: 100,
        inter_arrival: 100,
        trace: Trace {
            id: "T1".to_string(),
            start: 0,
            end: 10,
            inner: vec![],
        },
    };

    let t2 = Task {
        id: "T2".to_string(),
        prio: 2,
        deadline: 200,
        inter_arrival: 200,
        trace: Trace {
            id: "T2".to_string(),
            start: 0,
            end: 30,
            inner: vec![
                Trace {
                    id: "R1".to_string(),
                    start: 10,
                    end: 20,
                    inner: vec![Trace {
                        id: "R2".to_string(),
                        start: 12,
                        end: 16,
                        inner: vec![],
                    }],
                },
                Trace {
                    id: "R1".to_string(),
                    start: 22,
                    end: 28,
                    inner: vec![],
                },
            ],
        },
    };

    let t3 = Task {
        id: "T3".to_string(),
        prio: 3,
        deadline: 50,
        inter_arrival: 50,
        trace: Trace {
            id: "T3".to_string(),
            start: 0,
            end: 30,
            inner: vec![Trace {
                id: "R2".to_string(),
                start: 10,
                end: 20,
                inner: vec![],
            }],
        },
    };

    // builds a vector of tasks t1, t2, t3
    let tasks: Tasks = vec![t1, t2, t3];

    println!("tasks {:?}\n", &tasks);
    // println!("tot_util {}", tot_util(&tasks));

    let (ip, tr) = pre_analysis(&tasks);
    println!("ip: {:?}\n", ip);
    println!("tr: {:?}\n", tr);

    let load = 100.0 * cpu_load(&tasks);
    println!("'exact' parameter value: {:?}\n", opts.exact);
    let srp_results = srp_analysis(&tasks, &ip, &tr, opts.exact);
    println!("{:?}\n", srp_results);
    
    render(&load, &srp_results);
    open();
}

fn open() {
    match open::that("target/srp_analysis.html") {
        Ok(exit_status) => {
	    println!("Success!")
        }
        Err(error) => println!("Failure! exit status: {:?}", error),
    }
}

/*
- What are the consequences of C(t) > A(t)?

If the execution time is larger than the inter arrival time it follows that the deadline has been missed.

- What are the consequences of load > 1?

It translates to more than 100% of the cpus capacity being required and hence the set of tasks are not schedulable.
*/
