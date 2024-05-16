use crate::graph::ArgumentationFramework;

const EPSILON : f64 = 0.0001;

pub fn solve(af : ArgumentationFramework, task_argument : usize) -> (f64, f64, f64, f64, f64) {
    let mut nb_hit = 0;
	let mut index_to_hit = Vec::with_capacity(af.nb_argument);
	let mut never_hit = vec![true;af.nb_argument];
	let mut scores_arg_hcat : Vec<f64> = vec![1.;af.nb_argument]; 
	let mut scores_arg_nsa : Vec<f64> = vec![1.;af.nb_argument]; 
	let mut scores_arg_maxb : Vec<f64> = vec![1.;af.nb_argument]; 
	let mut scores_arg_card : Vec<f64> = vec![1.;af.nb_argument]; 
	let mut scores_arg_eucli : Vec<f64> = vec![1.;af.nb_argument]; 
	index_to_hit.push(task_argument);
	let mut old_score_t_arg_hcat = 0.;
	let mut old_score_t_arg_nsa = 0.;
	let mut old_score_t_arg_max = 0.;
	let mut old_score_t_arg_card = 0.;
	let mut old_score_t_arg_eucli = 0.;
	while nb_hit < index_to_hit.len() {
		let arg = index_to_hit[nb_hit];
		for new_arg in &af.af_attacker[arg] {
			if never_hit[*new_arg as usize] {
				index_to_hit.push(*new_arg as usize);
				never_hit[*new_arg as usize] = false;
			}
		}
		nb_hit+=1;
	}
	loop  { // HCAT
		for arg in &index_to_hit {
			let mut sum_score_attacker = 0.;
			for new_arg in &af.af_attacker[*arg] {
				sum_score_attacker += scores_arg_hcat[*new_arg as usize];
			}
			scores_arg_hcat[*arg] = 1. / (1. + sum_score_attacker);
		}
		if (old_score_t_arg_hcat - scores_arg_hcat[task_argument]).abs() <= EPSILON {
			break;
		}
		old_score_t_arg_hcat = scores_arg_hcat[task_argument];
	}
	loop  { // NO self attacked
		for arg in &index_to_hit {
			let mut sum_score_attacker = 0.;
			for new_arg in &af.af_attacker[*arg] {
				if *arg == *new_arg as usize {
					scores_arg_nsa[*arg] = 0.;
					break;
				}
				sum_score_attacker += scores_arg_nsa[*new_arg as usize];
			}
			if scores_arg_nsa[*arg] == 0. { continue; }
			scores_arg_nsa[*arg] = 1. / (1. + sum_score_attacker);
		}
		if (old_score_t_arg_nsa - scores_arg_nsa[task_argument]).abs() <= EPSILON {
			break;
		}
		old_score_t_arg_nsa = scores_arg_nsa[task_argument];
	}
	loop  { // MAX Based
		for arg in &index_to_hit {
			let mut sum_score_attacker = 0.;
			for new_arg in &af.af_attacker[*arg] {
				unsafe {
	            	if sum_score_attacker < *scores_arg_maxb.get_unchecked(*new_arg as usize) {
                    	sum_score_attacker = *scores_arg_maxb.get_unchecked(*new_arg as usize);
					}
				}
			}
			scores_arg_maxb[*arg] = 1. / (1. + sum_score_attacker);
		}
		if (old_score_t_arg_max - scores_arg_maxb[task_argument]).abs() <= EPSILON {
			break;
		}
		old_score_t_arg_max = scores_arg_maxb[task_argument];
	}
	loop  { //CARD
		for arg in &index_to_hit {
			let mut sum_score_attacker = 0.;
			for new_arg in &af.af_attacker[*arg] {
				unsafe {
					sum_score_attacker += scores_arg_card.get_unchecked(*new_arg as usize);
				}
			}
			scores_arg_card[*arg] =  1. / (1. + (sum_score_attacker / af.af_attacker[*arg].len() as f64) + af.af_attacker[*arg].len() as f64);
			if af.af_attacker[*arg].is_empty() {
				scores_arg_card[*arg] = 1.;
			}
		}
		if (old_score_t_arg_card - scores_arg_card[task_argument]).abs() <= EPSILON {
			break;
		}
		old_score_t_arg_card = scores_arg_card[task_argument];
	}
	loop  { // Euclidian based
		for arg in &index_to_hit {
			let mut sum_score_attacker = 0.;
			for new_arg in &af.af_attacker[*arg] {
				sum_score_attacker += scores_arg_eucli[*new_arg as usize].powi(2);
			}
			scores_arg_eucli[*arg] = 1. / (1. + sum_score_attacker.sqrt());
		}
		if (old_score_t_arg_eucli - scores_arg_eucli[task_argument]).abs() <= EPSILON {
			break;
		}
		old_score_t_arg_eucli = scores_arg_eucli[task_argument];
	}
	(old_score_t_arg_hcat, old_score_t_arg_nsa, old_score_t_arg_card, old_score_t_arg_max, old_score_t_arg_eucli)
}