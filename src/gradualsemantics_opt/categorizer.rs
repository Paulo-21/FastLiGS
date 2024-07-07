use crate::graph::ArgumentationFramework;

const EPSILON : f64 = 0.0001;

pub fn solve(af : ArgumentationFramework, task_argument : usize) -> (f64, f64, f64, f64, f64) {
    let mut nb_hit = 0;
	let mut index_to_hit = Vec::with_capacity(af.nb_argument);
	let mut never_hit = vec![true;af.nb_argument];
	let mut scores_arg : Vec<f64> = vec![1.;af.nb_argument];
	//let mut scores_arg_res : Vec<f64> = vec![1.;af.nb_argument];
	index_to_hit.push(task_argument);
	let mut old_score_t_arg_hcat = 0.;
	let mut old_score_t_arg_nsa = 0.;
	let mut old_score_t_arg_max = 0.;
	let mut old_score_t_arg_card = 0.;
	//let mut old_score_t_arg_eucli = 0.;
	unsafe {
	while nb_hit < index_to_hit.len() {
		let arg = *index_to_hit.get_unchecked(nb_hit);
		for new_arg in af.af_attacker.get_unchecked(arg) {
			if *never_hit.get_unchecked(*new_arg as usize) {
				index_to_hit.push(*new_arg as usize);
				*never_hit.get_unchecked_mut(*new_arg as usize) = false;
			}
		}
		nb_hit+=1;
	}
	loop  { // HCAT
		for arg in &index_to_hit {
			let mut sum_score_attacker = 0.;
			for new_arg in af.af_attacker.get_unchecked(*arg) {
				sum_score_attacker += scores_arg.get_unchecked(*new_arg as usize);
			}
			*scores_arg.get_unchecked_mut(*arg) = 1. / (1. + sum_score_attacker);
		}
		if (old_score_t_arg_hcat - scores_arg.get_unchecked(task_argument)).abs() <= EPSILON {
			old_score_t_arg_hcat = *scores_arg.get_unchecked(task_argument);
			break;
		}
		old_score_t_arg_hcat = *scores_arg.get_unchecked(task_argument);
		//std::mem::swap(&mut scores_arg, &mut scores_arg_res);
	}
	scores_arg.fill(1.);
	loop  { // NO self attacked
		for arg in &index_to_hit {
			let mut sum_score_attacker = 0.;
			for new_arg in af.af_attacker.get_unchecked(*arg) {
				if *arg == *new_arg as usize {
					*scores_arg.get_unchecked_mut(*arg) = 0.;
					break;
				}
				sum_score_attacker += scores_arg[*new_arg as usize];
			}
			if *scores_arg.get_unchecked(*arg) == 0. { continue; }
			*scores_arg.get_unchecked_mut(*arg) = 1. / (1. + sum_score_attacker);
		}
		if (old_score_t_arg_nsa - *scores_arg.get_unchecked(task_argument)).abs() <= EPSILON {
			old_score_t_arg_nsa = *scores_arg.get_unchecked(task_argument);
			break;
		}
		old_score_t_arg_nsa = *scores_arg.get_unchecked(task_argument);
	}
	scores_arg.fill(1.);
	loop  { // MAX Based
		for arg in &index_to_hit {
			let mut sum_score_attacker = 0.;
			for new_arg in af.af_attacker.get_unchecked(*arg) {
	            if sum_score_attacker < *scores_arg.get_unchecked(*new_arg as usize) {
                	sum_score_attacker = *scores_arg.get_unchecked(*new_arg as usize);
				}
			}
			*scores_arg.get_unchecked_mut(*arg) = 1. / (1. + sum_score_attacker);
		}
		if (old_score_t_arg_max - scores_arg.get_unchecked(task_argument)).abs() <= EPSILON {
			old_score_t_arg_max = *scores_arg.get_unchecked(task_argument);
			break;
		}
		old_score_t_arg_max = *scores_arg.get_unchecked(task_argument);
	}
	scores_arg.fill(1.);
	loop  { //CARD
		for arg in &index_to_hit {
			let mut sum_score_attacker = 0.;
			for new_arg in af.af_attacker.get_unchecked(*arg) {
				sum_score_attacker += scores_arg.get_unchecked(*new_arg as usize);
			}
			*scores_arg.get_unchecked_mut(*arg) =  1. / (1. + (sum_score_attacker / af.af_attacker.get_unchecked(*arg).len() as f64) + af.af_attacker.get_unchecked(*arg).len() as f64);
			if af.af_attacker.get_unchecked(*arg).is_empty() {
				*scores_arg.get_unchecked_mut(*arg) = 1.;
			}
		}
		if (old_score_t_arg_card - scores_arg.get_unchecked(task_argument)).abs() <= EPSILON {
			old_score_t_arg_card = *scores_arg.get_unchecked(task_argument);
			break;
		}
		old_score_t_arg_card = *scores_arg.get_unchecked(task_argument);
	}/*
	scores_arg.fill(1.);
	loop  { // Euclidian based
		for arg in &index_to_hit {
			let mut sum_score_attacker = 0.;
			for new_arg in af.af_attacker.get_unchecked(*arg) {
				sum_score_attacker += scores_arg.get_unchecked(*new_arg as usize).powi(2);
			}
			*scores_arg.get_unchecked_mut(*arg) = 1. / (1. + sum_score_attacker.sqrt());
		}
		if (old_score_t_arg_eucli - scores_arg.get_unchecked(task_argument)).abs() <= EPSILON {
			old_score_t_arg_eucli = *scores_arg.get_unchecked(task_argument);
			break;
		}
		old_score_t_arg_eucli = *scores_arg.get_unchecked(task_argument);
	}*/
	}
	(old_score_t_arg_hcat, old_score_t_arg_nsa, old_score_t_arg_card, old_score_t_arg_max, 0./*old_score_t_arg_eucli*/)
}
