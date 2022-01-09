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
use std::fs;
use yaml_rust::YamlEmitter;
use yaml_rust::YamlLoader;

#[derive(Serialize, Deserialize, Debug)]
struct Perso {
    // la structure de votre héro chaque paramètre influencera les actions possibles
    nom: String,
    charisme: u64,
    force: u64,
    intelligence: u64,
    current_his: usize, // grace a ce fameux LukeTPeterson, qui m'a aider à regler mon problème indexed
}

struct Image {
    // la structure de votre héro chaque paramètre influencera les actions possibles
    image: String,//image en ascii art
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

    //permet de dire que tes arriver a la fin budy
    mort: bool,
}

//permet de définir le nombre de points que gagnera le personnages à chaque fois qu'il gagnera
const GAIN: u64 = 10;
const LIMIT_GAIN: u64 = 90;
//grace à la lib from https://crates.io/crates/serde_yaml
//et https://stackoverflow.com/questions/68404178/how-to-deserialize-this-yaml-in-struct-with-serde-yaml
//pour Deserialize
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

struct Popup {
    show_popup: bool,
    n_popup: i32,
    image:String,
}

fn main() -> io::Result<()> {
    let histoire = fs::read_to_string("hist2").expect("Something went wrong reading the file");

    let mut rng = rand::thread_rng();
    let mut perso = Perso {
        // la structure de héro elle est pour l'instant static
        nom: "Pachat".to_string(),
        charisme: 40,
        force: 40,
        intelligence: 40,
        current_his: 1,
    };
    //définition pop_up
    //ouverture d'image
    let i = fs::read_to_string("image").expect("Something went wrong reading the file");
    let docs_i = YamlLoader::load_from_str(&i).unwrap();
    let doc_i = &docs_i[0];
    let mut out_i = String::new();
    let mut emitter_i = YamlEmitter::new(&mut out_i);
    emitter_i.dump(doc_i).unwrap(); // dump the YAML object to a String
    let image_i=out_i.to_string();

    let mut popup = Popup {
        show_popup: true,
        n_popup: 0,
        image:image_i.to_string(),
    };

    let docs = YamlLoader::load_from_str(&histoire).unwrap();

    //manal à défini dés qui se rénitialise à chaque fois et qui prend la valeur des tirages d'avant pour montrer ce qu'on a fait
    let mut dés: u16 = 0;
    let mut taux_réussite: u16 = 0;
    let mut color: Color = Color::Red;

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
                    [
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                        Constraint::Percentage(33),
                    ]
                    .as_ref(),
                )
                .split(chunks[0]);

                //Nous nous sommes aider de la la doc et sa demo pour faire un pop up
                //https://github.com/fdehau/tui-rs/blob/master/examples/popup.rs
                let pop_up= Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        //les differrents ecart entre chaque cases qu'on créera
                        [
                            Constraint::Percentage(20),
                            Constraint::Percentage(60),
                            Constraint::Percentage(20),
                        ]
                        .as_ref(),
                    )
                    .split(f.size());


            let option = [
                // l'affichage des différent static que vous possédés
                Text::raw("[q]: quitter\n[s]: sauvegarder\n[c]: charger votre sauvegarde"),
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
                .block(Block::default().title("Options").borders(Borders::ALL));
            let lower_left_pane = Paragraph::new(stat.iter())
                .block(Block::default().title("Stats").borders(Borders::ALL));
            /* let right_pane =
            Block::default().title("Histroire").borders(Borders::ALL);
            */

            let mut page = "   ".to_owned() + &current.description + "\n" + "\n";

            //affichage les choix si la partie n'est pas fini
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

            let right_pane = Paragraph::new(text.iter())
                .block(Block::default().title("Histroire").borders(Borders::ALL))
                //.scroll(0) // on donne un titer a la case et on instancie le scroll(a modifier)
                .wrap(true);

            //gauge créé à l'aide de la demo de la bibliothèque à l'adresse https://github.com/fdehau/tui-rs/blob/master/examples/demo/ui.rs
            if dés <= taux_réussite {
                color = Color::Green;
            } else {
                color = Color::Red;
            }

            let label = format!("{}% resultat / {}% stat", dés, taux_réussite);
            let gauge = Gauge::default()
                .block(
                    Block::default()
                        .title("Resultat dés:")
                        .borders(Borders::ALL),
                )
                .style(Style::default().fg(color).modifier(Modifier::ITALIC))
                .label(&label)
                .percent(dés);




            f.render_widget(upper_left_pane, left_chunks[0]); // on rajoute les differentes cases a notre interfaces
            f.render_widget(lower_left_pane, left_chunks[1]);
            f.render_widget(gauge, left_chunks[2]);
            f.render_widget(right_pane, chunks[1]);

            if popup.show_popup {// création du pop up
                let mut popup_m = "   ".to_owned();
                let mut popup_title ;
                match popup.n_popup {
                    1=> popup_m = "Vous allez enregistrer cette partie et écraser l'ancienne. Confirmer ? ".to_owned()+ "\n"+ "\n"+ "[o] pour oui (écraser)     [n] pour non (ne pas enregistrer) ",// Popup confirmation enregistrement
                    2=> popup_m = "Vous avez enregistré cette partie et écraser l'ancienne !".to_owned(),
                    3=> popup_m = "Vous allez charger la dernière partie enregistrée !".to_owned()+ "\n"+ "\n"+ "[o] pour oui (charger)     [n] pour non (ne pas charger)",//Popup confirmation de chargement de partie
                    4=> popup_m = "Vous avez chargé la dernière partie enregistrée !".to_owned(),
                    5=> popup_m = "Souhaiter vous vraiment quitter l'application ? (N'oubliez pas d'enregistrer!)".to_owned()+ "\n"+ "\n"+ "[o] pour oui (quitter)     [n] pour non ",//Confirmation de sortie de l'application
                    6=> popup_m = "Au revoir!".to_owned(),
                    7=> popup_m = popup.image.to_owned(),
                    _=> popup_m = "Titre".to_owned(),//reste au cas ou on se trompe (pop up)
                }

                match popup.n_popup {//Nom popup
                    1|2=> popup_title= "Enregistrer",
                    3|4=> popup_title= "Chargee",
                    5|6=> popup_title= "Quitter",//Popup confirmation de chargement de partie
                    _  => popup_title = "Accueil",//reste au cas ou on se trompe (pop up)
                };
                popup_m = popup_m + "\n"+ "\n"+ "\n"+ "[p] Fermer le pop up";// p pour fermer le pop up
                let popup_message = [Text::raw(popup_m), Text::styled("", Style::default())];
                let block = Paragraph::new(popup_message.iter())
                .block(Block::default().title( popup_title).borders(Borders::ALL));
                f.render_widget(Clear, pop_up[1]); //this clears out the background
                f.render_widget(block, pop_up[1]);
            }

        })?;

        // permet de gérer les type de de d'évenement du clavier
        // grace à Florian Dehau sur github dans https://github.com/fdehau/tui-rs/blob/master/examples/barchart.rs :
        //j'ai pu comprendre que je pouvais faire comme cela pour les différents entrées claviers
        //et nom juste attendre que l'utilsateur tape et appuie sur entrée comme dans: https://www.tutorialspoint.com/rust/rust_input_output.htm
        if let Event::Key(key) = event::read().expect("error") {
            // Action possible quand il y as un pop up
            if popup.show_popup == true {
                if popup.n_popup == 1 {
                    if let KeyCode::Char('o') = key.code {
                        //sauvegarder de la partie actuel
                        let mut serialized_state = serde_yaml::to_string(&perso).unwrap();
                        let mut file = File::create("test.rs")?;
                        file.write_all(serialized_state.as_bytes())?;
                        popup.n_popup = 2;
                    }
                    if let KeyCode::Char('n') = key.code {
                        // permet de fermer le popup.
                        popup.show_popup = false;
                    }
                }

                if popup.n_popup == 3 {
                    if let KeyCode::Char('o') = key.code {
                        // le cas de confirmation de chargement.
                        let sauvegarde = fs::read_to_string("test.rs")
                            .expect("Something went wrong reading the file");
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

                        popup.n_popup = 4;
                    }
                    if let KeyCode::Char('n') = key.code {
                        // permet de fermer le popup.
                        popup.show_popup = false;
                    }
                }

                if popup.n_popup == 5 {
                    if let KeyCode::Char('o') = key.code {
                        //le cas de confirmation la fermeture de l'application.
                        popup.n_popup = 6;
                        break;
                    }
                    if let KeyCode::Char('n') = key.code {
                        // permet de fermer le popup.
                        popup.show_popup = false;
                    }
                }

                if let KeyCode::Char('p') = key.code {
                    // le cas on on appuye sur 'p', permet de fermer le popup.
                    popup.show_popup = false;
                }
            } else {
                //Options
                if let KeyCode::Char('q') = key.code {
                    // le cas on on appuye sur 'q', permet de quitter la partie sans sauvegarder.
                    popup.n_popup = 5;
                    popup.show_popup = true;
                }
                if let KeyCode::Char('s') = key.code {
                    // le cas on on appuye sur 's', permet de sauvegarder votre progression
                    popup.n_popup = 1;
                    popup.show_popup = true;
                }
                if let KeyCode::Char('c') = key.code {
                    // le cas on on appuye sur 'c', permet de charger votre ancienne sauvegarder
                    popup.n_popup = 3;
                    popup.show_popup = true;
                }

                if current.mort == false {// Si le perso n'est pas mort choix
                    if let KeyCode::Char('1') = key.code {
                        // le cas on on appuye sur '1', permet de choisir le choix numéro 1.
                        dés = rng.gen_range(0..100) as u16;
                        taux_réussite = perso.charisme as u16;
                        if dés <= taux_réussite {
                            if perso.charisme< LIMIT_GAIN{
                                perso.charisme += GAIN;
                            }
                            perso.current_his = current.n_1_reussit;
                        } else {
                            perso.current_his = current.n_1_echec;
                        }
                    }
                    if let KeyCode::Char('2') = key.code {
                        // le cas on on appuye sur '2', permet de choisir le choix numéro 2.
                        dés = rng.gen_range(0..100) as u16;
                        taux_réussite = perso.force as u16;
                        if dés <= taux_réussite {
                            if perso.force < LIMIT_GAIN{
                                perso.force += GAIN;
                            }
                            perso.current_his = current.n_2_reussit;
                        } else {
                            perso.current_his = current.n_2_echec;
                        }
                    }
                    if let KeyCode::Char('3') = key.code {
                        // le cas on on appuye sur '3', permet de choisir le choix numéro 3.
                        dés = rng.gen_range(0..100) as u16;
                        taux_réussite = perso.intelligence as u16;
                        if dés <= taux_réussite {
                            if perso.charisme< LIMIT_GAIN{
                                perso.intelligence += GAIN;
                            }
                            perso.current_his = current.n_3_reussit;
                        } else {
                            perso.current_his = current.n_3_echec;
                        }
                    }
                }
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
