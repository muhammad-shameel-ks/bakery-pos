# Indian Legal Compliance Plan - A Plus Invoicing

Last updated: 2026-07-28

---

## Legal Context

This document outlines the compliance requirements for **A Plus Invoicing** (a desktop POS application for bakeries/restaurants) under Indian law.

### Business Model

- Offline desktop POS application (Tauri + React + Rust + SQLite)
- Sold as a product to bakeries/restaurants
- No cloud/SaaS layer currently (future possibility)

### Key Distinction

The **bakery** using this software is the "Data Fiduciary" under the DPDP Act
(they collect and process customer/supplier data).
We are the **software vendor / tool maker**.
We do NOT access or process our customers' data.
However, we must build compliance capabilities INTO the software
so our customers can be DPDP-compliant, and we have our own obligations
under the IT Rules 2021.

---

## Applicable Laws

| Law | Status | Key Requirements |
|-----|--------|------------------|
| **Digital Personal Data Protection Act, 2023 (DPDP Act)** | Phased enforcement - Full compliance by **May 13, 2027** | Consent, privacy notice, security safeguards, breach notification, data retention/deletion |
| **IT (Intermediary Guidelines & Digital Media Ethics Code) Rules, 2021** | In force (amended Feb 2026) | Terms of Use, Privacy Policy, Grievance Officer, content takedown |
| **IT Act, 2000** | In force (to be replaced by Digital India Act) | Intermediary liability, cyber offences, CERT-In directions |
| **CERT-In Directions (April 28, 2022)** | In force | 6-hour cybersecurity incident reporting for certain entities |
| **GST Law** | In force | GST-compliant invoicing, HSN codes, tax slabs - **already implemented** |
| **Digital India Act (proposed)** | Consultation phase - not yet introduced | Will replace IT Act - monitor for updates |

### Critical Deadline

**May 13, 2027** - All substantive DPDP Act provisions become enforceable.

---

## Compliance Work Plan

### Phase 1: Legal Documentation (DO BEFORE FIRST SALE)

| Item | Description | Law |
|------|-------------|-----|
| **End User License Agreement (EULA)** | Define terms of use, prohibited uses, termination, liability caps, disclaimers | IT Rules 2021 Rule 3(1)(a) |
| **Privacy Policy** | Explain what data the app collects (none if no telemetry), how it's stored (local only), and data subjects' rights | DPDP Act Sec 5, IT Rules 2021 |
| **In-app Privacy Notice** | Shown on first launch - states "All data stays on your device. We do not access or transmit your data." | DPDP Act Sec 5 |
| **Grievance Officer** | Designate a person with name, email, address. Published in app and website. Acknowledge complaints in 24h, resolve in 15 days (7 days under 2026 amendment) | IT Rules 2021 Rule 3(2)(a) |
| **DPDP Notice Template for Customers** | A template privacy notice your bakery customers can customize for their own end customers | DPDP Act Sec 5 |
| **Data Retention Policy** | Document specifying what data is retained, for how long, and deletion procedure | DPDP Act Sec 8(7) |

#### Task Checklist

- [ ] Draft EULA
- [ ] Draft Privacy Policy
- [ ] Build in-app privacy notice component (first-launch dialog)
- [ ] Add Grievance Officer details in app Settings / About page
- [ ] Create DPDP Notice template for distribution to bakery customers
- [ ] Document data retention policy

---

### Phase 2: Database & Storage Changes

| Item | Change Required | Details |
|------|----------------|---------|
| **Database encryption** | Migrate from plain SQLite to SQLCipher | `db.rs`: `Connection::open` -> `Connection::open_with_cipher`. Encrypts the local `.db` file at rest using a key derived from the app password. |
| **Application-level auth** | Add login/password screen on first launch | Prevents unauthorized physical access to data. Admin sets a master password. |
| **GSTIN validation** | Validate format before storing | Regex: `^[0-9]{2}[A-Z]{5}[0-9]{4}[A-Z]{1}[1-9A-Z]{1}Z[0-9A-Z]{1}$` |
| **Phone/email validation** | Basic format validation for contact fields | |
| **Audit log table** | New table: `audit_log` | Fields: `id, timestamp, user_id, action, table_name, record_id, old_values, new_values` |
| **Consent record table** | New table: `consent_records` | Fields: `id, data_principal_id, purpose, consent_given_at, consent_withdrawn_at, consent_version` |
| **Data retention purge column** | Add `retention_purge_at` to transaction tables | Background cleanup job on app startup |
| **Structured customer data** | Expand `retail_sale_headers.customer_name` to allow optional phone/ID fields | Enables data subject rights (correction, erasure, portability) |

#### Task Checklist

- [ ] Add SQLCipher dependency to `Cargo.toml`
- [ ] Rewrite `db.rs` to use SQLCipher with key derivation
- [ ] Create auth system (Rust-side password hashing with argon2)
- [ ] Add GSTIN validation in `commands.rs`
- [ ] Add phone/email validation
- [ ] Create `audit_log` table and recording hooks in commands
- [ ] Create `consent_records` table
- [ ] Build retention scheduler (check on app startup)
- [ ] Expand customer data model for retail sales

#### Migration Path (for existing users)

The first launch after upgrade should:
1. Prompt for admin password (creates encryption key)
2. Re-encrypt existing SQLite database with new key
3. Show privacy notice
4. Create new tables (audit_log, consent_records)

---

### Phase 3: Application-Level Changes

