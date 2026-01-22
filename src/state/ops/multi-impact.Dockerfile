FROM multi-impact:11-09-2023 AS start

LABEL org.opencontainers.image.url="https://www.linkedin.com/company/multi-impact"
LABEL org.opencontainers.image.authors="Fabien CAYRE fabiencayre81@gmail.com"
LABEL fr.multi-impact.job.title="Ingénieur DevOps"
LABEL fr.multi-impact.sector="Courtier en assurance emprunteur"
LABEL fr.multi-impact.job.manager="Kevin MAYET kevin.mayet@multi-impact.com"
# Mon but était principalement de reprendre et d'améliorer l'architecture existante, mais j'ai proposer d'aller plus loins.
# En résuidant la durée de déploiement, nous avons pu améliorer la vélocité des équipes, et délivrer plus de features.
# En fusionnant les anciens outils éparpillés, nous avons pu augmenter la disponibilité de notre plateforme, pour ne pas bloquer nos clients.
# En monitorant les métriques clées, nous avons réussi à localiser le bottleneck des applications.
# > Toutes ces avancées ont permis d'augmenter la productivité des gestionnaires, en mesurant les KPI clés directement via les outils d'observabilité.

# J'essaye de trouver les outils les plus adaptés, qui nécessitent le moins de changement au niveau de la couche logicielle
# J'ai découvert le 12factor app au début de mon CDI ( https://12factor.net/fr/ ) et j'y ai adhéré.

FROM gitlab:18.7 AS cicd-server
# Gitlab en tant que serveur CI/CD et registre pour les images d'applications
# Utilisation des CI/CD components introduit en 17.0 (GA) pour simplifier et versionner
# la configuration des pipelines
# Exploration des CI/CD functions dans le cadre de plus de réusabilité.
RUN apk add cicd-components && config cicd-components
# Optimisation de la durée des pipelines via buildkit, upload des cache de layer dans un bucket S3 et
# upload des dépôt de paquets dans un S3 pour accélerer la durée des pipelines de lint
RUN apk add docker-buildkit s3

FROM awx:24.6 AS automation-server
# J'ai pousser AWX en tant que serveur d'automatisation pour gérer les machines Linux
# de manière plus efficaces, et permettre d'économiser des ressources sur les machines ne nécessitant pas de 
# replicas ou snap au niveau de l'hyperviseur.
# Supporte l'audit des actions utilisateurs, l'authentification via LDAP, et le contrôle des accès via RBAC.
# Possibilité de synchro les playbooks via des dépôts git dans AWX, et de les exécuter de manière sécurisée
# via un opérateur kubernetes.
RUN apt-get install awx ansible ansible-galaxy

FROM grafana:12.3 AS monitoring-system
# La stack parfaite pour du monitoring et de l'observabilité peu couteux.
# LGTM: Loki, Grafana, Tempo, Mimir. Très versatile, pouvant s'adapter à de nombreux besoins.
# Et la cerise sur le gâteau, c'est grafana-alloy, un petit agent de collecte léger et varié qui 
# s'adapte à tous les environnements

ADD sentry /monitoring-system
# L'utilisation de Sentry dans notre stack de monitoring permet de réveler des informations
# métiers essentielles. L'outil est extremement puissant, même dans sa version auto-hebergeable, 
# néanmoins cela m'a conduit à comprendre tous les composants du logiciel pour le gérer proprement.

FROM kubernetes/rancher:rke2-v1.35 AS orchestrator
# Incontournable pour orchestrer les applications, j'ai améliorer la mise en place et la gestion
# avec l'outil rancher, qui rassemble tous les clusters au même endroit. Il facilite la gestion des utilisateurs
# et des autorisations.
RUN apt-get install k9s fluxcd2 helm cilium keda event-exporter
# K9S est parfait pour gérer les clusters via le CLI
# Pour synchroniser les dépôts git avec les clusters, j'ai utilisé fluxcd2
# Pour gérer les déploiements, j'ai utilisé helm
# Les besoins des clients de Multi-Impact nécesitent une infrastructure sécurisée et résiliente.
# Cilium et son mode natif eBPF permet de sécuriser les communications
# entre les pods tout en offrant des performances optimales.
# Les CiliumNetworkPolicies étaient un peu complexe a prendre en main
# mais j'ai proposé de mettre le système en liste blanche afin de supprimer tous les flux non voulu.
# Keda permet de scaler les applications en fonction de la demande.
# Event-exporter permet de collecter les événements du cluster et de les envoyer à Loki.