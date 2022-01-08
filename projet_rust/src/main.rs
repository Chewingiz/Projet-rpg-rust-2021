//manal m'a découvrir cette lib pour les actions d'entré clavier
use crossterm::event::{self, Event, KeyCode};

use std::io::{self, Write};
use termion::{clear, raw::IntoRawMode};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::*,
    widgets::{BarChart, Block, Borders, Paragraph},
    *,
};

use rand::Rng;
use std::fs::File;
#[derive(Serialize, Deserialize, Debug)]
struct Perso {
    // la structure de votre héro chaque paramètre influencera les actions possibles
    nom: String,
    charisme: u64,
    force: u64,
    intelligence: u64,
    current_his: usize, // grace a ce fameux LukeTPeterson, qui m'a aider à regler mon problème indexed
}

use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Result;
//use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, PartialEq, Debug)]
struct Current {
    description: String,

    choix_1: String,
    type1: String,
    n_1_reussit: usize,
    n_1_echec: usize,

    choix_2: String,
    type2: String,
    n_2_reussit: usize,
    n_2_echec: usize,

    choix_3: String,
    type3: String,
    n_3_reussit: usize,
    n_3_echec: usize,

    //mort:Some(false)
    mort: bool,
}
use serde_json::from_str;
use serde_yaml::Value;
use std::fs;
use yaml_rust::YamlEmitter;
use yaml_rust::YamlLoader;
const GAIN: u64 = 10;
//grace à la lib from https://crates.io/crates/serde_yaml
//et https://stackoverflow.com/questions/68404178/how-to-deserialize-this-yaml-in-struct-with-serde-yaml
fn typed_example(data: &str) -> serde_yaml::Result<Current> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    //let p: Person = serde_json::from_str(data)?;
    let c: Current = serde_yaml::from_str(data)?;
    // Do things just like with any other Rust data structure.
    //println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(c)
}

