use std::io::{self, Write};
use tui::{
    *,
    layout::*,
    backend::TermionBackend,
    widgets::*,
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, Borders, Dataset, Gauge,
        Paragraph, Row, Sparkline, Table, Tabs, 
    },
    Frame,
};
use termion::{
    raw::IntoRawMode,
    clear,
};
struct Perso{ // la structure de votre héro chaque paramètre influencera les actions possibles
    nom: String,
    charisme: u64,
    force: u64,
    intellignece: u64,
}
use yaml_rust::YamlEmitter;
use yaml_rust::YamlLoader;
use rand::Rng;

fn main() -> Result<(), io::Error> {

    let histoire =
    "
    1:
        -C’est le matin, de la lumière orangée passe à travers vos rideaux de toile fine et brunie par le temps. Ils sont si anciens que vous ne vous souvenez plus de leur couleur originale… gris ? Marron peut-être… De toute façon vous n’en avez que faire, c’est la dernière fois que vous aurez l'occasion de les voir. Aujourd’hui vous partez.
        « Cette vie n’est pas faite pour moi» vous vous dites en regardant la tâche de moisie au-dessus de votre lit qui ressemble vaguement à un visage. Vous préparez rapidement vos affaires et ramassez quelque provision de celle qui ne semble pas encore létale (un morceau de pain sec) et une dague à la lame bleutée .
        Depuis le retour des monstres, vampires et autre hamster tueur, la famine s’est abattue sur le royaume. Vous ne savez pas à quoi vous attendre mais vous n’avez pas le choix, vous allez mourir de faim si vous ne tentez rien.
        
        De plus, le roi rongé par la maladie, sans successeur, met le pays entier dans une position délicate. Il a fait distribuer dans tout le royaume des invitations à une sorte de spectacle lors duquel il choisira la personne qui le remplacera.
        « Qui sait » vous vous êtes dit en la lisant «Je n’ai absolument rien à faire, rien à manger et aucun background ! C’est presque comme si la personne qui m'avait écrit avait eu la flemme d’en créer et qu'elle devait trouver une raison afin de commencer une aventure le plus vite possible sans raison particulière pour ne pas perdre l’intérêt des lecteurs. Je devrais y aller ! ».
        Vous êtes face à votre demeure… Enfin... vôtre cabane, l'aventure commence ! Que faites-vous?
    2:
        - va bruler la maison
    3:
        - va lire la doc 
    ";

let docs = YamlLoader::load_from_str(histoire).unwrap();
let doc = &docs[0];
let mut out_str = String::new();
let mut emitter = YamlEmitter::new(&mut out_str);
emitter.dump(doc).unwrap(); // dump the YAML object to a String


    let perso = Perso{ // la structure de héro elle est pour l'instant static 
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
        let test = BarChart::default()
        .block(Block::default().title("BarChart").borders(Borders::ALL)) // on précise que tous sera entouré par une bordure 
        .bar_width(1) // d'une épaiseur de 1
        .bar_gap(5) // enfin elle seront toutes une distances des uns des autres de 5 (les diférentes case)
        .style(Style::default().fg(Color::Yellow).bg(Color::Red))
        .value_style(Style::default().fg(Color::Red).modifier(Modifier::BOLD))
        .label_style(Style::default().fg(Color::White)); // la couleur de la bordure sera blanche 
        let chunks = Layout::default()// on crée les différents case de notre interface sur le terminal 
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints( //les differrents ecart entre chaque cases qu'on créera
                [
                    Constraint::Percentage(30), 
                    Constraint::Percentage(50),
                ]
                .as_ref(),
            )
            .split(f.size());
        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(//les differrents ecart entre chaque cases qu'on créera
                [
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]
                .as_ref(),
            )
            .split(chunks[0]);
        
            let option = [ // l'affichage des différent static que vous possédés
                Text::raw("q: pour quitter\ns: pour sauvegarder"),
                Text::styled("", Style::default())
            ];
            let help= "Pseudo: ".to_owned()+ &perso.nom+"\n" + "Charisme: " + &perso.charisme.to_string()+"\n"+"Force: "+ &perso.force.to_string() +"\n"+ "Intelligence: " + &perso.intellignece.to_string()+"\n";
            let stat = [
                Text::raw(help ),
                Text::styled("", Style::default())
        ];
        let upper_left_pane = Paragraph::new(option.iter())
        .block(Block::default().title("Option").borders(Borders::ALL));
        let lower_left_pane =Paragraph::new(stat.iter())
            .block(Block::default().title("Stat").borders(Borders::ALL));
       /* let right_pane =
            Block::default().title("Histroire").borders(Borders::ALL);
         */   
                    
        let text = [ // le texte qui sera vue dans l'interface du terminal
    	Text::styled(out_str, Style::default())
	];
let right_pane =Paragraph::new(text.iter())
    .block(Block::default().title("Histroire").borders(Borders::ALL)).scroll(0)// on donne un titer a la case et on instancie le scroll(a modifier)
    .wrap(true);
        
        
        f.render_widget(upper_left_pane, left_chunks[0]);// on rajoute les differentes cases a notre interfaces
        f.render_widget(lower_left_pane, left_chunks[1]);
        f.render_widget(right_pane, chunks[1]);
    })?;
    print!("\r\n");
    Ok(())
}