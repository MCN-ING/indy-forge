# Wallet Tool Guide / Guide de l'outil Wallet

## About / À propos

The Wallet Tool in IndyForge provides functionality for creating and managing DIDs (Decentralized Identifiers) for use
with Indy networks.

L'outil Wallet dans IndyForge fournit des fonctionnalités pour créer et gérer les DID (Identifiants Décentralisés) pour
une utilisation avec les réseaux Indy.

## Features / Fonctionnalités

- Create temporary wallets with DIDs / Créer des wallets temporaires avec des DID
- Support for both DID:SOV and DID:INDY formats / Prise en charge des formats DID:SOV et DID:INDY
- Secure seed-based DID generation / Génération sécurisée de DID basée sur une seed
- Genesis file management for network connectivity / Gestion des fichiers genesis pour la connectivité réseau

## Usage / Utilisation

### Creating a Wallet / Création d'un Wallet

1. Enter a 32-byte seed in the seed input field / Entrez une seed de 32 octets dans le champ de saisie
    - This seed will be used to generate your DID / Cette seed sera utilisée pour générer votre DID
    - The same seed will always generate the same DID / La même seed générera toujours le même DID
    - Keep this seed secure as it controls the DID / Gardez cette seed en sécurité car elle contrôle le DID

2. Select DID Version / Sélectionnez la version du DID
    - SOV: Traditional Sovrin DID format / Format DID Sovrin traditionnel
    - Indy: Newer Indy DID format / Nouveau format DID Indy
    - Choose based on your network requirements / Choisissez selon les exigences de votre réseau

3. Click "Create Wallet" / Cliquez sur "Create Wallet"
    - The tool will generate and display your DID and Verkey / L'outil générera et affichera votre DID et Verkey
    - This information will be used for ledger operations / Ces informations seront utilisées pour les opérations sur le
      registre

### Connecting to a Network / Connexion à un Réseau

1. Select Genesis File Source / Sélectionnez la source du fichier Genesis:
    - Local File: Use "Select Local Genesis File" to choose a file from your system / Fichier local : Utilisez "Select
      Local Genesis File" pour choisir un fichier depuis votre système
    - URL: Enter a genesis file URL and click "Load URL" / URL : Entrez l'URL d'un fichier genesis et cliquez sur "Load
      URL"

2. Verify Connection / Vérifiez la Connexion
    - The active genesis source will be displayed / La source genesis active sera affichée
    - A successful connection is required for ledger operations / Une connexion réussie est nécessaire pour les
      opérations sur le registre

## Best Practices / Bonnes Pratiques

1. Seed Management / Gestion des Seeds
    - Use unique seeds for different DIDs / Utilisez des seeds uniques pour différents DID
    - Store seeds securely / Stockez les seeds de manière sécurisée
    - Never share your seeds / Ne partagez jamais vos seeds

2. Network Selection / Sélection du Réseau
    - Ensure your genesis file matches your target network / Assurez-vous que votre fichier genesis correspond à votre
      réseau cible
    - Verify network compatibility before operations / Vérifiez la compatibilité du réseau avant les opérations
    - Keep genesis files up to date / Gardez les fichiers genesis à jour

## Troubleshooting / Dépannage

Common issues and solutions / Problèmes courants et solutions:

- Invalid seed length: Ensure exactly 32 bytes / Longueur de seed invalide : Assurez-vous d'avoir exactement 32 octets
- Genesis file errors: Verify file format and network availability / Erreurs de fichier genesis : Vérifiez le format du
  fichier et la disponibilité du réseau
- Connection issues: Check network connectivity and genesis file validity / Problèmes de connexion : Vérifiez la
  connectivité réseau et la validité du fichier genesis