# 🔍 OSINT Toolkit - Guide d'utilisation

## Configuration initiale

### Configurer le proxy
```bash
# Proxy simple
./osint_toolkit config set-proxy http://proxy.example.com:8080

# Proxy avec authentification
./osint_toolkit config set-proxy http://proxy.example.com:8080 -u username -p password
```

### Configurer l'User-Agent
```bash
# Presets disponibles
./osint_toolkit config set-user-agent chrome    # Chrome (défaut)
./osint_toolkit config set-user-agent firefox   # Firefox
./osint_toolkit config set-user-agent safari    # Safari
./osint_toolkit config set-user-agent edge      # Edge
./osint_toolkit config set-user-agent mobile    # Mobile
./osint_toolkit config set-user-agent bot       # Bot

# User-Agent personnalisé
./osint_toolkit config set-user-agent "Mon User Agent Custom"
```

### Configurer les threads
```bash
./osint_toolkit config set-threads 20
```

### Ajouter des clés API
```bash
./osint_toolkit config set-key shodan YOUR_SHODAN_API_KEY
./osint_toolkit config set-key github YOUR_GITHUB_TOKEN
./osint_toolkit config set-key hibp YOUR_HIBP_KEY
```

### Voir la configuration
```bash
./osint_toolkit config list
```

## 🔎 Recherches & Analyse

### Recherche de pseudos
```bash
# Recherche basique
./osint_toolkit username johndoe

# Filtrer les plateformes
./osint_toolkit username johndoe -p "github,twitter,linkedin"

# Export en JSON
./osint_toolkit username johndoe -f json -o results.json

# Export en CSV
./osint_toolkit username johndoe -f csv -o results.csv
```

### Analyse d'emails
```bash
# Vérification basique
./osint_toolkit email john.doe@example.com

# Avec recherche de fuites et réseaux sociaux
./osint_toolkit email john.doe@example.com --breaches --social

# Export formaté
./osint_toolkit email john.doe@example.com --social -f json
```

### Analyse de numéros de téléphone
```bash
# Analyse basique
./osint_toolkit phone +33612345678

# Avec informations opérateur
./osint_toolkit phone +33612345678 --carrier

# Export JSON
./osint_toolkit phone +33612345678 --carrier -f json
```

### Analyse IP
```bash
# Analyse complète
./osint_toolkit ip 8.8.8.8 --reverse --asn --geo

# Reverse DNS uniquement
./osint_toolkit ip 8.8.8.8 --reverse

# Géolocalisation uniquement  
./osint_toolkit ip 8.8.8.8 --geo
```

### Analyse de domaines
```bash
# Analyse complète
./osint_toolkit domain example.com --dns --ssl --whois

# DNS uniquement
./osint_toolkit domain example.com --dns
```

### Vérification de fuites
```bash
# HaveIBeenPwned
./osint_toolkit leaks john@example.com --hibp

# Avec recherche de mots de passe
./osint_toolkit leaks johndoe --hibp --passwords
```

### Extraction de métadonnées
```bash
# Analyser un fichier
./osint_toolkit metadata /path/to/file.jpg

# Export JSON
./osint_toolkit metadata /path/to/document.pdf -f json
```

## 🌍 Réseaux & Infrastructure

### Énumération de sous-domaines
```bash
# Certificate Transparency
./osint_toolkit subdomain example.com --crt

# Brute force DNS
./osint_toolkit subdomain example.com --brute

# Les deux méthodes
./osint_toolkit subdomain example.com --crt --brute
```

### Recherche Shodan
```bash
# Recherche basique
./osint_toolkit shodan "apache"

# Avec vulnérabilités
./osint_toolkit shodan "apache" --vulns

# Limiter les résultats
./osint_toolkit shodan "apache" -l 50
```

### Scan de ports
```bash
# Scan complet
./osint_toolkit portscan 192.168.1.1

# Plage de ports spécifique
./osint_toolkit portscan 192.168.1.1 -p "1-1000"

# Type de scan
./osint_toolkit portscan 192.168.1.1 --scan-type tcp
```

### WHOIS
```bash
# Lookup basique
./osint_toolkit whois example.com

# Avec parsing
./osint_toolkit whois example.com --parse

# IP
./osint_toolkit whois 8.8.8.8
```

### GeoIP
```bash
# Géolocalisation basique
./osint_toolkit geoip 8.8.8.8

# Avec informations ISP
./osint_toolkit geoip 8.8.8.8 --isp
```

## 🔍 Recherche Web

### Moteurs de recherche
```bash
# DuckDuckGo (défaut)
./osint_toolkit search "cybersecurity tools"

# Google
./osint_toolkit search "rust programming" -e google

# Bing
./osint_toolkit search "security research" -e bing

# Avec Deep Web
./osint_toolkit search "security research" --deep
```

