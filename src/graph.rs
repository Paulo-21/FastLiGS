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
        unsafe{
            if self.af_attacker.get_unchecked(target as usize).len() == self.af_attacker.get_unchecked(target as usize).capacity() {
                self.af_attacker.get_unchecked_mut(target as usize).reserve(10);
            }
            if self.af_attackee.get_unchecked(attacker as usize).len() == self.af_attackee.get_unchecked(attacker as usize).capacity() {
                self.af_attackee.get_unchecked_mut(attacker as usize).reserve(10);
            }
        }
        /*unsafe {
            if self.af_attacker.get_unchecked(target as usize).len() == self.af_attacker.get_unchecked(target as usize).capacity() {
                self.af_attacker.get_unchecked_mut(target as usize).reserve_exact(10);
            }
            if self.af_attackee.get_unchecked(attacker as usize).len() == self.af_attackee.get_unchecked(attacker as usize).capacity() {
                self.af_attackee.get_unchecked_mut(attacker as usize).reserve_exact(10);
            }
            let len = self.af_attacker.get_unchecked(target as usize).len();
            let end = self.af_attacker.get_unchecked_mut(target as usize).as_mut_ptr().add(len);
            ptr::write(end, attacker);
            self.af_attacker.get_unchecked_mut(target as usize).set_len(len + 1);
            //-------------------------
            let len = self.af_attackee.get_unchecked(attacker as usize).len();
            let end = self.af_attackee.get_unchecked_mut(attacker as usize).as_mut_ptr().add(len);
            ptr::write(end, target);
            self.af_attackee.get_unchecked_mut(attacker as usize).set_len(len + 1); 
        }*/
        unsafe {
            self.af_attacker.get_unchecked_mut(target as usize).push(attacker);
            self.af_attackee.get_unchecked_mut(attacker as usize).push(target);
        }
    }
}
