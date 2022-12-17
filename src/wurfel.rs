use rand::distributions::{Distribution, Uniform};
use std::collections::HashSet;

/// Würfelklasse
pub struct Wurfel {
    /// Die Zahlen welche gewürfelt werden
    zahl: [i32; 5],
    /// Ob die Würfel in benutzung sind
    inuse: [bool; 5]
}

impl Wurfel{

    /// Quicksort Algorythmus, welcher aus dem Internet geklaut wurde 🙃
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


    /// Konstruktor, welcher ein Standard Würfelobjekt ausgibt mit 5 Einsen, und allen Würfeln in benutzung
    /// 
    /// # Examples
    /// 
    /// ```
    /// use wurfel::Wurfel;
    /// let wurfel = Wurfel::Wurfel_builder();
    /// ```
    pub fn wurfel_builder() -> Wurfel {
        Wurfel { zahl: [1, 1, 1, 1, 1], inuse: [true, true, true, true, true] }
    }

    /// Getter des Zahlen Arrays
    pub fn get_zahlen(&self) -> [i32; 5] {
        self.zahl
    }

    /* pub fn get_zahl(&self, index: usize) -> i32 {
        self.zahl[index]
    } */

    /// Wie oft wurde eine bestimmte Zahl gewürfelt. Gibt diese anzahl aus.
    /// 
    /// # Argument
    /// `zahl` - Auf welche Zahl geprüft werden soll
    pub fn get_zahl(&self, zahl :i32) -> i32 {
        let mut gesamt = 0;
        for x in self.zahl {
            if x == zahl {
                gesamt += zahl;
            }
        }
        gesamt
    }

    /// Gibt die Gesamtaugenzahl der Würfel aus
    pub fn get_gesamt(&self) -> i32 {
        let mut gesamt = 0;
        for x in self.zahl {
            gesamt += x;
        }
        gesamt
    }

    /// Prüft ob ein bestimmter Würfel in benutzung ist.
    /// 
    /// # Argument
    /// `index` - Welcher Würfel geprüft werden soll
    pub fn get_inuse(&self, index: usize) -> bool {
        self.inuse[index]
    }

    /// Nutzt die Crate [`rand`] um den Würfeln, welche in benutzung sind,
    /// eine zufällige Zahl zwischen 1 und 6 inkl. zuzuweisen.
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

    /// Gibt die Würfel schön in der Konsole aus
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

    /// Prüft ob ein Dreierpasch existiert
    pub fn dreierp(&self) -> bool {
        let mut zahlen = self.zahl.clone();
        Self::sort(&mut zahlen);
        let mut dreier = false;
        for i in 0..=3 {
            if zahlen[i] == zahlen[i+1] && zahlen[i] == zahlen[i+2] {
                dreier = true;
            } 
        }
        dreier
    }

    /// Prüft ob ein Viererpasch existiert
    pub fn viererp(&self) -> bool {
        let mut zahlen = self.zahl.clone();
        Self::sort(&mut zahlen);
        let mut vierer = false;
        for i in 0..=2 {
            if zahlen[i] == zahlen[i+1] && zahlen[i] == zahlen[i+2] && zahlen[i] == zahlen[i+3] {
                vierer = true
            }
        }
        vierer
    }

    /// Prüft ob ein Full-House exisitiert
    pub fn full_house(&self) -> bool {
        let mut zahlen = self.zahl.clone();
        Self::sort(&mut zahlen);
        let mut fh = false;
        if zahlen[0] == zahlen[1] && (zahlen[1] == zahlen[2] || zahlen[2] == zahlen[3]) && zahlen[3] == zahlen[4] {
            fh = true;
        }
        fh
    }

    /// Prüft ob eine kleine Straße existiert
    pub fn kleine_str(&self) -> bool {
        let zahl: HashSet<i32> = self.zahl.iter().cloned().collect(); // Nutzt HashSet um identische Werte zu entfernen
        for i in 1..=3 {
            if zahl.contains(&i) && zahl.contains(&(i + 1)) && zahl.contains(&(i + 2)) && zahl.contains(&(i + 3)) {
                return true;
            }
        }
        false
    }

    /// Prüft ob ein große Straße existiert
    pub fn grosse_str(&self) -> bool {
        let zahl: HashSet<i32> = self.zahl.iter().cloned().collect();
        for i in 1..=2 {
            if zahl.contains(&i) && zahl.contains(&(i + 1)) && zahl.contains(&(i + 2)) && zahl.contains(&(i + 3)) && zahl.contains(&(i + 4)) {
                return true;
            }
        }
        false
    
    }

    /// Prüft ob ein Kniffel existiert
    pub fn kniffel(&self) -> bool {
        let mut kniffel = false;
        if self.zahl[0] == self.zahl[1] && self.zahl[1] == self.zahl[2] && self.zahl[2] == self.zahl[3] && self.zahl[3] == self.zahl[4] {
            kniffel = true;
        }
        kniffel
    }

    /// Legt die ausgewählten Würfel weg, diese werden nicht erneut gewürfelt
    /// 
    /// # Argument
    /// `vec` - Vector der ausgewählten Würfel
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