# Projet RPG textuel
| Nom             | Prénom | numéro étudiant | Classe| Lien git                                         |
|---              |---     |---              |---    |---                                               |
| Elsegahy        | Rayan  | 20008434        | L2b   |https://code.up8.edu/Smanal/projet-rust-rpg-2021  |
| Saidi           | Manal  | 20007381        | L2b   |                                                  |


# Introduction
Le jeu que nous souhaitons réaliser s'inspire d'un type de livres appelé "Livre dont vous êtes le héros". Il s'agira d'un RPG textuel situé dans un monde de fantasy. Vos choix auront un impact sur vos statistiques et sur le cours de votre aventure. Certains choix seront plus difficiles à faire en raison de vos choix antérieurs, le tout agrémenté de lancers de dés pour ajouter une dimension aléatoire à certaines actions.


# Objectifs 
- Créer un arbre représentant toutes les possibilités de l'histoire, afin de l'implémenter dans notre jeu. Il est important de le faire de manière simple pour permettre aux utilisateurs d'ajouter leurs propres histoires.

- Nous allons implémenter un système de lancer de dés qui sera impacté par les statistiques du joueur, permettant ainsi l'avancement de ce dernier. Comme dans un RPG papier, plus la valeur du dé est grande, moins le lancé est réussi. Les statistiques du personnage déterminent la valeur maximale à partir de laquelle une action est réussie.

	Voici quelques exemples pour clarifier le fonctionnement du système :
	Si votre stat de force correspond à 40 et que vous faites un 99 au lancer de dé, l'action sera ratée.
	En revanche, si vous faites un 10 au lancer de dé, l'action sera réussie.

	En d'autres termes, la difficulté d'une action sera déterminée par la valeur maximale que peut atteindre le dé, qui sera elle-même influencée par les statistiques du personnage. Ce système permettra de rendre les actions plus aléatoires et de donner une dimension de chance et de stratégie aux combats et aux interactions avec l'environnement.

- Le joueur pourra également augmenter ses statistiques grâce aux objets qu'il pourra récolter durant son aventure. Ces objets pourront augmenter temporairement ou définitivement les statistiques du joueur, lui permettant ainsi de progresser plus facilement dans le jeu. Implémenter un système de sauvegarde pour que le joueur puisse revenir plus tard dans sa partie.
- Nous allons implémenter les statistiques dans notre jeu, à savoir la Force, le Charisme et l'Intelligence. Ces trois statistiques permettront de déterminer les compétences et les capacités de notre personnage, ainsi que son niveau de réussite dans les différentes actions qu'il entreprendra. La Force permettra d'améliorer la puissance physique du personnage, le Charisme influencera sa capacité à communiquer et à persuader, tandis que l'Intelligence permettra de résoudre des énigmes et des problèmes complexes. L'ajout de ces statistiques permettra une personnalisation plus poussée du personnage, ainsi qu'une plus grande diversité dans les actions et les choix que pourra effectuer le joueur.
- Nous allons implémenter un système de sauvegarde pour permettre au joueur de sauvegarder sa progression dans le jeu et de reprendre là où il s'était arrêté lorsqu'il le souhaitera.
- Nous allons créer une histoire simple mais originale, tout en respectant la deadline. Nous prévoyons d'implémenter plusieurs fins possibles pour permettre une plus grande diversité dans les choix et les actions du joueur. L'histoire sera élaborée de manière à ce qu'elle soit immersive et captivante, tout en restant accessible à tous les types de joueurs. Nous veillerons également à ce que l'histoire soit en cohérence avec les différentes mécaniques du jeu, telles que les statistiques du personnage et les lancers de dés, pour une expérience de jeu fluide et satisfaisante.

# Objectifs réussis
 -  Implémentation des statistiques.
 -  Écriture du scénario.
 -  Les utilisateurs peuvent ajouter leur propre histoire en JSON en respectant la structure que nous avons définie.
 -  Remplacement de l'arbre par un vecteur.
 -  Système de sauvegarde fonctionnel.
 -  Système de lancer de dé fonctionnel.
 -  Implémentation d'une interface graphique sur le terminal, avec l'utilisation de couleurs et de formes pour une meilleure expérience de jeu.

# Difficultés
 -  Comprendre la librairie Serd/Yaml pour la manipulation de données structurées en RDF et YAML.
 -  Rencontré de nombreux problèmes lors de la désérialisation des données.
 -  Difficulté à implémenter l'interface utilisateur graphique avec TUI (Terminal User Interface).
 -  Un bug majeur a été identifié : si la taille de la fenêtre du terminal est réduite et que l'espace disponible pour le label de la jauge devient insuffisant, le programme s'arrête. Ce problème est connu sous le numéro de ticket '#494' et est censé être résolu dans la version "0.16" de TUI, mais passer d'une version à une autre peut générer des erreurs supplémentaires.

