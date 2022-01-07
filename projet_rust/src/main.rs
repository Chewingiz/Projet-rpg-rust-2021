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

#[derive(Serialize, Deserialize, Debug)]
struct Perso {
    // la structure de votre héro chaque paramètre influencera les actions possibles
    nom: String,
    charisme: u64,
    force: u64,
    intellignece: u64,
    current_his: usize, // grace a ce fameux LukeTPeterson, qui m'a aider à regler mon problème indexed 
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, PartialEq, Debug)]
struct Current {
    description: i32,
    choix_1: i32,
    n_1: i32,
    choix_2: i32,
    n_2: i32,
    choix_3: i32,
    n_3: i32,
}
use std::fs;
use std::fs::File;
use std::time::Duration;
use std::time::Instant;
use yaml_rust::YamlEmitter;
use yaml_rust::YamlLoader;
fn main() -> Result<(), io::Error> {
    //https://doc.rust-lang.org/book/ch12-02-reading-a-file.html : ma permit de comprendre comment lire dans un fiichier
    let histoire = fs::read_to_string("best").expect("Something went wrong reading the file");
    let docs = YamlLoader::load_from_str(&histoire).unwrap();

    let mut perso = Perso {
        // la structure de héro elle est pour l'instant static
        nom: "Pachat".to_string(),
        charisme: 1,
        force: 2,
        intellignece: 4,
        current_his: 1,
    };

    loop {
        // boucle de notre jeux qui s'arrete si on attein un end game ou si on appui sur 'q'
        //https://crates.io/crates/yaml-rust: m'a permit de comprendre comment marche la Yaml
        let doc = &docs[0][perso.current_his];
        let mut out_str = String::new();
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(doc).unwrap(); // dump the YAML object to a String


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
                + &perso.intellignece.to_string()
                + "\n";
            let stat = [Text::raw(help), Text::styled("", Style::default())];
            let upper_left_pane = Paragraph::new(option.iter())
                .block(Block::default().title("Option").borders(Borders::ALL));
            let lower_left_pane = Paragraph::new(stat.iter())
                .block(Block::default().title("Stat").borders(Borders::ALL));
            /* let right_pane =
            Block::default().title("Histroire").borders(Borders::ALL);
            */

            let text = [
                // le texte qui sera vue dans l'interface du terminal
                Text::styled(out_str, Style::default()),
            ];
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
            if let KeyCode::Char('c') = key.code {
                // le cas on on appuye sur 'c', permet de charger votre ancienne sauvegarder
                let sauvegarde = fs::read_to_string("test.rs").expect("Something went wrong reading the file");
                let docs = YamlLoader::load_from_str(&sauvegarde).unwrap();
                let doc = &docs[0];
                let mut resulta = String::new();
                let mut emitter = YamlEmitter::new(&mut resulta);
                emitter.dump(doc).unwrap(); 
                let p: Perso =serde_yaml::from_str(&resulta).expect("error");
                perso.nom = p.nom;
                perso.charisme = p.charisme;
                perso.intellignece = p.intellignece;
                perso.force = p.force;
                perso.current_his = p.current_his;
            }
        }
        

        print!("\r\n");
    }
    Ok(())
}
