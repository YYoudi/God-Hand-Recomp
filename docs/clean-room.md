# Méthode d'implémentation indépendante

La décision est une réimplémentation du moteur, pas une conversion du programme PS2.
L'ISO reste nécessaire comme source de données personnelles au lancement.
Cela demande de spécifier puis reconstruire logique, formats et interprétation.

Une traduction MIPS → C++, même nettoyée, renommée ou réécrite ligne par ligne en Rust,
n'est pas une implémentation indépendante. Ne pas l'importer dans le moteur.

## Observation → spécification → implémentation

1. **Observer** entrées, sorties et comportement sur sa propre copie. Garder captures
   du jeu, dumps et notes sensibles localement.
2. **Spécifier** le contrat dans specs/ : provenance, méthode, unités, tolérances et
   incertitudes. Aucun code original, pseudo-code décompilé ou adresse de fonction.
3. **Implémenter** à partir du contrat et des standards publics, en code original,
   avec des tests et échantillons synthétiques.
4. **Valider** les résultats observables et publier conclusions/mesures agrégées,
   sans données propriétaires.

Pour un composant dont le contrat provient d'une analyse interne du jeu, privilégier
des personnes distinctes pour observer/spécifier et implémenter. Une session exposée
au désassemblage d'un composant ne doit pas prétendre à une séparation stricte pour
son implémentation. Consigner l'exposition et prévoir une revue indépendante.

Le projet adopte un protocole de provenance ; il ne prétend pas à une clean room
formelle déjà accomplie ou à une certification juridique. Les fondations actuelles
reposent sur ISO 9660 et des mécaniques génériques originales.

## Publication

Public : code original, fixtures créées de toutes pièces, contrats fonctionnels,
documentation originale et captures du dashboard. Les images du sandbox original
nécessitent une vérification de provenance avant ajout.

Local uniquement : ISO, BIOS, ELF, IRX, AFS, traductions du jeu, projets Ghidra, dumps,
sauvegardes et captures du jeu. Ne pas les joindre aux issues, releases ou artefacts CI.

La CI réduit le risque de fuite ; elle ne détermine pas la titularité d'un code.
Toute contribution déclare sa provenance et conserve les notices. Voir NOTICE.md.
