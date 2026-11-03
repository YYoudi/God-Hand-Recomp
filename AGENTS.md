# AGENTS.md — God Hand independent engine lab

Répondre en français. Lire docs/workflow.md, docs/clean-room.md et docs/architecture.md
avant de modifier le moteur. La décision utilisateur du 2026-11-03 remplace le
pipeline initial : **réimplémentation indépendante en Rust + Bevy**, code original
public sous MIT, ISO personnelle requise au lancement.

## Périmètre

- crates/disc : lecteur ISO 9660 en lecture seule, sans exécution du jeu.
- crates/engine : logique originale testable sans disque ni GPU.
- crates/app : CLI et sandbox Bevy optionnel (--features viewer).
- specs/ : contrats de comportement avec provenance, sans désassemblage.
- docs/dashboard/data.json : source unique de roadmap et journal.
- scripts/dashboard.py : génère ROADMAP.md ; --check détecte les écarts.
- scripts/check_public_tree.py : contrôle l'index Git avant publication.

MIT couvre notre code seulement. Ne copier ni désassemblage, ni pseudo-code du jeu,
ni C++ recompilé, ni code tiers sous une autre licence dans le moteur. Une traduction
ou un renommage ne rend pas un code clean room. Ne pas prétendre à une certification
juridique ou à une séparation formelle non réalisée. Documenter la provenance.

## Workflow

1. Spécifier comportement observable, provenance et critères avant implémentation.
2. Tests synthétiques uniquement ; aucune donnée du jeu dans la CI.
3. cargo test --workspace --locked ; pour Bevy, cargo check -p godhand-lab --features viewer --locked.
4. Actualiser tâches, journal, validations et limites dans docs/dashboard/data.json.
   Ne marquer « fait » qu'après vérification ; ne pas inventer FPS ou gameplay.
5. Pour un changement visuel, prendre une vraie capture du dashboard. Captures du
   jeu locales seulement. Inscrire date et provenance au manifeste avant indexation.
6. python scripts/dashboard.py ; indexation explicite des sources publiques ;
   python scripts/check_public_tree.py ; python scripts/dashboard.py --check.
7. Après push main, vérifier CI et Pages. Pages gratuit depuis main:/docs,
   domaine github.io, runners standards publics, sans artefacts ou caches persistants.

## Workspace privé préexistant

God Hand (USA).iso, extraction/, GH-Prototype/, ghidra/, PS2Recomp/, recomp/, tools/,
downloads/, pcsx2-v2.8.2-windows-x64-Qt/ et .zcode/ sont ignorés. Ne jamais modifier
les disques ou extractions originales. Avant un travail prototype/diffing, lire
docs/prototype-tcrf-notes.md (local, non publié). Garder cette recherche séparée de
l'implémentation publique. Les anciens prompts 01–05 ne décrivent pas le nouveau moteur.

God Hand et Bevy utilisent **Y comme hauteur**. Les ELF sont strippés. Une déclaration
MCP dans .zcode/config.json ne garantit pas sa disponibilité dans Codex : vérifier
les outils présents avant de l'annoncer.

Audit machine 2026-11-03 : Rust 1.98, toolchains MSVC et GNU installées ; gcc, CMake et
Ninja via G:/Tools/WinLibs/mingw64/bin. L'ancien chemin vcvars64 est absent.
Ghidra 12.1.2 présent. PS2Recomp local propre au commit e5636981 ; binaires présents,
non revalidés sur cette machine. La piste native épinglée dans docs/toolchains.lock.json
est une référence de recherche, pas le moteur choisi.

Évaluation locale du 2026-11-03 : ps2-native-recomp au commit cba42f57 construit et
observé jusqu'au premier combat. Snapshot du runtime enregistré/rechargé ; entrées
programmées testées. Voir docs/native-evaluation-2026-11-03.md pour les limites.
Les artefacts restent dans recomp/experiments/native-2026-11-03/ et le backend ignoré.
Lancement local : recomp/Play-GodHand.cmd (ISO requise). Ce résultat ne décrit pas
la compatibilité du moteur Rust/Bevy. Aucun code original du jeu n'a été consulté
pour implémenter une fonctionnalité de ce moteur durant ce test.

Garder ce fichier, README et prompts/00-contexte-maitre.md cohérents. Rapporter les
artefacts locaux avec chemins absolus. Ne pas déléguer sans instruction explicite.