# Voici les Instructions pour lancer le jeu
Pour pouvoir exécuter le programme en Rust sur votre ordinateur, il est nécessaire d'installer le gestionnaire de paquets [Cargo](https://doc.rust-lang.org/cargo/) si vous ne l'avez pas déjà. Ce dernier permet d'automatiser la compilation et la gestion des dépendances de notre projet.

Clonez le dépot sur votre machine : 

`$ git clone https://github.com/Chewingiz/Projet-rpg-rust-2021.git`

Entrer dans le fichiers ou ce trouve le jeux :

`$ cd projet_rust/src`

puis écrire dans le terminal :

`$ cargo run`

Voila le jeu est lancé !

# Instructions

q : Pour quitter le jeu.

s : Sauvegarder votre progression.

c : Charger votre ancienne partie.

1 : Choisir l'option 1.

2 : Choisir l'option 2.

3: Choisir l'option 3.

# Les choses importantes à savoir

## dé
Le système de dé fonctionne aléatoirement. Pour réussir une action, un dé à 100 faces est lancé et vous devez obtenir un résultat inférieur ou égal à la stat correspondante au choix que vous avez fait.

## Histoire 

Nous avons utilisé la sérialisation/désérialisation YAML. Ainsi, si nous souhaitons redéfinir une histoire, il suffit de la réécrire en respectant la syntaxe YAML pour qu'elle soit comprise par notre programme.

```
2: 
  description: | 
    Très étrange comme façons etc...
  choix_1: Essayez de les séduire.
  type1: Charisme
  n_1_reussit: 12
  n_1_echec: 3
  choix_2: L'affronter pour déterminer qui va gagner.
  type2: Force
  n_2_reussit: 13
  n_2_echec: 3
  type3: Intelligence
  choix_3: Le nourrir de protéines pour le rendre aussi puissant qu'un personnage de jojo.
  n_3_reussit: 14
  n_3_echec: 3
  mort: false
```
"X:" : définit le numéro de l'étape
"description "  : cette section est réservée à la saisie de vos textes.

"typeX" : Il est important de spécifier le type d'action pour déterminer à quelle stat la valeur du dé sera comparée afin de déterminer la réussite ou l'échec de l'action.

"n_X_reussite" : si l'action réussit, cette section indique vers quelle étape de l'histoire le joueur sera redirigé.

"n_X_echec" : si l'action échoue, cette section indique vers quelle étape de l'histoire le joueur sera redirigé.

"mort" : il s'agit d'un booléen qui permet de préciser si cette partie de l'histoire est une scène de fin.

# Bibliothèques 
Les bibliothèques que nous avons utilisées sont définies comme dependances dans le "cargo.toml" de notre projet.
```
[package]
name = "projet_rust"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.0"			# Bibliothèque pour créé des valeurs aléatoire
yaml-rust = "0.4"		# Bibliothèque Yaml
tui = "0.9"			# Bibliothèque pour la creation d'une interface utilisateur sur le terminal
termion = "1.5"		# Bibliothèque qui manipule les données bas niveau, comme la lecture ou l'écriture dans le terminal
				# Nécessaire pour l'utilisation de tui
serde = { version = "1.0.103", features = ["derive"] } # Bibliothèque  de serialisation/déserialisation
serde_derive = "1.0.103"	# Bibliothèque pour la deserialisation
serde_yaml = "0.8"		# Bibliothèque serialisation/déserialisation Yaml

[dependencies.crossterm]	# Permet de capter les évenements durant l'execution 
version = "0.17"
features = ["event-stream"] 

```
# Images
# Liens
## Documentations
  - Doc yaml-rust: https://crates.io/crates/yaml-rust
  - Doc lecture de fichier et sauvegarder des données:
https://dev.to/dandyvica/different-ways-of-reading-files-in-rust-2n30#:~:text=Basically%2C%20there%27re%203%20ways%20of%20reading%20ASCII%20files,read%20%28%29%20function%20or%20Ruby%27s%20File.read%20%28%29%20methods 
  - Doc tui : https://docs.rs/tui/0.9.5/tui/
  - Doc rust : https://doc.rust-lang.org/book/

## Bibliothèques
  - Bibliothèque tui : https://github.com/fdehau/tui-rs
  - Bibliotèque de serde: https://github.com/serde-rs/serde
  - Bibliotèque de hjson: https://hjson.github.io/
  - Bibliotèque serde Yaml: https://github.com/dtolnay/serde-yaml

## Aide général
  - Blog sur tui : https://monkeypatch.io/blog/2021/2021-05-31-rust-tui/
  - Tuto utilisation bibli rust : https://www.youtube.com/watch?v=PWfTugeKiOE
  - Reddit tui pour comprendre la bibliothèque : https://www.reddit.com/r/learnrust/comments/gwu0ay/help_with_tuirs/
  - Traducteur Json -> Yaml: https://www.json2yaml.com/ 



