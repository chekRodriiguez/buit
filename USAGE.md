# 🔍 OSINT Toolkit - Guide d'utilisation

## Configuration initiale

### Configurer le proxy
```bash
# Proxy simple
./buit config set-proxy http://proxy.example.com:8080

# Proxy avec authentification
./buit config set-proxy http://proxy.example.com:8080 -u username -p password
w

### Configurer l'User-Agent
```bash
# Presets disponibles
./buit config set-user-agent chrome    # Chrome (défaut)
./buit config set-user-agent firefox   # Firefox
./buit config set-user-agent safari    # Safari
./buit config set-user-agent edge      # Edge
./buit config set-user-agent mobile    # Mobile
./buit config set-user-agent osint       # Osint profile 

# User-Agent personnalisé
./buit config set-user-agent "Mon User Agent Custom"
```

### Configurer les threads
```bash
./buit config set-threads 20
```

### Ajouter des clés API
```bash
./buit config set-key shodan YOUR_SHODAN_API_KEY
./buit config set-key github YOUR_GITHUB_TOKEN
./buit config set-key hibp YOUR_HIBP_KEY
```

### Voir la configuration
```bash
./buit config list
```

## 🔎 Recherches & Analyse

### Recherche de pseudos
```bash
# Recherche basique
./buit username johndoe

# Filtrer les plateformes
./buit username johndoe -p "github,twitter,linkedin"

# Export en JSON
./buit username johndoe -f json -o results.json

# Export en CSV
./buit username johndoe -f csv -o results.csv
```

### Analyse d'emails
```bash
# Vérification basique
./buit email john.doe@example.com

# Avec recherche de fuites et réseaux sociaux
./buit email john.doe@example.com --breaches --social

# Export formaté
./buit email john.doe@example.com --social -f json
```

### Analyse de numéros de téléphone
```bash
# Analyse basique
./buit phone +33612345678

# Avec informations opérateur
./buit phone +33612345678 --carrier

# Export JSON
./buit phone +33612345678 --carrier -f json
```

### Analyse IP
```bash
# Analyse complète
./buit ip 8.8.8.8 --reverse --asn --geo

# Reverse DNS uniquement
./buit ip 8.8.8.8 --reverse

# Géolocalisation uniquement  
./buit ip 8.8.8.8 --geo
```

### Analyse de domaines
```bash
# Analyse complète
./buit domain example.com --dns --ssl --whois

# DNS uniquement
./buit domain example.com --dns
```

### Vérification de fuites
```bash
# HaveIBeenPwned
./buit leaks john@example.com --hibp

# Avec recherche de mots de passe
./buit leaks johndoe --hibp --passwords
```

### Extraction de métadonnées
```bash
# Analyser un fichier
./buit metadata /path/to/file.jpg

# Export JSON
./buit metadata /path/to/document.pdf -f json
```

## 🌍 Réseaux & Infrastructure

### Énumération de sous-domaines
```bash
# Certificate Transparency
./buit subdomain example.com --crt

# Brute force DNS
./buit subdomain example.com --brute

# Les deux méthodes
./buit subdomain example.com --crt --brute
```

### Recherche Shodan
```bash
# Recherche basique
./buit shodan "apache"

# Avec vulnérabilités
./buit shodan "apache" --vulns

# Limiter les résultats
./buit shodan "apache" -l 50
```

### Scan de ports
```bash
# Scan complet
./buit portscan 192.168.1.1

# Plage de ports spécifique
./buit portscan 192.168.1.1 -p "1-1000"

# Type de scan
./buit portscan 192.168.1.1 --scan-type tcp
```

### WHOIS
```bash
# Lookup basique
./buit whois example.com

# Avec parsing
./buit whois example.com --parse

# IP
./buit whois 8.8.8.8
```

### GeoIP
```bash
# Géolocalisation basique
./buit geoip 8.8.8.8

# Avec informations ISP
./buit geoip 8.8.8.8 --isp
```

## 🔍 Recherche Web

### Moteurs de recherche
```bash
# DuckDuckGo (défaut)
./buit search "cybersecurity tools"

# Google
./buit search "rust programming" -e google

# Bing
./buit search "security research" -e bing

