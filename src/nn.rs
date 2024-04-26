use burn::tensor::Tensor;
use burn_ndarray::NdArray;
use burn_ndarray::NdArrayDevice;

use crate::graph::ArgumentationFramework;
use crate::cli::{Task, Problem, Semantics};
use crate::gr_solver;
use crate::gradualsemantics_opt;
use crate::model::linear_dc_co;
use crate::model::linear_dc_st;
use crate::model::linear_ds_pr;
use crate::model::linear_ds_st;
use std::process::exit;
use std::time::Instant;

fn use_nn(task : Task, hcat : f64, nsa : f64, card : f64, maxb :f64, gr : f32, in_degree_c : f32, out_degree_c : f32, sa : f32) {
    let now = Instant::now();
    /*let mut path = String::from("../IAFGNN/model_ln/model_ln_");
    path.push_str(&task.problem_name);
    path.push_str(".pt");*/

    let v : [f32;8]= [gr, hcat as f32, card as f32, nsa as f32, maxb as f32, in_degree_c, out_degree_c, sa];
    
    if task.verbose {
        println!("{};", now.elapsed().as_millis() as f32 /1000.);
    }
    let device = NdArrayDevice::default();
    let inputs = Tensor::<NdArray, 1>::from_data(v, &device);
    //let inputs: Tensor<NdArray, 2> = input.reshape([1, 8]);

    let proba = match task.problem {
        Problem::DC => {
            match task.semantics {
                Semantics::CO => {
                    let model : linear_dc_co::Model<NdArray<f32>> = linear_dc_co::Model::default();
                    let o = model.forward(inputs);
                    let proba = o.into_scalar();
                    proba
                },
                Semantics::ST => {
                    let model : linear_dc_st::Model<NdArray<f32>> = linear_dc_st::Model::default();
                    let o = model.forward(inputs);
                    let proba = o.into_scalar();
                    proba
                },
                _ => {
                    panic!("Unsupported semantics");
                }
            }
        },
        Problem::DS => {
            match task.semantics {
                Semantics::ST => {
                    let model : linear_ds_st::Model<NdArray<f32>> = linear_ds_st::Model::default();
                    let o = model.forward(inputs);
                    let proba = o.into_scalar();
                    proba
                },
                Semantics::PR => {
                    let model : linear_ds_pr::Model<NdArray<f32>> = linear_ds_pr::Model::default();
                    let o = model.forward(inputs);
                    let proba = o.into_scalar();
                    proba
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
    
    if proba > 0.5 {
        println!("YES");
    }
    else {
        println!("NO");
    }
}
pub fn af_nn(af : ArgumentationFramework, task : Task)  {
    let start = Instant::now();
    let gr = gr_solver::solve(&af);
    if gr.contains(&task.argument) {
        println!("YES");
        if task.verbose {
			print!("None;None;");
		}
        exit(0);
    }
    for attacker in &af.af_attacker[task.argument] {
		if gr.contains(&(*attacker as usize)) {
			if task.verbose {
				print!("None;None;");
			}
			println!("NO");
            exit(0);
		}
	}
    if af.af_attackee[task.argument].contains(& (task.argument as i32)) {
        if task.verbose {
		    print!("None;None;");
		}
		println!("NO");
        exit(0);
    }
    if task.verbose {
        print!("{};", start.elapsed().as_millis() as f32 / 1000.);
    }
    let start = Instant::now();
    let n = af.nb_argument as f32 - 1.;
    let self_attack = if af.af_attacker[task.argument].contains(&(task.argument as i32)) { 0.} else { 0.5 };
    let in_degree_centrality  = af.af_attackee[task.argument].len() as f32 / n;
    let out_degree_centrality = af.af_attacker[task.argument].len() as f32 / n;
    let (hcat, nsa, card, maxb) = gradualsemantics_opt::categorizer::solve(af, task.argument);
    if task.verbose {
        print!("{};", start.elapsed().as_millis() as f32 / 1000.);
    }
    
    use_nn(task, hcat, nsa, card, maxb, 0.5, in_degree_centrality, out_degree_centrality , self_attack);
}