# ADR 001 — Rust + Bevy, moteur indépendant

Date : 2026-11-03. Statut : adopté à la demande de l'utilisateur.

## Choix

Rust pour le moteur original, Bevy 0.20.1 pour ECS/rendu/entrées.
Le moteur ne charge ni n'exécute le programme MIPS et ne lie pas sa traduction.
Les données seront lues depuis l'ISO fournie par l'utilisateur.

PS2Recomp et ps2-native-recomp restent des références de recherche locales.
Ils ne sont pas des dépendances du moteur public. Leur compatibilité God Hand
ne représente pas notre avancement.

| Couche | Responsabilité | Socle |
| --- | --- | --- |
| godhand-disc | ISO 9660 en lecture seule, contrôle des bornes | Racine, SYSTEM.CNF, identification |
| Formats | AFS/FST, géométrie, textures, squelette, audio | À spécifier |
| godhand-engine | Simulation originale testable sans GPU | Déplacement générique sur X/Z |
| Application Bevy | Fenêtre, rendu, caméra, entrées | Sandbox optionnel sol/capsule |
| Jeu | Scènes et comportements observables | Non implémenté |

Le pas de 60 Hz et la vitesse du sandbox sont des choix techniques provisoires,
pas des mesures du jeu. Y = hauteur. Les intentions sont lues à chaque image puis
consommées au pas fixe. Les événements ponctuels devront être tamponnés.

## Travaux nécessaires

Formats et scripts, scènes, rendu, animations, collisions, combat, IA, progression,
audio, sauvegardes et validation de tous les niveaux. Bevy ne fournit pas la logique
de God Hand. Aucune date de fin ou couverture de gameplay n'est annoncée.

## Alternatives

PS2Recomp (GPL-3.0) : traduction et handlers spécifiques, runtime expérimental.
ps2-native-recomp (MIT) : piste pratique de recompilation avec test God Hand rapporté.
PCSX2 : référence de comportement pour observer, pas moteur Bevy.
Voir research-2026-11-03.md pour les sources et limites.