fn main() -> io::Result<()> {
    let histoire = fs::read_to_string("hist2").expect("Something went wrong reading the file");
    let mut rng = rand::thread_rng();
    let mut perso = Perso {
        // la structure de héro elle est pour l'instant static
        nom: "Pachat".to_string(),
        charisme: 50,
        force: 10,
        intelligence: 10,
        current_his: 1,
    };

    let docs = YamlLoader::load_from_str(&histoire).unwrap();

    loop {
        // boucle de notre jeux qui s'arrete si on attein un end game ou si on appui sur 'q'
        //https://crates.io/crates/yaml-rust: m'a permit de comprendre comment marche la Yaml
        let doc = &docs[0][perso.current_his];
        let mut out_str = String::new();
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String
        let current: Current = typed_example(&out_str).unwrap();

        let mut stdout = io::stdout().into_raw_mode()?;
        write!(stdout, "{}", clear::All)?;
        let backend = TermionBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?; // on crée donc ce qui nous sera l'interface ou l'utilisateur pourra interagir dans le terminal
        terminal.draw(|mut f| {
            let _test = BarChart::default()
                .block(Block::default().title("BarChart").borders(Borders::ALL)) // on précise que tous sera entouré par une bordure
                .bar_width(1) // d'une épaiseur de 1
                .bar_gap(5) // enfin elle seront toutes une distances des uns des autres de 5 (les diférentes case)
                .style(Style::default().fg(Color::Yellow).bg(Color::Red))
                .value_style(Style::default().fg(Color::Red).modifier(Modifier::BOLD))
                .label_style(Style::default().fg(Color::White)); // la couleur de la bordure sera blanche
            let chunks = Layout::default() // on crée les différents case de notre interface sur le terminal
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    //les differrents ecart entre chaque cases qu'on créera
                    [Constraint::Percentage(30), Constraint::Percentage(50)].as_ref(),
                )
                .split(f.size());
            let left_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    //les differrents ecart entre chaque cases qu'on créera
                    [Constraint::Percentage(50), Constraint::Percentage(50)].as_ref(),
                )
                .split(chunks[0]);

            let option = [
                // l'affichage des différent static que vous possédés
                Text::raw("q: quitter\ns: sauvegarder\nc: charger votre sauvegarde"),
                Text::styled("", Style::default()),
            ];
            let help = "Pseudo: ".to_owned()
                + &perso.nom
                + "\n"
                + "Charisme: "
                + &perso.charisme.to_string()
                + "\n"
                + "Force: "
                + &perso.force.to_string()
                + "\n"
                + "Intelligence: "
                + &perso.intelligence.to_string()
                + "\n";
            let stat = [Text::raw(help), Text::styled("", Style::default())];
            let upper_left_pane = Paragraph::new(option.iter())
                .block(Block::default().title("Option").borders(Borders::ALL));
            let lower_left_pane = Paragraph::new(stat.iter())
                .block(Block::default().title("Stat").borders(Borders::ALL));
            /* let right_pane =
            Block::default().title("Histroire").borders(Borders::ALL);
            */

            let mut page = "   ".to_owned() + &current.description + "\n" + "\n";

            if current.mort == false {
                page = page
                    + "[1]: "
                    + &current.choix_1
                    + "["
                    + &current.type1
                    + "]"
                    + "\n"
                    + "[2]: "
                    + &current.choix_2
                    + "["
                    + &current.type2
                    + "]"
                    + "\n"
                    + "[3]: "
                    + &current.choix_3
                    + "["
                    + &current.type3
                    + "]"
                    + "\n";
            }
            let text = [Text::raw(page), Text::styled("", Style::default())];
            //let right_pane =
            let right_pane = Paragraph::new(text.iter())
                .block(Block::default().title("Histroire").borders(Borders::ALL))
                .scroll(0) // on donne un titer a la case et on instancie le scroll(a modifier)
                .wrap(true);

            f.render_widget(upper_left_pane, left_chunks[0]); // on rajoute les differentes cases a notre interfaces
            f.render_widget(lower_left_pane, left_chunks[1]);
            f.render_widget(right_pane, chunks[1]);
        })?;
        // permet de gérer les type de de d'évenement du clavier
        // grace à Florian Dehau sur github dans https://github.com/fdehau/tui-rs/blob/master/examples/barchart.rs :
        //j'ai pu comprendre que je pouvais faire comme cela pour les différents entrées claviers
        //et nom juste attendre que l'utilsateur tape et appuie sur entrée comme dans: https://www.tutorialspoint.com/rust/rust_input_output.htm
        if let Event::Key(key) = event::read().expect("error") {
            if let KeyCode::Char('q') = key.code {
                // le cas on on appuye sur 'q', permet de quitter la partie sans sauvegarder.
                break;
            }
            if let KeyCode::Char('s') = key.code {
                // le cas on on appuye sur 's', permet de sauvegarder votre progression

                //sauvegarder de la partie actuel
                let mut serialized_state = serde_yaml::to_string(&perso).unwrap();
                let mut file = File::create("test.rs")?;
                file.write_all(serialized_state.as_bytes())?;
            }
            if current.mort == false {
                if let KeyCode::Char('1') = key.code {
                    // le cas on on appuye sur '1', permet de choisir le choix numéro 1.
                    if rng.gen_range(0..100) <= perso.charisme {
                        perso.charisme += GAIN;
                        perso.current_his = current.n_1_reussit;
                    } else {
                        perso.current_his = current.n_1_echec;
                    }
                }
                if let KeyCode::Char('2') = key.code {
                    // le cas on on appuye sur '2', permet de choisir le choix numéro 2.
                    if rng.gen_range(0..100) <= perso.force {
                        perso.force += GAIN;
                        perso.current_his = current.n_2_reussit;
                    } else {
                        perso.current_his = current.n_2_echec;
                    }
                }
                if let KeyCode::Char('3') = key.code {
                    // le cas on on appuye sur '3', permet de choisir le choix numéro 3.
                    if rng.gen_range(0..100) <= perso.intelligence {
                        perso.intelligence += GAIN;
                        perso.current_his = current.n_3_reussit;
                    } else {
                        perso.current_his = current.n_3_echec;
                    }
                }
            }
            if let KeyCode::Char('c') = key.code {
                // le cas on on appuye sur 'c', permet de charger votre ancienne sauvegarder
                let sauvegarde =
                    fs::read_to_string("test.rs").expect("Something went wrong reading the file");
                let docs = YamlLoader::load_from_str(&sauvegarde).unwrap();
                let doc = &docs[0];
                let mut resulta = String::new();
                let mut emitter = YamlEmitter::new(&mut resulta);
                emitter.dump(doc).unwrap();
                let p: Perso = serde_yaml::from_str(&resulta).expect("error");
                perso.nom = p.nom;
                perso.charisme = p.charisme;
                perso.intelligence = p.intelligence;
                perso.force = p.force;
                perso.current_his = p.current_his;
            }
        }

        print!("\r\n");
    }
    Ok(())
} //perso."intellignece"

