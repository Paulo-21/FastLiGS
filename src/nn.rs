use tch::nn::Module;
use crate::graph::ArgumentationFramework;
use crate::cli::Task;
use crate::gr_solver;
use crate::gradualsemantics_opt;
use std::process::exit;
use std::time::Instant;

fn use_nn(task : Task, hcat : f64, nsa : f64, card : f64, maxb :f64) {
    let now = Instant::now();
    let mut path = String::from("../IAFGNN/model_save/model_ln_");
    path.push_str(&task.problem_name);
    path.push_str(".pt");
    let v : [f32;4]= [hcat as f32, nsa as f32, card as f32, maxb as f32];
    let input = tch::Tensor::from_slice(&v);
    let model = tch::CModule::load(path).unwrap();
    let output = model.forward(&input);
    if task.verbose {
        println!("{};", now.elapsed().as_millis()/1000);
    }
    //output.print();
    let mut r :[f32;1]= [0.];
    output.copy_data(&mut r, 1);
    //println!("{:?}", r);
    if r[0] > 0.5 {
        println!("YES");
    }
    else {
        println!("NO");
    }
}
pub fn af_nn(af : ArgumentationFramework, task : Task)  {
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
    let start = Instant::now();
    //let maxb = gradualsemantics_opt::max_based::solve(&af, &task);
    //let card = gradualsemantics_opt::card_based::solve(&af, &task);
    let (hcat, nsa, card, maxb) = gradualsemantics_opt::categorizer::solve(af, task.argument);
    if task.verbose {
        print!("{};", start.elapsed().as_millis() as f32 / 1000.);
    }
    use_nn(task, hcat, nsa, card, maxb);
}