| Item | Change Required | Details |
|------|----------------|---------|
| **First-launch wizard** | Show privacy notice + consent. Set up admin password. Configure data retention period. | |
| **User authentication** | Login screen. Rust-side password hashing (argon2). Store password hash, encrypt DB key with it. | |
| **Multi-user roles (preparation)** | Prepare role system: Admin (full access), Cashier (POS only). Gate Tauri IPC permissions. | |
| **"Clear My Data" / Deletion** | "Delete transactions older than X years" + "Wipe all data and reset" | DPDP Act Sec 8(7) |
| **Data export (machine-readable)** | "Export My Data" button - exports all data as structured JSON/CSV | DPDP Act right to data portability |
| **Data correction UI** | Allow editing personal/business data. Add "Correct My Data" flow. | DPDP Act right to correction |
| **Audit trail recording** | Log all CRUD operations on business_parties, bakery_profile, settings | DPDP Act accountability |
| **Secure backup/restore** | Export DB in encrypted format, not plaintext | |
| **Content Security Policy** | Fix `tauri.conf.json` - set proper CSP instead of `null` | Tauri security best practice |

#### Task Checklist

- [ ] Build first-launch wizard component (React)
- [ ] Implement auth system (Rust + React)
- [ ] Design and implement role/permission system (even if single-user initially)
- [ ] Add "Delete old data" feature with date range
- [ ] Add "Export My Data" feature (JSON/CSV)
- [ ] Ensure all data fields are editable for correction
- [ ] Wire audit logging into all command handlers
- [ ] Build encrypted backup/restore
- [ ] Fix CSP in tauri.conf.json

---

### Phase 4: DPDP Act Full Readiness (by May 2027)

| Item | Details |
|------|---------|
| **Consent management UI** | For bakeries to record consent from their B2B suppliers/customers. Include consent form, withdrawal mechanism, re-consent reminders. |
| **Data Protection Impact Assessment (DPIA)** | Prepare a DPIA template that your bakery customers can use. Document data flows. |
| **Data mapping exercise** | Document every data element, its purpose, processing activity, retention period, lawful basis. |
| **Breach detection & notification** | Integrity check on DB startup. Template for breach notification to DPB + affected data principals. 72-hour notification requirement. |
| **Significant Data Fiduciary readiness** | If bakery customers cross DPDP Act thresholds, provide tooling: DPO designation, data auditor access, compliance reports. |

#### Task Checklist

- [ ] Build consent management feature (UI, storage, withdrawal)
- [ ] Document DPIA template
- [ ] Complete data mapping document
- [ ] Build DB integrity checker on startup
- [ ] Create breach notification templates
- [ ] Build compliance reporting for SDF customers

---

### Phase 5: Future Cloud/SaaS Migration Prep

| Item | Recommendation |
|------|---------------|
| **Abstract storage layer** | Use a trait/interface for data access. Swap SQLite for PostgreSQL later without rewriting all queries. |
| **Multi-tenant data isolation** | Add `tenant_id` column to all tables now (even if unused). Design for tenant-per-schema or tenant-per-row. |
| **API architecture** | Structure Tauri commands to mirror REST API patterns for future cloud migration. |
| **Encryption** | Already doing encryption at rest (Phase 2). For cloud: add TLS in transit. |
| **Data localization** | Ensure servers in India for cloud hosting. DPDP Act "blacklist" model but safest to keep data in India. |
| **Consent manager integration** | Prepare API hooks to integrate with registered Consent Managers under DPDP Act. |

#### Task Checklist

- [ ] Add `tenant_id` column to all tables
- [ ] Create storage trait/interface in Rust
- [ ] Structure commands to mirror REST patterns
- [ ] Research cloud hosting options in India

---

### Phase 6: Ongoing Compliance

| Item | Frequency | Details |
|------|-----------|---------|
| Regulatory monitoring | Quarterly | Track DPDP Rules amendments, Digital India Act progress, CERT-In updates |
| Compliance audit | Annual | Review documentation, code, data handling practices |
| Penetration testing | Annual | Security assessment of the application |
| Update legal docs | When laws change | Keep EULA, Privacy Policy current |
| User communication | When material changes occur | Notify users of compliance updates |

---

## Penalty Risk Assessment

For the current offline model (selling desktop app, no cloud):

| Risk | Scenario | Max Penalty |
|------|----------|-------------|
| **Low** | App has no auth, no encryption. If device is stolen, data exposed. | Up to Rs. 250 crore (failure to implement reasonable security safeguards - DPDP Act) |
| **Low** | No privacy notice or consent mechanism built into the app | Up to Rs. 50 crore (failure to provide notice - DPDP Act) |
| **Medium** | Selling the app without EULA / ToS / Privacy Policy | IT Act penalties + loss of safe harbour under Section 79 |

If the product moves to cloud/SaaS, risk increases substantially
(you become a Data Fiduciary/Processor yourself with full DPDP Act obligations).

---

## Priority Summary

### Must Do (Before First Sale)

1. EULA, Privacy Policy, Privacy Notice
2. Grievance Officer designation
3. Database encryption (SQLCipher)
4. Application authentication
5. First-launch privacy notice wizard
6. Fix CSP in tauri.conf.json

### Should Do (Before May 2027)

7. Data export / deletion features
8. Consent management UI + storage
9. Audit logging
10. GSTIN/phone validation
11. Retention scheduler
12. Data mapping documentation

### Nice to Have (Future)

13. Multi-user roles
14. Cloud migration preparation (tenant_id, storage abstraction)
15. Consent manager integration
16. DPIA automation tooling

---

## References

- DPDP Act 2023: https://www.dpdpa.com/
- IT Rules 2021 (MeitY): https://www.meity.gov.in/
- CERT-In Directions 2022: https://www.cert-in.org.in/
- DPDP Rules 2025: https://www.pib.gov.in/PressReleasePage.aspx?PRID=2190014
- IT Intermediary Amendment Rules 2026: https://www.argus-p.com/updates/updates/update-overview-of-the-it-intermediary-amendment-rules-2026/
