use rand::distributions::{Distribution, Uniform};
pub struct Wurfel {
    zahl: [i32; 5],
    inuse: [bool; 5]
}

impl Wurfel{
    fn sort(array: &mut [i32; 5]) {
        let start = 0;
        let end = array.len() - 1;
        Self::quick_sort_partition(array, start, end as isize);
    }
      
    fn quick_sort_partition(array: &mut [i32], start: isize, end: isize) {
        if start < end && end - start >= 1 {
            let pivot = Self::partition(array, start as isize, end as isize);
            Self::quick_sort_partition(array, start, pivot - 1);
            Self::quick_sort_partition(array, pivot + 1, end);
        }
    }
      
    fn partition(array: &mut [i32], l: isize, h: isize) -> isize {
        let pivot = array[h as usize];
        let mut i = l - 1; // Index of the smaller element
      
        for j in l..h {
          if array[j as usize] <= pivot {
            i = i + 1;
            array.swap(i as usize, j as usize);
          }
        }
      
        array.swap((i + 1) as usize, h as usize);
      
        i + 1
    }


    pub fn wurfel_builder() -> Wurfel {
        Wurfel { zahl: [1, 1, 1, 1, 1], inuse: [true, true, true, true, true] }
    }

    pub fn get_zahlen(&self) -> [i32; 5] {
        self.zahl
    }

    /* pub fn get_zahl(&self, index: usize) -> i32 {
        self.zahl[index]
    } */

    pub fn get_zahl(&self, zahl :i32) -> i32 {
        let mut gesamt = 0;
        for x in self.zahl {
            if x == zahl {
                gesamt += zahl;
            }
        }
        gesamt
    }

    pub fn get_gesamt(&self) -> i32 {
        let mut gesamt = 0;
        for x in self.zahl {
            gesamt += x;
        }
        gesamt
    }

    pub fn get_inuse(&self, index: usize) -> bool {
        self.inuse[index]
    }

    pub fn wurf(&mut self) {
        let mut rng = rand::thread_rng();
        let die = Uniform::from(1..=6);
        let mut ind = 0;
        for i in self.inuse {
            if i { 
            self.zahl[ind] = die.sample(&mut rng);
            }
            ind += 1;
        }
    }

    pub fn print(&self) {
        let mut ind = 0;
        println!("Gewürfelt: ");
        print!("In Benutzung: ");
        for i in self.inuse {
            if i {
                print!("\t{}", self.zahl[ind]);
            } else {
                print!("\t");
            }
            ind += 1;
        }
        print!("\nBehalten: ");
        ind = 0;
        for i in self.inuse {
            if !i {
                print!("\t{}", self.zahl[ind]);
            } else {
                print!("\t");
            }
            ind += 1;
        }
        println!("\n");
    }

    pub fn dreierp(&self) -> bool {
        let mut zahlen = self.zahl.clone();
        Self::sort(&mut zahlen);
        let mut dreier = false;
        if (zahlen[0] == zahlen[1] && zahlen[1] == zahlen[2]) ||
            (zahlen[1] == zahlen[2] && zahlen[2] == zahlen[3]) ||
            (zahlen[2] == zahlen[3] && zahlen[3] == zahlen[4]) {
                dreier = true;
        }
        dreier
    }

    pub fn viererp(&self) -> bool {
        let mut zahlen = self.zahl.clone();
        Self::sort(&mut zahlen);
        let mut vierer = false;
        if (zahlen[0] == zahlen[1] && zahlen[1] == zahlen[2] && zahlen[2] == zahlen[3]) ||
            (zahlen[1] == zahlen[2] && zahlen[2] == zahlen[3] && zahlen[3] == zahlen[4] ) {
                vierer = true;
        }
        vierer
    }

    pub fn full_house(&self) -> bool {
        let mut zahlen = self.zahl.clone();
        Self::sort(&mut zahlen);
        let mut fh = false;
        if zahlen[0] == zahlen[1] && (zahlen[1] == zahlen[2] || zahlen[2] == zahlen[3]) && zahlen[3] == zahlen[4] {
            fh = true;
        }
        fh
    }

    pub fn kleine_str(&self) -> bool {
        let mut zahl = self.zahl.clone();
        Self::sort(&mut zahl);
        let mut ks = false;
        if (zahl[0]+1 == zahl[1] && zahl[1]+1 == zahl[2] && zahl[2]+1 == zahl[3]) ||
            (zahl[1]+1 == zahl[2] && zahl[2]+1 == zahl[3] && zahl[3]+1 == zahl[4]) || 
            (zahl[0]+1 == zahl[2] && zahl[2]+1 == zahl[3] && zahl[3]+1 == zahl[4]) ||
            (zahl[0]+1 == zahl[1] && zahl[1]+1 == zahl[3] && zahl[3]+1 == zahl[4]) ||
            (zahl[0]+1 == zahl[1] && zahl[1]+1 == zahl[2] && zahl[2]+1 == zahl[4]){
                ks = true;
        } 
        ks
    }

    pub fn grosse_str(&self) -> bool {
        let mut zahl = self.zahl.clone();
        Self::sort(&mut zahl);
        let mut gs = false;
        if zahl[0]+1 == zahl[1] && zahl[1]+1 == zahl[2] && zahl[2]+1 == zahl[3] && zahl[3]+1 == zahl[4] {
            gs = true;
        }
        gs
    }

    pub fn kniffel(&self) -> bool {
        let mut kniffel = false;
        if self.zahl[0] == self.zahl[1] && self.zahl[1] == self.zahl[2] && self.zahl[2] == self.zahl[3] && self.zahl[3] == self.zahl[4] {
            kniffel = true;
        }
        kniffel
    }

    pub fn weglegen(&mut self, vec :Vec<u32>) {
        for x in vec {
            match x {
                1 => self.inuse[0] = false,
                2 => self.inuse[1] = false,
                3 => self.inuse[2] = false,
                4 => self.inuse[3] = false,
                5 => self.inuse[4] = false,
                _ => (),
            }
        }
    }
}