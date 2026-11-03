# Périmètre et dépendances

La licence MIT couvre uniquement le code et la documentation originaux de ce dépôt.
God Hand, ses programmes, images, musiques, textes et marques appartiennent à leurs
ayants droit. Aucune licence sur ces éléments n'est accordée ici. Projet indépendant,
sans affiliation avec Capcom, Clover Studio ou Sony.

Ce dépôt distribue un moteur indépendant en cours de préparation, pas un exemplaire
du jeu ni un port déjà jouable. Chaque utilisateur fournit son image du disque USA
(`SLUS_215.03`). Le lancement vérifie sa présence ; les tests utilisent des fixtures
synthétiques sans disque. Aucune fonction
de téléchargement de jeu, de BIOS ou de contournement de protection n'est fournie.

## Dépendances téléchargées séparément

| Projet | Licence observée | Usage |
| --- | --- | --- |
| [Rust](https://www.rust-lang.org/) et [Bevy](https://bevy.org/) | MIT / Apache-2.0 | Outils et moteur ECS ; dépendances téléchargées par Cargo |
| [ps2-native-recomp](https://github.com/exoteric33/ps2-native-recomp) | MIT | Référence de recherche locale ignorée, pas dépendance du moteur |
| [ps2-web-recomp](https://github.com/snuri00/ps2-web-recomp) | MIT | Origine de la piste native, référence |
| [PS2Recomp](https://github.com/ran-j/PS2Recomp) | GPL-3.0 | Piste historique conservée localement |
| [wgpu-native](https://github.com/gfx-rs/wgpu-native) | MIT / Apache-2.0 | Rendu de la piste native |
| [SDL2](https://github.com/libsdl-org/SDL) | Zlib | Fenêtre, audio et contrôleurs |

Les notices complètes restent dans les dépendances. Le décodeur IPU du backend natif
inclut une notice BSD pour des tables provenant de Play!. Les outils portables ont
leurs propres licences. MIT ne permet pas de réétiqueter du code GPL importé : toute
future incorporation exige de conserver les notices et de respecter les obligations
de la licence concernée. Aucun code tiers de ces backends n'est copié dans ce dépôt.

## Publication

Ne pas publier : disques, BIOS, exécutables PS2, IRX, archives de données, traductions
C++ du jeu, binaires liés au jeu, dumps mémoire, projets Ghidra, sauvegardes ou captures
contenant des éléments du jeu. Les captures publiques actuelles montrent exclusivement
notre dashboard, sans logo ni illustration du jeu. Les captures de gameplay restent
locales dans `recomp/` ; leur éventuelle publication nécessiterait une décision distincte.

Les exclusions Git et le contrôle CI réduisent le risque de fuite accidentelle ; ils
ne constituent pas une analyse automatique exhaustive des droits d'auteur.