# Avec Deep Web
./buit search "security research" --deep
```

### Google Dorks
```bash
# Recherche de PDFs sur un domaine
./buit dork "confidential" -d example.com -f pdf

# Recherche dans l'URL
./buit dork "admin" --inurl admin

# Recherche dans le titre
./buit dork "login" --intitle "admin panel"

# Recherche de fichiers sensibles
./buit dork "password" -f txt

# Combinaison complexe
./buit dork "database backup" -d example.com -f sql --inurl backup
```

## 👤 Profiling & Social Media

### Reconnaissance sociale
```bash
# Analyse complète avec profiling
./buit social johndoe --analyze

# Par email
./buit social john@example.com --id-type email

# Plateformes spécifiques
./buit social johndoe -p "tech,gaming,social"

# Par numéro de téléphone
./buit social +33612345678 --id-type phone
```

### GitHub OSINT
```bash
# Analyse d'utilisateur
./buit github johndoe --repos

# Recherche de secrets
./buit github johndoe --secrets

# Organisation
./buit github mycompany --repos --secrets
```

### Recherche d'images inversée
```bash
# Par URL
./buit reverse-image "https://example.com/image.jpg"

# Par fichier local
./buit reverse-image "/path/to/image.jpg"

# Moteurs spécifiques
./buit reverse-image "image.jpg" -e "google,yandex"
```

## 🧰 Outils Techniques

### Identification de hash
```bash
# Identifier un hash
./buit hash "5d41402abc4b2a76b9719d911017c592" --identify

# Tenter un crack
./buit hash "5d41402abc4b2a76b9719d911017c592" --crack

# Les deux
./buit hash "5d41402abc4b2a76b9719d911017c592" --identify --crack
```

### Scan d'URLs
```bash
# Scan basique
./buit urlscan "https://example.com"

# Avec screenshot
./buit urlscan "https://example.com" --screenshot
```

### Wayback Machine
```bash
# Historique complet
./buit wayback "https://example.com"

# Filtrer par année
./buit wayback "https://example.com" -y 2020

# Limiter les résultats
./buit wayback "https://example.com" -l 10
```

## 📊 Rapports et Exports

### Génération de rapports
```bash
# Rapport HTML
./buit report "Investigation Target X" -f html -o report.html

# Rapport Markdown
./buit report "OSINT Analysis" -f markdown -o report.md

# Rapport PDF
./buit report "Security Assessment" -f pdf -o report.pdf
```

### Mode interactif
```bash
# Lancer le mode interactif
./buit interactive
```

## 💡 Exemples d'investigations complètes

### Investigation d'une personne
```bash
# 1. Recherche de pseudo
./buit username johndoe -f json -o johndoe_profiles.json

# 2. Vérification email (si trouvé)
./buit email john.doe@example.com --breaches --social

# 3. Analyse téléphone (si trouvé) 
./buit phone +33612345678 --carrier

# 4. Social media avec analyse
./buit social johndoe --analyze

# 5. GitHub OSINT
./buit github johndoe --repos --secrets
```

### Investigation d'une organisation
```bash
# 1. Analyse du domaine principal
./buit domain example.com --dns --ssl --whois

# 2. Énumération sous-domaines
./buit subdomain example.com --crt --brute

# 3. Recherche Shodan
./buit shodan "ssl:example.com" --vulns

# 4. Google Dorks
./buit dork "" -d example.com -f pdf
./buit dork "confidential" -d example.com
./buit dork "" -d example.com --inurl admin

# 5. GitHub de l'organisation
./buit github example-org --repos --secrets
```

### Investigation technique d'une IP
```bash
# 1. Analyse IP complète
./buit ip 192.168.1.100 --reverse --asn --geo

# 2. Scan de ports
./buit portscan 192.168.1.100 -p "1-65535"

# 3. WHOIS
./buit whois 192.168.1.100

# 4. Recherche Shodan
./buit shodan "192.168.1.100"
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
./buit config test

# Vérifier les clés API
./buit config list

# Tester la connectivité
./buit ip 8.8.8.8 --geo
```

### Performance
```bash
# Réduire le nombre de threads si timeout
./buit config set-threads 5

# Augmenter le timeout (modifier src/config/mod.rs)
# timeout: 60 // secondes
```