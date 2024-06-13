pub struct ArgumentationFramework {
    pub af_attacker : Vec<Vec<u32>>,
	pub af_attackee : Vec<Vec<u32>>,
    pub nb_argument : usize
}

impl ArgumentationFramework {
    pub fn new(nb_arg : usize) -> Self {
        let af_attackee = vec![Vec::new();nb_arg];
        let af_attacker = vec![Vec::new();nb_arg];
        Self { af_attackee , af_attacker, nb_argument : nb_arg }
    }
    #[inline(always)]
    pub fn add_attack(&mut self, attacker : u32, target : u32) {
        /*attacker = attacker-1;
        target = target-1;*/
        /*if self.af_attacker.len() == self.af_attacker.capacity() {
            self.af_attacker[target as usize].reserve(5);
            self.af_attackee[attacker as usize].reserve(5);
        }
        unsafe {
            let len = self.af_attacker[target as usize].len();
            let end = self.af_attacker[target as usize].as_mut_ptr().add(len);
            ptr::write(end, attacker);
            self.af_attacker[target as usize].set_len(len + 1);
            //-------------------------
            let len = self.af_attackee[attacker as usize].len();
            let end = self.af_attackee[attacker as usize].as_mut_ptr().add(len);
            ptr::write(end, target);
            self.af_attackee[attacker as usize].set_len(len + 1);
        }*/
        self.af_attacker[target as usize].push(attacker);
        self.af_attackee[attacker as usize].push(target);
    }
}
