use crate::score::Score;
use crate::wurfel::Wurfel;
use std::io::{self, Write, Read};

pub mod score;
pub mod wurfel;

fn main() {
    let mut spieler1 = Score::score_builder();
    let mut spieler2 = Score::score_builder();();
    let mut wurfel = Wurfel::wurfel_builder();
    
    let mut dreimal = 3;
    let mut auswahl = 0;
    print!("\x1B[2J");
    while !spieler1.fertig() && !spieler2.fertig() {
        println!("\n");
        println!("Spieler 1:");
        while auswahl != 1 && dreimal > 0 {
            spieler1.print();
            wurfel.wurf();
            wurfel.print();
            let pos = get_pos(&spieler1, &wurfel);
            let mut erfolg = false;

            while erfolg == false{
                if dreimal > 1 {
                    println!("\nWass willst du tun?\n1. Eintragen\n2. Würfel behalten und nochmal würfeln");
                    auswahl = get_i32_input();
                    match auswahl {
                        1 => erfolg = eintragen(&mut spieler1, &wurfel, &pos),
                        2 => {weglegen(&mut wurfel); erfolg = true},
                        _ => (),
                    }
                } else {
                    erfolg = eintragen(&mut spieler1, &wurfel, &pos);
                }
            }
            dreimal -= 1; 
        }
        dreimal = 3;
        auswahl = 0;
        wurfel = Wurfel::wurfel_builder();
        println!("\n");
        println!("Spieler 2:");
        while auswahl != 1 && dreimal > 0 {
            spieler2.print();
            wurfel.wurf();
            wurfel.print();
            let pos = get_pos(&spieler2, &wurfel);
            let mut erfolg = false;

            while erfolg == false {
                if dreimal > 1 {
                    println!("\nWass willst du tun?\n1. Eintragen\n2. Würfel behalten und nochmal würfeln");
                    auswahl = get_i32_input();
                    match auswahl {
                        1 => erfolg = eintragen(&mut spieler2, &wurfel, &pos),
                        2 => {weglegen(&mut wurfel);  erfolg = true;},
                        _ => (),
                    }
                } else {
                    erfolg = eintragen(&mut spieler2, &wurfel, &pos)
                }
            }
            dreimal -= 1; 
        }
        auswahl = 0;
        dreimal = 3;
        wurfel = Wurfel::wurfel_builder();
        
    };

    let diff = spieler1.get_gesamt() - spieler2.get_gesamt();

    println!("Endergebniss:");
    println!("Spieler 1:");
    spieler1.print();
    println!("Spieler 2:");
    spieler2.print();

    if diff > 0 {
        println!("Spieler 1 hat gewonnen");
    } else if diff < 0 {
        println!("Spieler 2 hat gewonnen");
    } else {
        println!("Unentschieden")
    }
    pause();

}

fn weglegen(wurfel: &mut Wurfel){
    println!("Welche Würfel willst du behalten.\n(z.B.: \"123\" für die ersten drei Würfel,\nmit Würfel 4 und 5 wird weiter gewürfelt)");
    let mut vec :Vec<u32> = Vec::new();
    let mut eingabe = String::new();
    match std::io::stdin().read_line(&mut eingabe) {      //get input
        Ok(_n) => (),
        Err(error) => {
            eprint!("Failed to read line: {error}");
            return;
        }
    }
    for c in eingabe.chars() {
        if c.is_numeric() {
            match c.to_digit(10) {
                None => (),
                Some(n) => vec.push(n),
            }
        }
    }
    wurfel.weglegen(vec);

}

