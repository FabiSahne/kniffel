pub struct Score {
    einer: i32,
    einerb: bool,
    zweier: i32,
    zweierb: bool,
    dreier: i32,
    dreierb: bool,
    vierer: i32,
    viererb: bool,
    funfer: i32,
    funferb: bool,
    sechser: i32,
    sechserb: bool,
    bonus: i32,
    dreierp: i32,
    dreierpb: bool,
    viererp: i32,
    viererpb: bool,
    full_house: i32,
    full_houseb: bool,
    kleine_str: i32,
    kleine_strb: bool,
    grosse_str: i32,
    grosse_strb: bool,
    kniffel: i32,
    kniffelb: bool,
    chance: i32,
    chanceb: bool,
}

impl Score{
    pub fn score_builder() -> Score {
        Score {
            einer: 0,
            einerb: false,
            zweier: 0,
            zweierb: false,
            dreier: 0,
            dreierb: false,
            vierer:0,
            viererb:false,
            funfer: 0,
            funferb: false,
            sechser: 0,
            sechserb: false,
            bonus: 0,
            dreierp: 0,
            dreierpb: false,
            viererp: 0,
            viererpb: false,
            full_house: 0,
            full_houseb: false,
            kleine_str: 0,
            kleine_strb: false,
            grosse_str: 0,
            grosse_strb: false,
            kniffel: 0,
            kniffelb: false,
            chance: 0,
            chanceb: false }
    }
    pub fn get_einer(&self) -> (i32, bool) {
        (self.einer, self.einerb)
    }
    pub fn set_einer(&mut self, zahl: i32) -> bool {
        if !self.get_einer().1 {
            self.einer = zahl;
            self.einerb = true;
            true
        } else {
            false
        }
    }
    pub fn get_zweier(&self) -> (i32, bool) {
        (self.zweier, self.zweierb)
    }
    pub fn set_zweier(&mut self, zahl :i32) -> bool {
        if !self.get_zweier().1{
            self.zweier = zahl;
            self.zweierb = true;
            true
        } else {
            false
        }
    }
    pub fn get_dreier(&self) -> (i32, bool) {
        (self.dreier, self.dreierb)
    }
    pub fn set_dreier(&mut self, zahl :i32) -> bool {
        if !self.get_dreier().1{
            self.dreier = zahl;
            self.dreierb = true;
            true
        } else {
            false
        }
    }
    pub fn get_vierer(&self) -> (i32, bool) {
        (self.vierer, self.viererb)
    }
    pub fn set_vierer(&mut self, zahl :i32) -> bool {
        if !self.get_vierer().1{
            self.vierer = zahl;
            self.viererb = true;
            true
        } else {
            false
        }
    }
    pub fn get_funfer(&self) -> (i32, bool){
        (self.funfer, self.funferb)
    }
    pub fn set_funfer(&mut self, zahl :i32) -> bool {
        if !self.get_funfer().1{
            self.funfer = zahl;
            self.funferb = true;
            true
        } else {
            false
        }
    }
    pub fn get_sechser(&self) -> (i32, bool) {
        (self.sechser, self.sechserb)
    }
    pub fn set_sechser(&mut self, zahl :i32) -> bool {
        if !self.get_sechser().1{
            self.sechser = zahl;
            self.sechserb = true;
            true
        } else {
            false
        }
    }
    pub fn get_oberteil(&self) -> i32 {
        self.einer + self.zweier + self.dreier + self.vierer + self.funfer + self.sechser
    }
    pub fn get_bonus(&self) -> i32 {
        self.bonus
    }
    pub fn set_bonus(&mut self) {
        if self.get_oberteil() >= 63 {
            self.bonus = 35;
        }
    }
    pub fn get_oberteil_gesamt(&self) -> i32 {
        self.get_oberteil() + self.get_bonus()
    }
    pub fn get_dreierp(&self) -> (i32, bool) {
        (self.dreierp, self.dreierpb)
    }
    pub fn set_dreierp(&mut self, zahl :i32) -> bool{
        if !self.get_dreierp().1{
            self.dreierp = zahl;
            self.dreierpb = true;
            true
        } else {
            false
        }
    }
    pub fn get_viererp(&self) -> (i32, bool) {
        (self.viererp, self.viererpb)
    }
    pub fn set_viererp(&mut self, zahl :i32) -> bool{
        if !self.get_viererp().1{
            self.viererp = zahl;
            self.viererpb = true;
            true
        } else {
            false
        }
    }
    pub fn get_full_house(&self) -> (i32, bool) {
        (self.full_house, self.full_houseb)
    }
    pub fn set_full_house(&mut self, nutz :bool) -> bool{
        if !self.get_full_house().1{
            if nutz {
                self.full_house = 25;
            } else {
                self.full_house = 0;
            }
            self.full_houseb = true;
            true
        } else {
            false
        }
    }
    pub fn get_kleine_str(&self) -> (i32, bool) {
        (self.kleine_str, self.kleine_strb)
    }
    pub fn set_kleine_str(&mut self, nutz: bool) -> bool {
        if !self.get_kleine_str().1{
            if nutz {
                self.kleine_str = 30;
            } else {
                self.kleine_str = 0;
            }
            self.kleine_strb = true;
            true
        } else {
            false
        }
    }
    pub fn get_grosse_str(&self) -> (i32, bool) {
        (self.grosse_str, self.grosse_strb)
    }
    pub fn set_grosse_str(&mut self, nutz :bool) -> bool {
        if !self.get_grosse_str().1{
            if nutz {
                self.grosse_str = 40;
            } else {
                self.grosse_str = 0;
            }
            self.grosse_strb = true;
            true
        } else {
            false
        }
    }
    pub fn get_kniffel(&self) -> (i32, bool) {
        (self.kniffel, self.kniffelb)
    }
    pub fn set_kniffel(&mut self, nutz :bool) -> bool {
        if !self.get_kniffel().1{
            if nutz {
                self.kniffel = 50;
            } else {
                self.kniffel = 0;
            }
            self.kniffelb = true;
            true
        } else {
            false
        }
    }
    pub fn get_chance(&self) -> (i32, bool) {
        (self.chance, self.chanceb)
    }
    pub fn set_chance(&mut self, zahl :i32) -> bool {
        if !self.get_chance().1{
            self.chance = zahl;
            self.chanceb = true;
            true
        } else {
            false
        }
    }
    pub fn get_unterteil(&self) -> i32 {
        self.dreierp + self.viererp + self.full_house + self.kleine_str + self.grosse_str + self.kniffel + self.chance
    }
    pub fn get_gesamt(&mut self) -> i32 {
        self.get_oberteil_gesamt() + self.get_unterteil()
    }
    pub fn print(&mut self) {
        println!("Scoreboard:");
        if self.get_einer().1{
            println!("\tEiner:\t\t{}\tBenutzt", self.einer);
        } else {
            println!("\t(1) Einer:\t{}", self.einer);
        }
        if self.get_zweier().1{
            println!("\tZweier:\t\t{}\tBenutzt", self.zweier);
        } else {
            println!("\t(2) Zweier:\t{}", self.zweier);
        }
        if self.get_dreier().1{
            println!("\tDreier:\t\t{}\tBenutzt", self.dreier);
        } else {
            println!("\t(3) Dreier:\t{}", self.dreier);
        }
        if self.get_vierer().1{
            println!("\tVierer:\t\t{}\tBenutzt", self.vierer);
        } else {
            println!("\t(4) Vierer:\t{}", self.vierer);
        }
        if self.get_funfer().1{
            println!("\tFünfer:\t\t{}\tBenutzt", self.funfer);
        } else {
            println!("\t(5) Fünfer:\t{}", self.funfer);
        }
        if self.get_sechser().1{
            println!("\tSechser:\t{}\tBenutzt", self.sechser);
        } else {
            println!("\t(6) Sechser:\t{}", self.sechser);
        }
        println!("\tOberer Teil:\t{}\n\tBonus:\t\t{}\n\tMit Bonus:\t{}\n", self.get_oberteil(), self.bonus, self.get_oberteil_gesamt());
        if self.get_dreierp().1{
            println!("\t3erpasch:\t{}\tBenutzt", self.dreierp);
        } else {
            println!("\t(7) 3erpasch:\t{}", self.dreierp);
        }
        if self.get_viererp().1{
            println!("\t4erpasch:\t{}\tBenutzt", self.viererp);
        } else {
            println!("\t(8) 4erpasch:\t{}", self.viererp);
        }
        if self.get_full_house().1{
            println!("\tFull House:\t{}\tBenutzt", self.full_house);
        } else {
            println!("\t(9) Full House:\t{}", self.full_house);
        }
        if self.get_kleine_str().1{
            println!("\tKl. Str.:\t{}\tBenutzt", self.kleine_str);
        } else {
            println!("\t(10) Kl. Str.:\t{}", self.kleine_str);
        }
        if self.get_grosse_str().1{
            println!("\tGr. Str.:\t{}\tBenutzt", self.grosse_str);
        } else {
            println!("\t(11) Gr. Str:\t{}", self.grosse_str);
        }
        if self.get_kniffel().1{
            println!("\tKniffel:\t{}\tBenutzt", self.kniffel);
        } else {
            println!("\t(12) Kniffel:\t{}", self.kniffel);
        }
        if self.get_chance().1{
            println!("\tChance:\t\t{}\tBenutzt", self.chance);
        } else {
            println!("\t(13) Chance:\t{}", self.chance);
        }
        println!("\tUnterer Teil:\t{}\n\tEndsumme:\t{}\n", self.get_unterteil(), self.get_gesamt());
    }

    pub fn fertig(&self) -> bool {
        let mut fertig = false;
        if self.einerb && self.zweierb && self.dreierb && self.viererb && self.funferb && self.sechserb && self.dreierpb && self.viererpb && self.full_houseb && self.kleine_strb && self.grosse_strb && self.kniffelb  && self.chanceb {
            fertig = true;
        }
        fertig
    }
}