# 🦖 Dino Adventure Game (dévellopé par Arthur Courant et Camille Lenne)

Bienvenue dans **Dino Adventure Game**, un jeu de déplacement et de survie en temps réel où vous incarnez un dinosaure explorant une carte tout en évitant les dangers !

---

## 🎮 Fonctionnalités

- **Déplacements** : Déplacez votre dinosaure sur une carte en utilisant les commandes clavier.
- **Survie** : Perdez des points de vie en cas d'erreur ou de piège, mais atteignez l'objectif pour gagner.
- **Multijoueur IA** : D'autres dinosaures (gérés par l'IA) se déplacent également sur la carte !
- **Score** : Accumulez des points pour chaque action réalisée et enregistrez vos meilleurs scores.

---

## ⚙️ Instructions

### Déplacements :
- Utilisez les touches suivantes :
  - `Z` : Haut
  - `Q` : Gauche
  - `S` : Bas
  - `D` : Droite
  - `X` : Quitter la partie
- L'objectif est d'atteindre le coin inférieur droit de la carte sans mourir.

#### Rencontres possibles :
  - **`Arbre`** :  
    Un arbre se trouve sur certaines cases de la carte. Si vous vous trouvez face à un arbre, vous avez la possibilité de le couper en appuyant sur `y`. En coupant un arbre, vous libérez la case et elle devient une case vide, mais vous n'obtenez aucune récompense immédiate.

  - **`Mur`** :  
    Un mur bloque votre chemin. Vous ne pouvez pas traverser une case qui contient un mur. Vous devrez trouver un autre chemin pour continuer votre exploration.

  - **`Bombe`** :  
    Les bombes sont des objets dangereux qui apparaissent aléatoirement sur la carte. Si vous entrez sur une case avec une bombe, vous risquez de perdre une partie de votre santé. Vous devez faire attention à éviter ces zones ou les désamorcer si vous en avez la possibilité.

  - **`Fin de partie`** :  
    La fin de la partie est atteinte lorsque vous atteignez la case marquée comme `Fin`. Dès que vous atteignez cette case, le jeu se termine et vous gagnez.


### Points de Vie :
- Chaque erreur ou piège vous fait perdre des **points de vie** :
  - **Avancer** : -2 points de vie. Il faut donc faire vite pour arriver au bout avant de vous épuiser!!!
  - **Case bombe** : -20 points de vie.
  - **rencontre avec un dinosaure** : -20 points de vie. **malheuresement pas fonctionnelle les dinosaures sont des figurantss**

### Score :
- Votre score augmente à chaque action. Il est sauvegardé à la fin de la partie si vous quittez avec `X` ou atteignez l'objectif.
- le but est donc d'arriver le plus vite à temps

---

## 📂 Fichiers principaux

- **`main.rs`** : Point d'entrée du jeu.
- **`map.rs`** : Gestion de la carte et des cases.
- **`player.rs`** : Déplacement et gestion des joueurs (humain et dinosaures IA).
- **`score.rs`** : Sauvegarde et affichage des meilleurs scores.
- **`score.txt`** : fichiers contenant tous les scores précédent.
---