fn eintragen(spieler : &mut Score, wurfel :&Wurfel, pos :&Vec<i32>) -> bool{
    let mut erfolg = false;
    println!("Was willst du eintragen?");
    let eingabe = get_i32_input();
    if pos.contains(&eingabe){
        match eingabe {
            1 => erfolg = spieler.set_einer(wurfel.get_zahl(1)),
            2 => erfolg = spieler.set_zweier(wurfel.get_zahl(2)),
            3 => erfolg = spieler.set_dreier(wurfel.get_zahl(3)),
            4 => erfolg = spieler.set_vierer(wurfel.get_zahl(4)),
            5 => erfolg = spieler.set_funfer(wurfel.get_zahl(5)),
            6 => erfolg = spieler.set_sechser(wurfel.get_zahl(6)),
            7 => erfolg = spieler.set_dreierp(wurfel.get_gesamt()),
            8 => erfolg = spieler.set_viererp(wurfel.get_gesamt()),
            9 => erfolg = spieler.set_full_house(true),
            10 => erfolg = spieler.set_kleine_str(true),
            11 => erfolg = spieler.set_grosse_str(true),
            12 => erfolg = spieler.set_kniffel(true),
            13 => erfolg = spieler.set_chance(wurfel.get_gesamt()),
            _ => (),
        }
    } else {
        match eingabe {
            1 => erfolg = spieler.set_einer(0),
            2 => erfolg = spieler.set_zweier(0),
            3 => erfolg = spieler.set_dreier(0),
            4 => erfolg = spieler.set_vierer(0),
            5 => erfolg = spieler.set_funfer(0),
            6 => erfolg = spieler.set_sechser(0),
            7 => erfolg = spieler.set_dreierp(0),
            8 => erfolg = spieler.set_viererp(0),
            9 => erfolg = spieler.set_full_house(false),
            10 => erfolg = spieler.set_kleine_str(false),
            11 => erfolg = spieler.set_grosse_str(false),
            12 => erfolg = spieler.set_kniffel(false),
            13 => erfolg = spieler.set_chance(0),
            _ => (),
        }
    }
    spieler.set_bonus();
    erfolg
}

fn get_i32_input() -> i32 {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {      //get input
        Ok(_n) => (),
        Err(error) => {
            eprint!("Failed to read line: {error}");
            return 0;
        }
    }
    if line.ends_with("\n") {                               //Truncate input
        line.pop();
        if line.ends_with("\r") {
            line.pop();
        }
    }
    match line.parse::<i32>() {                             //parse input to i32
        Ok(n) => 
            n,
        Err(_err) => {
            0
        }
    }
}

fn get_pos(spieler :&Score, wurfel :&Wurfel) -> Vec<i32>{
    let mut pos = Vec::new();
    println!("Empfohlen:");
    if spieler.get_einer().1 == false && wurfel.get_zahlen().contains(&1) {
        println!("1. Einer");
        pos.push(1);
    }
    if spieler.get_zweier().1 == false && wurfel.get_zahlen().contains(&2) {
        println!("2. Zweier");
        pos.push(2);
    }
    if spieler.get_dreier().1 == false && wurfel.get_zahlen().contains(&3) {
        println!("3. Dreier");
        pos.push(3);
    }
    if spieler.get_vierer().1 == false && wurfel.get_zahlen().contains(&4) {
        println!("4. Vierer");
        pos.push(4);
    }
    if spieler.get_funfer().1 == false && wurfel.get_zahlen().contains(&5) {
        println!("5. Fünfer");
        pos.push(5);
    }
    if spieler.get_sechser().1 == false && wurfel.get_zahlen().contains(&6) {
        println!("6. Sechser");
        pos.push(6);
    }
    if spieler.get_dreierp().1 == false && wurfel.dreierp() {
        println!("7. Dreierpasch");
        pos.push(7);
    }
    if spieler.get_viererp().1 == false && wurfel.viererp() {
        println!("8. Viererpasch");
        pos.push(8);
    }
    if spieler.get_full_house().1 == false && wurfel.full_house() {
        println!("9. Full House");
        pos.push(9);
    }
    if spieler.get_kleine_str().1 == false && wurfel.kleine_str() {
        println!("10. Kleine Straße");
        pos.push(10);
    }
    if spieler.get_grosse_str().1 == false && wurfel.grosse_str() {
        println!("11. Große Straße");
        pos.push(11);    
    }
    if spieler.get_kniffel().1 == false && wurfel.kniffel() {
        println!("12. Kniffel");
        pos.push(12);
    }
    if spieler.get_chance().1 == false{
        println!("13. Chance");
        pos.push(13);
    }
    pos
}


fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Drücke Enter um zu beenden...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}