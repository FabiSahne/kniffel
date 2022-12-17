use crate::score::Score;
use crate::wurfel::Wurfel;
use std::io::{self, Write, Read};

pub mod score;
pub mod wurfel;

fn main() {
    //Abfrage wie viele Spieler spielen wollen
    println!("Wie viele Spieler?");
    let anzahl: usize = string_to_i32(get_input()) as usize; //Anzahl der spieler (als usize, da als Vectorgröße benutzt)



    let mut spieler: Vec<Score> = vec![Score::score_builder(); anzahl]; //Vector von Spielerobjecten der Klasse Score
    let mut wurfel = Wurfel::wurfel_builder();  //Würfelobjekt der Klasse Würfel
    
    let mut dreimal = 3; //Der Spieler darf drei Mal würfeln, daher diese Zählvariable (hätte wohl auch als for-loop funktioniert)
    let mut auswahl = 0; //Variable was der Spieler ausgewählt hat zu machen
    print!("\x1B[2J"); //Konsole leeren
    loop { //Da maximal 13 Runden auch als Zählschleife möglich
        for i in 0..anzahl { //Jeder Spieler einmal
            println!("\n");
            println!("Spieler {}:", i+1);
            while auswahl != 1 && dreimal > 0 { //Solange noch nicht Eingetragen ist (auswahl != 1) und wir noch nicht 3 Mal gewürfelt haben (dreimal > 0)
                spieler[i].print(); //Scoreboard ausgeben
                wurfel.wurf(); //Würfeln
                wurfel.print(); //Würfel ausgeben
                let pos = get_pos(&spieler[i], &wurfel); //Möglichkeiten/Empfehlungen ausgeben
                let mut erfolg = false; //Prüfe ob erfolgreich ausgewählt/eingetragen wurde mit folgender while-Schleife
    
                while erfolg == false{
                    if dreimal > 1 { //Beim 1. und 2. Würfeln können Würfel weggelegt werden, deshalb muss danach gefragt werden, was getan werden soll
                        println!("\nWass willst du tun?\n1. Eintragen\n2. Würfel behalten und nochmal würfeln"); //Frag was gemacht werden soll
                        auswahl = string_to_i32(get_input()); //Auswahl abfragen
                        match auswahl {
                            1 => erfolg = eintragen(&mut spieler[i], &wurfel, &pos), //ins Scoreboard eintragen
                            2 => {weglegen(&mut wurfel); erfolg = true}, //Würfel weglegen
                            _ => (), //Falls eingabe nicht 1 oder 2 war, mach nichts, die Abfrage wird wiederholt
                        }
                    } else { //Beim 3. Würfeln muss eingetragen werden
                        erfolg = eintragen(&mut spieler[i], &wurfel, &pos);
                    }
                }
                dreimal -= 1; //Zählvariable erniedrigen
            }
            dreimal = 3; //Zählvariable zurücksetzen
            auswahl = 0; //Auswahl zurücksetzen
            wurfel = Wurfel::wurfel_builder(); //Würfel zurücksetzen
        }

        //Prüfen ob alle Spieler fertig sind
        let mut fertig = true; //Wir gehen davon aus, dass wir fertig sind
        for i in 0..anzahl { //Für anzahl an Spielern
            if spieler[i].fertig() == false { //Falls wir noch nicht fertig sind
                fertig = false; //setze die Variable auf false
            }
        }
        if fertig { //Falls wir tatsächlich fertig sind
            break; //Breche den loop ab
        }
        
    };

    //Das Endergebniss ausgeben
    println!("Endergebniss:");

    //Gibt die Gesamtpunktzahl von jedem Spieler aus
    for i in 0..anzahl {
        println!("Spieler {}: {}", i+1, spieler[i].get_gesamt());
    }

    //Damit die Konsole nicht direkt Schließt, warte auf den User
    pause();

}


