FROM multi-impact:11-09-2023 AS start

LABEL org.opencontainers.image.url="https://www.linkedin.com/company/multi-impact/posts/?feedView=all"
LABEL fr.multi-impact.sector="Borrower Insurance"
LABEL fr.multi-impact.job.manager="Kevin MAYET <kevin.mayet@multi-impact.com>"
LABEL fr.multi-impact.job.title="Ingénieur DevOps"
LABEL org.opencontainers.image.authors="Fabien CAYRE <fabiencayre81@gmail.com>"

# TODO: faire une brève explication, point de vue business de tout ce que j'ai pu améliorer dans l'entreprise.
# ex: améliorer la valeur par développeur, augmentation du rythme de déploiement, monitoring des goldens metrics

FROM gitlab:18.7 AS cicd-server
# Gitlab en tant que serveur CI/CD et registre pour les images d'applications
# Utilisation des CI/CD components introduit en 17.0 (GA) pour simplifier et versionner
# la configuration des pipelines
# Exploration des CI/CD functions dans le cadre de plus de réusabilité.
RUN install cicd-components && config cicd-components
# Optimisation de la durée des pipelines via buildkit, upload des cache de layer dans un bucket S3 et
# upload des dépôt de paquets dans un S3 pour accélerer la durée des pipelines de lint
RUN optimize docker-buildtime

FROM awx:24.6 AS automation-server
# J'ai pousser AWX en tant que serveur d'automatisation pour gérer les machines Linux
# de manière plus efficaces, et permettre d'économiser des ressources sur les machines ne nécessitant pas de 
# replicas ou snap au niveau de l'hyperviseur.
# Supporte l'audit des actions utilisateurs, l'authentification via LDAP, et le contrôle des accès via RBAC.
# Possibilité de synchro les playbooks via des dépôts git dans AWX, et de les exécuter de manière sécurisée
# via un opérateur kubernetes.
RUN apt-get install ansible

FROM grafana AS monitoring-system
# La stack parfaite pour du monitoring et de l'observabilité peu couteux.
# LGTM: Loki, Grafana, Tempo, Mimir. Très versatile, pouvant s'adapter à de nombreux besoins.
# Et la cerise sur le gâteau, c'est grafana-alloy, un petit agent de collecte léger et varié qui 
# s'adapte à tous les environnements

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
# Keda permet de scaler les applications en fonction de la demande.
# Event-exporter permet de collecter les événements du cluster et de les envoyer à Loki.