use burn::tensor::Tensor;
use burn_ndarray::NdArray;
use burn_ndarray::NdArrayDevice;

use crate::graph::ArgumentationFramework;
use crate::cli::{Task, Problem, Semantics};
use crate::gr_solver;
use crate::gradualsemantics_opt;
use crate::model::linear_dc_co;
use crate::model::linear_dc_st;
use crate::model::linear_dc_sst;
use crate::model::linear_dc_id;
use crate::model::linear_ds_pr;
use crate::model::linear_ds_sst;
use crate::model::linear_ds_st;
use std::process::exit;
use std::time::Instant;

fn use_nn(task : Task, features : [f32;9]) {
    let now = Instant::now();

    let device = NdArrayDevice::default();
    let inputs = Tensor::<NdArray, 1>::from_data(features, &device);
    
    let nn_output = match task.problem {
        Problem::DC => {
            match task.semantics {
                Semantics::CO => {
                    let model : linear_dc_co::Model<NdArray<f32>> = linear_dc_co::Model::default();
                    model.forward(inputs)
                },
                Semantics::ST => {
                    let model : linear_dc_st::Model<NdArray<f32>> = linear_dc_st::Model::default();
                    model.forward(inputs)
                },
                Semantics::SST => {
                    let model : linear_dc_sst::Model<NdArray<f32>> = linear_dc_sst::Model::default();
                    model.forward(inputs)
                },
                Semantics::ID => {
                    let model : linear_dc_id::Model<NdArray<f32>> = linear_dc_id::Model::default();
                    model.forward(inputs)
                },
                _ => {
                    panic!("Unsupported semantics");
                }
            }
        },
        Problem::DS => {
            match task.semantics {
                Semantics::PR => {
                    let model : linear_ds_pr::Model<NdArray<f32>> = linear_ds_pr::Model::default();
                    model.forward(inputs)
                },
                Semantics::ST => {
                    let model : linear_ds_st::Model<NdArray<f32>> = linear_ds_st::Model::default();
                    model.forward(inputs)
                },
                Semantics::SST => {
                    let model : linear_ds_sst::Model<NdArray<f32>> = linear_ds_sst::Model::default();
                    model.forward(inputs)
                },
                _ => {
                    panic!("Unsupported semantics");
                }
            }
        },
        _ => {
            panic!("Problem is not supported");
        }
    };
    //get the probabilty of acceptability
    let proba = nn_output.into_scalar();
    
    if task.verbose { print!("{};", now.elapsed().as_micros() as f32 /1000000.); }
    
    if proba > 0.5 { println!("YES"); }
    else { println!("NO"); }
}
pub fn af_nn(af : ArgumentationFramework, task : Task)  {
    let start = Instant::now();
    if af.af_attackee[task.argument].contains(& (task.argument as u32)) && (task.problem != Problem::DS && task.semantics != Semantics::ST) {
        if task.verbose {
            print!("None;None;");
        }
        println!("NO");
        exit(0);
    }
    let gr = gr_solver::solve(&af);
    if gr.contains(&task.argument) {
        if task.verbose {
            print!("{};", start.elapsed().as_millis() as f32 / 1000.);
            print!("None;");
		}
        println!("YES");
        exit(0);
    }
    for attacker in &af.af_attacker[task.argument] {
		if gr.contains(&(*attacker as usize)) {
			if task.verbose {
                print!("{};", start.elapsed().as_millis() as f32 / 1000.);
				print!("None;");
			}
			println!("NO");
            exit(0);
		}
	}
    
    if task.verbose {
        print!("{};", start.elapsed().as_millis() as f32 / 1000.);
    }
    let start = Instant::now();
    let n = af.nb_argument as f32 - 1.;
    let self_attack = if af.af_attacker[task.argument].contains(&(task.argument as u32)) { 0.} else { 0.5 };
    let in_degree_centrality  = af.af_attackee[task.argument].len() as f32 / n;
    let out_degree_centrality = af.af_attacker[task.argument].len() as f32 / n;
    let (hcat, nsa, card, maxb, eucli) = gradualsemantics_opt::categorizer::solve(af, task.argument);
    let features : [f32;9]= [0.5, hcat as f32, card as f32, nsa as f32, maxb as f32, eucli as f32, in_degree_centrality, out_degree_centrality, self_attack];
    
    if task.verbose {
        print!("{};", start.elapsed().as_micros() as f32 / 1000000.);
    }
    
    use_nn(task, features);
}
