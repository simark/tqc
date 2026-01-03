# tqc

Un outil en ligne de commande pour télécharger des épisodes de [Télé-Québec](https://video.telequebec.tv/).

## Fonctionnalités

- **Lister les émissions** - Parcourir les saisons et épisodes avec leurs métadonnées (durée, identifiants)
- **Téléchargements flexibles** - Télécharger un seul épisode, une saison complète ou une émission au complet
- **Suivi de la progression** - Affiche la progression (ex: `[5/52]`) lors du téléchargement de plusieurs épisodes
- **Résilient** - Continue le téléchargement des épisodes restants si un échoue, avec un résumé à la fin

## Prérequis

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) doit être installé et accessible dans votre PATH

## Installation

```bash
cargo install --path .
```

Ou compiler manuellement :

```bash
cargo build --release
./target/release/tqc --help
```

## Utilisation

### Trouver le slug d'une émission

Les slugs se trouvent dans les URLs de Télé-Québec. Par exemple, l'émission « Simon » à :
```
https://video.telequebec.tv/tv-show/32951-simon
```
a le slug `32951-simon`.

### Lister tous les épisodes

```bash
tqc list 32951-simon
```

Sortie :
```
Show: Simon (ID: 32951)

Saison 1 (ID: 32952) (52 episodes):
  EP01 (ID: 33285): Super lapin (6 min)
  EP02 (ID: 33284): Les petites roues (6 min)
  ...
```

### Télécharger un seul épisode

```bash
tqc download 32951-simon 1 5    # Saison 1, Épisode 5
```

### Télécharger une saison complète

```bash
tqc download 32951-simon 1      # Toute la saison 1
```

### Télécharger une émission au complet

```bash
tqc download 32951-simon        # Toutes les saisons
```

## Format des fichiers

Les fichiers téléchargés sont nommés ainsi :
```
{Émission} - S{saison}E{épisode} - {Titre}.mp4
```

Par exemple : `Simon - S01E01 - Super lapin.mp4`

## Fonctionnement

tqc utilise l'API publique de Télé-Québec (basée sur Brightcove) pour énumérer les émissions, saisons et épisodes. Le téléchargement vidéo est effectué par yt-dlp, qui supporte nativement le lecteur Télé-Québec/Brightcove.

## Licence

MIT
