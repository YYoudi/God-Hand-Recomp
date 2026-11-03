# God Hand · Independent Engine Lab

Réimplémentation indépendante du moteur de **God Hand (USA, PS2)** en **Rust + Bevy**.
Notre code original est public sous MIT. Chaque utilisateur doit fournir son ISO.
Aucun code, modèle, texture, son ou niveau du jeu n'est distribué.

**État : fondations techniques. Le jeu n'est pas jouable dans ce moteur.**
Le lecteur ISO est en lecture seule. Le sandbox utilise uniquement des primitives originales.

- [Dashboard et journal](https://YYoudi.github.io/God-Hand-Recomp/)
- [Roadmap](ROADMAP.md) · [Workflow](docs/workflow.md)
- [Méthode clean room](docs/clean-room.md) · [Architecture](docs/architecture.md)
- [État de l'art au 3 novembre 2026](docs/research-2026-11-03.md)
- [Contribuer](CONTRIBUTING.md) · [Licences et périmètre](NOTICE.md)

## Démarrer

Installer Rust **1.98+** et les [prérequis Bevy](https://bevy.org/learn/quick-start/getting-started/setup/).
Bevy est épinglé à **0.20.1**, les dépendances dans Cargo.lock.
Les tests ne nécessitent ni GPU ni données du jeu.

```sh
cargo test --workspace --locked
cargo run -p godhand-lab -- --iso "/chemin/God Hand (USA).iso"
cargo run -p godhand-lab --features viewer -- --iso "/chemin/God Hand (USA).iso" --sandbox
```

Le lancement exige une ISO dont SYSTEM.CNF référence SLUS_215.03. Ce contrôle
reconnaît une structure et un numéro de série ; il ne certifie pas l'authenticité.
Le sandbox affiche un sol et une capsule, déplaçable avec WASD ou les flèches.
Ce n'est pas une scène du jeu.

Windows : une toolchain GNU peut être utilisée avec
`cargo +stable-x86_64-pc-windows-gnu ...` et MinGW sur le PATH.
MSVC nécessite Visual C++ Build Tools et le SDK Windows.

## Sources publiques et données locales

| Public | Exclus de Git |
| --- | --- |
| crates/ : lecteur, logique originale, application Bevy | ISO, BIOS, ELF, IRX, AFS |
| specs/ : contrats avec provenance | Code recompilé, binaires du jeu, dumps |
| docs/ : architecture, roadmap, dashboard | Extractions, Ghidra, sauvegardes |
| scripts/ : outils de suivi et vérification | Backends de recherche, captures du jeu |

L'ancienne piste de recompilation reste **locale pour la recherche**. Elle n'alimente
pas le moteur public. Une traduction MIPS vers C++ n'est pas du code clean room.

## Suivi public gratuit

GitHub Pages publie **main:/docs** sur le domaine gratuit github.io. Le dashboard
statique ne dépend ni d'une API payante ni d'un hébergement externe. Les contrôles CI
utilisent des runners Ubuntu standards publics, sans cache ni artefact persistant.

Après chaque modification : actualiser docs/dashboard/data.json, régénérer
ROADMAP.md avec `python scripts/dashboard.py`, ajouter une vraie capture du
dashboard en cas de changement visuel, puis contrôler l'index avec
`python scripts/check_public_tree.py`.
