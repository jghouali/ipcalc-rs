# ipcalc-rs
Calculateur de sous-réseaux IPv4/IPv6 en ligne de commande, écrit en Rust. Inspiré de l’outil classique `ipcalc`.
## Fonctionnalités
- Parsing d’adresses **IPv4** et **IPv6** (notation CIDR `adresse/masque`)
- Affichage de l’adresse cible
- Calcul optionnel de :
  - adresse réseau
  - broadcast
  - masque
  - première adresse utilisable (hostmin)
  - dernière adresse utilisable (hostmax)
  - nombre d’hôtes (count)
## Prérequis
- [Rust](https://www.rust-lang.org/tools/install)
## Installation
```bash
git clone <url-du-repo>
cd ipcalc-rs
cargo build --release
Le binaire se trouve dans target/release/ipcalc-rs.
```
Utilisation
```bash
# Affiche uniquement l’adresse
ipcalc-rs 192.168.1.10/24
# Affiche le réseau et le broadcast
ipcalc-rs -n -b 192.168.1.10/24
# IPv6
ipcalc-rs -n -m 2001:db8::1/64
#Options
-n --network
Adresse réseau
-b --broadcast
Adresse broadcast (IPv4)
-m --mask
Masque de sous-réseau
-i --hostmin
Première adresse hôte utilisable
-a --hostmax
Dernière adresse hôte utilisable
-c --count
Nombre d’hôtes dans le sous-réseau
```
L’adresse IP (avec ou sans /cidr) doit être passée en argument positionnel. Sans CIDR, le masque par défaut est /32 (IPv4) ou /128 (IPv6).
```bash
Exemple de sortie
$ ipcalc-rs -n -b -m 10.0.0.1/24
10.0.0.1/24
Network: 10.0.0.0/24
Broadcast: 10.0.0.255/24
Mask: 255.255.255.0/24
```
## Auteur
Jeremy Ghouali — jghouali@gmail.com

## Licence
MIT
