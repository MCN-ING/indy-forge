# Publishing Tool Guide / Guide de l'outil de Publication

## About / À propos

The Publishing Tool in IndyForge allows you to create and submit various transactions to an Indy ledger, with options to
prepare, sign, and verify transactions before submission.

L'outil de Publication dans IndyForge vous permet de créer et de soumettre diverses transactions au registre Indy, avec
des options pour préparer, signer et vérifier les transactions avant la soumission.

## Features / Fonctionnalités

- NYM transaction creation and registration / Création et enregistrement de transactions NYM
- Schema creation and publishing / Création et publication de schémas
- Custom transaction handling / Gestion des transactions personnalisées
- Transaction preparation and review / Préparation et révision des transactions
- Optional transaction signing and submission / Signature et soumission optionnelles des transactions

## Transaction Types / Types de Transactions

### NYM Transactions / Transactions NYM

1. Enter NYM Details / Saisissez les détails du NYM:
    - DID: The DID to be registered / Le DID à enregistrer
    - Verkey: The verification key / La clé de vérification
    - Alias (optional): A human-readable name / Un nom lisible (optionnel)
    - Role: Select from Author, Endorser, Network Monitor, Steward, or Trustee / Sélectionnez parmi Author, Endorser,
      Network Monitor, Steward, ou Trustee

2. Transaction Options / Options de Transaction:
    - Sign Transaction: Generate cryptographic signature / Générer une signature cryptographique
    - Send to Ledger: Submit to the network / Soumettre au réseau

3. Review and Submit / Révision et Soumission:
    - Verify transaction details / Vérifier les détails de la transaction
    - Copy transaction for external use / Copier la transaction pour usage externe
    - Submit when ready / Soumettre quand prêt

### Schema Transactions / Transactions de Schéma

1. Define Schema / Définir le Schéma:
    - Name: Schema identifier / Identifiant du schéma
    - Version: Format x.x.x / Format x.x.x
    - Attributes: Add/remove as needed / Ajouter/supprimer selon les besoins

2. Preparation / Préparation:
    - Review schema structure / Réviser la structure du schéma
    - Verify attribute list / Vérifier la liste des attributs
    - Choose signing and submission options / Choisir les options de signature et de soumission

### Custom Transactions / Transactions Personnalisées

1. Input Transaction / Saisie de la Transaction:
    - Paste prepared transaction JSON / Coller le JSON de la transaction préparée
    - Verify format and content / Vérifier le format et le contenu

2. Options / Options:
    - Sign: Add cryptographic signature / Ajouter une signature cryptographique
    - Submit: Send to ledger / Envoyer au registre
    - Preview: Review without sending / Réviser sans envoyer

## Transaction Options / Options de Transaction

For all transaction types / Pour tous les types de transactions:

- **Sign Transaction / Signer la Transaction**:
    - When enabled: Transaction will be cryptographically signed / Activé : La transaction sera signée
      cryptographiquement
    - When disabled: Transaction prepared without signature / Désactivé : Transaction préparée sans signature

- **Send to Ledger / Envoyer au Registre**:
    - When enabled: Transaction is submitted to the network / Activé : La transaction est soumise au réseau
    - When disabled: Transaction is prepared but not sent / Désactivé : La transaction est préparée mais pas envoyée

## Best Practices / Bonnes Pratiques

1. Transaction Review / Révision des Transactions:
    - Always review transactions before signing / Toujours réviser les transactions avant de signer
    - Verify all fields are correct / Vérifier que tous les champs sont corrects
    - Check role permissions / Vérifier les permissions des rôles

2. Testing / Tests:
    - Use preparation mode for verification / Utiliser le mode préparation pour la vérification
    - Test on development networks first / Tester d'abord sur les réseaux de développement
    - Keep track of transaction responses / Garder une trace des réponses des transactions

3. Security / Sécurité:
    - Verify signing authority / Vérifier l'autorité de signature
    - Double-check role assignments / Revérifier les attributions de rôles
    - Maintain proper key management / Maintenir une bonne gestion des clés

## Troubleshooting / Dépannage

Common issues / Problèmes courants:

- Invalid DID format: Check DID syntax / Format DID invalide : Vérifier la syntaxe du DID
- Role permission errors: Verify authority / Erreurs de permission de rôle : Vérifier l'autorité
- Network issues: Check connection status / Problèmes réseau : Vérifier l'état de la connexion
- Transaction rejections: Review format and permissions / Rejets de transactions : Réviser le format et les permissions