/// Userinput entscheidet welche Würfel weggelegt/behalten werden.
/// Die Ausgewählten Würfel werden nicht erneut gewürfelt.
/// 
/// Dafür wird das Attribut inuse der Würfel Klasse benutzt und auf false gesetzt mit der Methode wurfel.weglegen(vec: Vec<i32>)
/// 
/// # Argument
/// `wurfel` - Das Würfelobjekt
fn weglegen(wurfel: &mut Wurfel){
    println!("Welche Würfel willst du behalten.\n(z.B.: \"123\" für die ersten drei Würfel,\nmit Würfel 4 und 5 wird weiter gewürfelt)");
    let mut vec :Vec<u32> = Vec::new(); // vec enthält die Ziffern als Vector elemente
    let mut eingabe = String::new(); // eingabe enthält die Ziffern als String
    match std::io::stdin().read_line(&mut eingabe) {      // Eingabe und Fehlerbehandlung
        Ok(_n) => (),
        Err(error) => {
            eprint!("Failed to read line: {error}");
            return;
        }
    }
    // Die Ziffern im String in den Vec vec packen
    for c in eingabe.chars() {
        if c.is_numeric() {
            match c.to_digit(10) {
                None => (),
                Some(n) => vec.push(n),
            }
        }
    }
    // Würfel weglegen
    wurfel.weglegen(vec);

}


/// Userinput entscheidet welcher Score eingetragen wird.
/// 
/// Dazu wird überprüft, ob dieser Score überhaupt Punkte gibt, oder nicht.
/// Anschließend wird versucht der ausgewählte Score zu setzen, fall dieser schon benutzt wurde, wird dieser false zurückgeben.
/// Zuletzt wird der Bonus aktualisiert
/// 
/// # Argumente
/// * `spieler` - Der Spieler der im Moment spielt
/// * `wurfel` - Das Würfelobjekt
/// * `pos` - Die Möglichkeiten, die auch Punkte geben würden
fn eintragen(spieler : &mut Score, wurfel :&Wurfel, pos :&Vec<i32>) -> bool{
    let mut erfolg = false; // Prüfvariable, ob das Eintragen ein Erfolg war
    println!("Was willst du eintragen?");
    let eingabe = string_to_i32(get_input()); //Userabfrage
    if pos.contains(&eingabe){  //Falls die Eingabe punkte gibt, trage die jeweiligen Punkte ein
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
    } else { //Falls nicht, trage 0 Punkte ein
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

    // Aktualisiere den Bonus
    spieler.set_bonus();
    // Gebe aus ob es ein Erfolg war
    erfolg
}


/// Holt sich Userinput über `io::stdin().readline(buf)`.
/// Entfernt den Zeilenumbruch am Ende und gibt den Input als `String` aus.
fn get_input() -> String {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) { //Userinput in Buffer -> line
        Ok(_n) => (),
        Err(error) => {
            eprint!("Failed to read line: {error}");
            return "".to_string();
        }
    }
    if line.ends_with("\n") { //Zeilenumbruch entfernen mit pop()
        line.pop();
        if line.ends_with("\r") {
            line.pop();
        }
    }
    line //String ausgeben
}

/// `String` zu `i32` wandeln
/// 
/// # Argument
/// `line` - String, der nur eine Zahl enthält
fn string_to_i32(line: String) -> i32 {
    match line.parse::<i32>() { //line zu `i32` parsen, bei Fehler, gebe 0 aus.
        Ok(n) => n,
        Err(_err) => 0
    }
}

/// Geht das Scoreboard durch, und prüft ob der Jeweilige Score mit unseren aktuellen Würfeln punkte gibt,
/// und gibt diese als Vector aus.
/// 
/// # Argumente
/// * `spieler` - Der Spieler der im Moment spielt
/// * `wurfel` - Das Würfelobjekt
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

/// Wartet auf Userinput
fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    write!(stdout, "Drücke Enter um zu beenden...").unwrap();
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}