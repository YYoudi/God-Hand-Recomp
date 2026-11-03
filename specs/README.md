# Spécifications indépendantes

Chaque contrat décrit un comportement, pas le programme original. Inclure :

- identifiant de tâche, version et auteur/date ;
- provenance : standard public ou protocole d'observation externe ;
- entrées/sorties, unités et cas limites ;
- observations, hypothèses et tolérances séparées ;
- tests synthétiques et critères d'acceptation ;
- déclaration d'exposition au code du composant original.

Lecteur actuel : [ECMA-119](https://ecma-international.org/publications-and-standards/standards/ecma-119/),
descripteur primaire au secteur 16, blocs de 2048 octets, répertoire et nombres dans
les deux ordres d'octets. Sous-ensemble : racine ISO 9660 sans Joliet, Rock Ridge,
attributs étendus ou entrelacement. Bornes vérifiées sur la taille du fichier.
Aucune écriture ni extraction.

SLUS_215.03 est un identifiant public de compatibilité, pas un algorithme du jeu.
Les constantes du sandbox sont des choix originaux provisoires.
Aucune spécification du gameplay n'est encore validée.
