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
struct Perso {
    // la structure de votre héro chaque paramètre influencera les actions possibles
    nom: String,
    charisme: u64,
    force: u64,
    intellignece: u64,
}
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::json;
//use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, PartialEq, Debug)]
struct Current{
    description: i32,
    choix_1: i32,
    n_1: i32,
    choix_2: i32,
    n_2: i32,
    choix_3: i32,
    n_3: i32,
}
use serde_json::from_str;
use std::fs;
use yaml_rust::YamlEmitter;
use yaml_rust::YamlLoader;
use serde_yaml::Value;

#[derive(Serialize, Deserialize,Debug)]
struct Person {
    name: String,
    age:  u16,
    phones: Vec<String>,
}

fn typed_example(data : &str) -> serde_yaml::Result<Person> {
    // Some JSON input data as a &str. Maybe this comes from the user.


    // Parse the string of data into a Person object. This is exactly the
    // same function as the one that produced serde_json::Value above, but
    // now we are asking it for a Person as output.
    //let p: Person = serde_json::from_str(data)?;
let p: Person =serde_yaml::from_str(data)?;
    // Do things just like with any other Rust data structure.
    println!("Please call {} at the number {}", p.name, p.phones[0]);

    Ok(p)
}

fn main() -> io::Result<()>  {
    let histoire = fs::read_to_string("hist2").expect("Something went wrong reading the file");

    let docs = YamlLoader::load_from_str(&histoire).unwrap();
    let doc = &docs[0][1];
    let mut out_str = String::new();
    let mut emitter = YamlEmitter::new(&mut out_str);
    emitter.dump(doc).unwrap(); // dump the YAML object to a String
    let data: &str = &out_str;
    /*let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
*/
        let myhson=typed_example(data);
    //let myhson: serde_json::Result<Person> = serde_json::from_str(stack_str).unwrap().expect("Something went wrong reading the file");
    println!("{:?}", myhson);

//: Result<Person , serde_json::Error> // : serde_json::Result<Person>
/*
    let perso = Perso {
        // la structure de héro elle est pour l'instant static
        nom: "Pachat".to_string(),
        charisme: 1,
        force: 2,
        intellignece: 4,
    };
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
    print!("\r\n");*/
    Ok(())
}