/*
// traducteur pour verifier: https://www.json2yaml.com/
Json te  YAML
{
     "description": "Ceci correspond à l'histoire.",
     "choix_1": "Ceci correspond au choix 1.",
     " N_1":1,
     "choix_2": "Ceci correspond au choix 2.",
     "N_2":2,
     "choix_3": "Ceci correspond au choix 3.",
     " N_3":3
}

Yaml
description: Ceci correspond à l'histoire.
choix_1: Ceci correspond au choix 1.
" N_1": 1
choix_2: Ceci correspond au choix 2.
N_2: 2
choix_3: Ceci correspond au choix 3.
" N_3": 3


*/
/* let doc = &docs[0][perso.current_his];
    let mut out_str = String::new();
    let mut emitter = YamlEmitter::new(&mut out_str);
    emitter.dump(doc).unwrap(); // dump the YAML object to a String;
    let current:Current=typed_example(&out_str).unwrap();
    //let myhson: serde_json::Result<Person> = serde_json::from_str(stack_str).unwrap().expect("Something went wrong reading the file");
    //println!("{:?}", myhson);
*/
//: Result<Person , serde_json::Error> // : serde_json::Result<Person>

//affichage(perso,current);

/* let doc = &docs[0][perso.current_his];
    let mut out_str = String::new();
    let mut emitter = YamlEmitter::new(&mut out_str);
    emitter.dump(doc).unwrap(); // dump the YAML object to a String;
    let current:Current=typed_example(&out_str).unwrap();
    //let myhson: serde_json::Result<Person> = serde_json::from_str(stack_str).unwrap().expect("Something went wrong reading the file");
    //println!("{:?}", myhson);
*/
//: Result<Person , serde_json::Error> // : serde_json::Result<Person>

//affichage(perso,current);

/*fn affichage(perso:Perso, current:Current) -> io::Result<()>  {
    let mut stdout = io::stdout().into_raw_mode()?;
    write!(stdout, "{}", clear::All)?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?; // on crée donc ce qui nous sera l'interface ou l'utilisateur pourra interagir dans le terminal
    terminal.draw(|mut f| {
        let _test = BarChart::default()
            .block(Block::default().title("BarChart").borders(Borders::ALL)) // on précise que tous sera entouré par une bordure
            .bar_width(1) // d'une épaiseur de 1
            .bar_gap(5) // enfin elle seront toutes une distances des uns des autres de 5 (les diférentes case)
            .style(Style::default().fg(Color::Yellow).bg(Color::Red))
            .value_style(Style::default().fg(Color::Red).modifier(Modifier::BOLD))
            .label_style(Style::default().fg(Color::White)); // la couleur de la bordure sera blanche
        let chunks = Layout::default() // on crée les différents case de notre interface sur le terminal
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                //les differrents ecart entre chaque cases qu'on créera
                [Constraint::Percentage(30), Constraint::Percentage(50)].as_ref(),
            )
            .split(f.size());
        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                //les differrents ecart entre chaque cases qu'on créera
                [Constraint::Percentage(50), Constraint::Percentage(50)].as_ref(),
            )
            .split(chunks[0]);

        let option = [
            // l'affichage des différent static que vous possédés
            Text::raw("q: pour quitter\ns: pour sauvegarder"),
            Text::styled("", Style::default()),
        ];
        let help = "Pseudo: ".to_owned()
            + &perso.nom
            + "\n"
            + "Charisme: "
            + &perso.charisme.to_string()
            + "\n"
            + "Force: "
            + &perso.force.to_string()
            + "\n"
            + "Intelligence: "
            + &perso.intellignece.to_string()
            + "\n";


        let stat = [Text::raw(help), Text::styled("", Style::default())];
        let upper_left_pane = Paragraph::new(option.iter())
            .block(Block::default().title("Option").borders(Borders::ALL));
        let lower_left_pane =
            Paragraph::new(stat.iter()).block(Block::default().title("Stat").borders(Borders::ALL));
        /* let right_pane =
           Block::default().title("Histroire").borders(Borders::ALL);
        */

        /*let text = [
            // le texte qui sera vue dans l'interface du terminal
            Text::styled(out_str, Style::default()),
        ];*/

        let page = //"Pseudo: ".to_owned()
        "   ".to_owned()
        + &current.description
        + "\n"
        + "\n"
        + "[1]: "
        + &current.choix_1
        + "\n"
        + "[2]: "
        + &current.choix_2
        + "\n"
        + "[3]: "
        + &current.choix_3
        + "\n";
        let text = [Text::raw(page), Text::styled("", Style::default())];
        //let right_pane =
        let right_pane =Paragraph::new(text.iter())
            .block(Block::default().title("Histroire").borders(Borders::ALL))
            .scroll(0) // on donne un titer a la case et on instancie le scroll(a modifier)
            .wrap(true);

        f.render_widget(upper_left_pane, left_chunks[0]); // on rajoute les differentes cases a notre interfaces
        f.render_widget(lower_left_pane, left_chunks[1]);
        f.render_widget(right_pane, chunks[1]);
    })?;
    print!("\r\n");
    Ok(())
}*/
