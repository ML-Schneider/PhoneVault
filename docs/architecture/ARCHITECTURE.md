# PhoneVault Architecture

**Version:** 1.0 Draft
**Creator:** Malachi Schneider

---

# Philosophy

PhoneVault is a local-first digital preservation platform.

Its purpose is to help people permanently preserve, organize, verify, and own their digital lives without requiring subscriptions, cloud services, or vendor lock-in.

The software prioritizes:

1. Data integrity
2. User ownership
3. Human-readable archives
4. Offline functionality
5. Long-term accessibility

---

# Design Principles

## Preserve First

PhoneVault never modifies source data during preservation.

Workflow:

Read

↓

Copy

↓

Verify

↓

Optional User Action

---

## Human Readable

The Digital Vault should remain understandable even if PhoneVault no longer exists.

Users should always be able to browse their archive directly using Finder, Windows Explorer, or Linux file managers.

---

## Local First

No account required.

No internet required for core functionality.

No analytics.

No telemetry.

---

## Verification Matters

PhoneVault never claims success until copied data has been cryptographically verified.

---

## Separation of Responsibilities

PhoneVault is divided into independent systems.

Digital Vault

Human-readable archive.

Emergency Restore Archive

Machine-readable device restoration.

Secure Identity Vault

Optional encrypted storage.

Each system has one responsibility.

---

# Project Architecture

PhoneVault/

apps/

desktop/

Native desktop application.

crates/

phonevault-core/

Archive engine.

phonevault-device/

Device communication.

phonevault-integrity/

Hashing and verification.

phonevault-organizer/

Metadata and categorization.

phonevault-reports/

Reporting engine.

docs/

Project documentation.

tests/

Integration tests.

scripts/

Development utilities.

assets/

Icons, branding, screenshots.

---

# Core Components

## Archive Engine

Responsible for:

Scanning

Copying

Organization

Verification

Manifest generation

---

## Device Engine

Responsible for:

Device detection

Inventory

Import

Connection status

Future Android support

---

## Integrity Engine

Responsible for:

SHA-256 hashing

Verification

Resume support

Health reports

Duplicate detection

---

## Organizer

Responsible for:

Metadata extraction

Timeline generation

Folder organization

File categorization

---

## Report Engine

Responsible for:

Archive receipts

Verification reports

JSON exports

PDF summaries

---

# Future Modules

Vault Explorer

Windows Support

Linux Support

CLI

Plugin API

Cloud Connectors (optional)

---

# Mission Statement

Own your memories.

Not a subscription.