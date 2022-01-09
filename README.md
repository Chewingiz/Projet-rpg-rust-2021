# Projet RPG textuel
| Nom             | Prénom | numéro étudiant | Classe| Lien git                                         |
|---              |---     |---              |---    |---                                               |
| Elsegahy        | Rayan  | 20008434        | L2b   |https://code.up8.edu/Smanal/projet-rust-rpg-2021  |
| Saidi           | Manal  | 20007381        | L2b   |                                                  |


# Introduction
Le jeu que nous voulons réaliser s'inspire d’un type de livres nommé Livre où vous êtes le héros. Il s’agira d’un RPG où vous serez transporté dans un monde fantaisie textuelle. Vos choix impacteront vos stat et la suite de votre aventure. Certains choix seront plus durs à réaliser que d’autre due a vos choix précédents, le tout saupoudré de lancer de dé permettant de voir aussi si vous pouvez réussir certaines actions.


# Objectifs 
- Pouvoir réussir à créer un arbre représentant toutes les possibilités de l’histoire, implémenter ça dans notre jeu. (bien implémenter ou prendre en comprenant bien et pouvoir expliquer des fonctionnements)
- Implémenter un système de lancé de dé qui sera impacté par les stats du joueur permettant l'avancement de ce dernier.
- Il pourra aussi par ces actions augmenter ces stats par le biais d’item récolter durant son aventure. Implémenter un système de sauvegarde pour que le joueur puisse revenir plus tard dans sa partie.
- Bien réussir à bien implémenter les stats dans notre jeu (Force, Charisme, Intelligence).
- Faire une histoire simple, mais à la fois originale tout en respectant la deadline (ne pas partir pour créer le scénario du seigneur des anneaux) avec plusieurs fins possibles.

# Objectif réussi
- Le joueur peut revenir plus tard dans sa partie.
- Bien réussir à bien implémenter les stats dans notre jeu (Force, Charisme, Intelligence).
- Faire une histoire simple, mais à la fois originale tout en respectant la deadline (ne pas partir pour créer le scénario du seigneur des anneaux) avec plusieurs fins possibles.
- On a remplacé l'arbre par un vecteur.
- Faire une fonction de sauvegarde et de chargement de sauvegarde, pour revenir à sa progression.

# Difficulter
- Comprendre la librairie de serd/Yaml.
- Beaucoup de problèmes à faire la désérialisation.
- À faire le terminal avec tui.
- Le bug principal, est que si on réduit trop la page du terminal et qui ne reste plus assez de place pour le label de la jauge, le programme s'arrête. Le problème est connu comme étant un problème de la jauge. Elle est censée être réparé dans la version "0.16" de tui '#494', mais le changement d'une version à une autre est ce compliqué ce qui génère beaucoup d'erreur.

# Voici les Instructions pour lancer le jeu

cloner le dépot sur votre machine : 

`$ git clone https://code.up8.edu/Smanal/projet-rust-rpg-2021.git`

entrer dans le fichiers ou ce trouve le jeux :

`$ cd projet_rust`

puis :

`$ cd src`

puis écrire dans le terminal :

`$ cargo run`

Voila le jeu est lancé !

# Instructions

q : Pour quitter le jeu.

s : Sauvegarder votre progression.

c : Charger votre ancienne partie.

1 : Choisir le choix 1.

2 : Choisir le choix 1.

3: Choisir le choix 3.

# Les choses importantes à savoir

## dé

Le système de dé ce fait aléatoirement. Pour pouvoir réussir une action, un dé de 0 à 100 va être lancé, il faut que le résultat de votre lancé soit inférieur ou égale à la stats du choix que vous avez décidé de choisir.

## Histoire 

Nous avons utilisé une sérialisation/désérialisation Yaml. Ainsi, si nous voulons redéfinir une histoire, nous avons juste à refaire la syntaxe Yaml pour qu'elle ne soit comprise pas notre programme.

```
2: 
  description: | 
    Très étrange comme façons etc...
  type1: Charisme
  n_1_reussit: 12
  n_1_echec: 3
  choix_2: l'affronter pour savoir qui soulèvera.
  type2: Force
  n_2_reussit: 13
  n_2_echec: 3
  type3: Intelligence
  choix_3: Le nourrir de protéin pour le rendre aussi puissant qu'un personnage de jojo.
  n_3_reussit: 14
  n_3_echec: 3
  mort: false
```
description : vous aurez juste à mettre vos textes.

type1 : vous devez décrire sur quel type l'action ou le dé va être utilisé sur quelle stats pour le choix.

n_X_reussit : si l'action réussie, vers ou le joueur sera redirigé.

n_X_echec : si l'action rate, vers ou le joueur sera redirigé.

mort : un booléen qui dit précise si le scénario est un scénario de fin.

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
# Lien
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