### Google Dorks
```bash
# Recherche de PDFs sur un domaine
./osint_toolkit dork "confidential" -d example.com -f pdf

# Recherche dans l'URL
./osint_toolkit dork "admin" --inurl admin

# Recherche dans le titre
./osint_toolkit dork "login" --intitle "admin panel"

# Recherche de fichiers sensibles
./osint_toolkit dork "password" -f txt

# Combinaison complexe
./osint_toolkit dork "database backup" -d example.com -f sql --inurl backup
```

## 👤 Profiling & Social Media

### Reconnaissance sociale
```bash
# Analyse complète avec profiling
./osint_toolkit social johndoe --analyze

# Par email
./osint_toolkit social john@example.com --id-type email

# Plateformes spécifiques
./osint_toolkit social johndoe -p "tech,gaming,social"

# Par numéro de téléphone
./osint_toolkit social +33612345678 --id-type phone
```

### GitHub OSINT
```bash
# Analyse d'utilisateur
./osint_toolkit github johndoe --repos

# Recherche de secrets
./osint_toolkit github johndoe --secrets

# Organisation
./osint_toolkit github mycompany --repos --secrets
```

### Recherche d'images inversée
```bash
# Par URL
./osint_toolkit reverse-image "https://example.com/image.jpg"

# Par fichier local
./osint_toolkit reverse-image "/path/to/image.jpg"

# Moteurs spécifiques
./osint_toolkit reverse-image "image.jpg" -e "google,yandex"
```

## 🧰 Outils Techniques

### Identification de hash
```bash
# Identifier un hash
./osint_toolkit hash "5d41402abc4b2a76b9719d911017c592" --identify

# Tenter un crack
./osint_toolkit hash "5d41402abc4b2a76b9719d911017c592" --crack

# Les deux
./osint_toolkit hash "5d41402abc4b2a76b9719d911017c592" --identify --crack
```

### Scan d'URLs
```bash
# Scan basique
./osint_toolkit urlscan "https://example.com"

# Avec screenshot
./osint_toolkit urlscan "https://example.com" --screenshot
```

### Wayback Machine
```bash
# Historique complet
./osint_toolkit wayback "https://example.com"

# Filtrer par année
./osint_toolkit wayback "https://example.com" -y 2020

# Limiter les résultats
./osint_toolkit wayback "https://example.com" -l 10
```

## 📊 Rapports et Exports

### Génération de rapports
```bash
# Rapport HTML
./osint_toolkit report "Investigation Target X" -f html -o report.html

# Rapport Markdown
./osint_toolkit report "OSINT Analysis" -f markdown -o report.md

# Rapport PDF
./osint_toolkit report "Security Assessment" -f pdf -o report.pdf
```

### Mode interactif
```bash
# Lancer le mode interactif
./osint_toolkit interactive
```

## 💡 Exemples d'investigations complètes

### Investigation d'une personne
```bash
# 1. Recherche de pseudo
./osint_toolkit username johndoe -f json -o johndoe_profiles.json

# 2. Vérification email (si trouvé)
./osint_toolkit email john.doe@example.com --breaches --social

# 3. Analyse téléphone (si trouvé) 
./osint_toolkit phone +33612345678 --carrier

# 4. Social media avec analyse
./osint_toolkit social johndoe --analyze

# 5. GitHub OSINT
./osint_toolkit github johndoe --repos --secrets
```

### Investigation d'une organisation
```bash
# 1. Analyse du domaine principal
./osint_toolkit domain example.com --dns --ssl --whois

# 2. Énumération sous-domaines
./osint_toolkit subdomain example.com --crt --brute

# 3. Recherche Shodan
./osint_toolkit shodan "ssl:example.com" --vulns

# 4. Google Dorks
./osint_toolkit dork "" -d example.com -f pdf
./osint_toolkit dork "confidential" -d example.com
./osint_toolkit dork "" -d example.com --inurl admin

# 5. GitHub de l'organisation
./osint_toolkit github example-org --repos --secrets
```

### Investigation technique d'une IP
```bash
# 1. Analyse IP complète
./osint_toolkit ip 192.168.1.100 --reverse --asn --geo

# 2. Scan de ports
./osint_toolkit portscan 192.168.1.100 -p "1-65535"

# 3. WHOIS
./osint_toolkit whois 192.168.1.100

# 4. Recherche Shodan
./osint_toolkit shodan "192.168.1.100"
```

## ⚖️ Bonnes pratiques

1. **Toujours obtenir l'autorisation** avant de scanner des systèmes
2. **Respecter les rate limits** des APIs 
3. **Utiliser un proxy** pour l'anonymat si nécessaire
4. **Sauvegarder les résultats** en JSON/CSV pour analyse ultérieure
5. **Générer des rapports** pour documenter les investigations
6. **Vérifier la configuration** avant les investigations importantes

## 🔧 Dépannage

### Erreurs communes
```bash
# Tester la configuration
./osint_toolkit config test

# Vérifier les clés API
./osint_toolkit config list

# Tester la connectivité
./osint_toolkit ip 8.8.8.8 --geo
```

### Performance
```bash
# Réduire le nombre de threads si timeout
./osint_toolkit config set-threads 5

# Augmenter le timeout (modifier src/config/mod.rs)
# timeout: 60 // secondes
```