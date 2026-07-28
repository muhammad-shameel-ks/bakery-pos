# Legal Guide for Building POS / Bakery Software in India

Not for business owners.
For developers building such applications.

---

## What This Covers

Every Indian law that touches a POS or bakery management application.
What you must build into your software to be compliant.
Not why. Just what.

---

## Table of Laws That Apply

| Law | Status | What It Regulates |
|-----|--------|-------------------|
| **DPDP Act 2023** | Full enforcement from **May 14, 2027** | Personal data of customers, suppliers, employees |
| **DPDP Rules 2025** | Phased - Phase 3 from **May 14, 2027** | Operational details of consent, breach, retention |
| **IT Act 2000** | In force (to be replaced by DIA) | Cyber offences, intermediary liability, CERT-In |
| **IT Rules 2021** (as amended Feb 2026) | In force | Grievance officer, ToS, privacy policy, takedown |
| **GST Act 2017** | In force | Invoice format, HSN codes, tax slabs |
| **FSSAI Act 2006** | In force | FSSAI license display for food businesses |
| **RBI Payment Circular 2018** | In force | Payment data must be stored only in India |
| **CERT-In Directions 2022** | In force | 6-hour cybersecurity incident reporting |
| **Digital India Act** | Not yet introduced - in consultation | Will replace IT Act 2000 (monitor) |

---

## 2027: The Big Enforcement Year

Three major things happen in 2027 that change everything for POS software.

### 1. DPDP Act Full Enforcement - May 14, 2027

**Source**: DPDP Rules 2025, Phase 3 timeline
[https://en.wikipedia.org/wiki/Digital_Personal_Data_Protection_Rules,_2025](https://en.wikipedia.org/wiki/Digital_Personal_Data_Protection_Rules,_2025)

After this date the Data Protection Board can impose penalties up to Rs. 250 crore.
Everything below in the "DPDP Act Requirements" section becomes enforceable.

**Phase timeline**:

| Date | What Happens |
|------|-------------|
| Nov 14, 2025 | Phase 1 - Data Protection Board established |
| Nov 14, 2026 | Phase 2 - Consent Manager rules effective |
| **May 14, 2027** | **Phase 3 - ALL substantive provisions + penalties** |

### 2. First SDF Audit Cycle - Q1 2027

Significant Data Fiduciaries (entities with >5M users or >Rs. 250Cr turnover
processing personal data) must complete their first mandatory:
- Independent data audit
- Data Protection Impact Assessment (DPIA)
- DPO appointment (based in India)

**Source**: DPDP Act Section 10, DPDP Rules - SDF obligations
[https://www.dlapiperdataprotection.com/?t=law&c=IN](https://www.dlapiperdataprotection.com/?t=law&c=IN)

### 3. Soft Enforcement Ends - Full Adjudication Begins

The Data Protection Board shifts from guidance/warnings to active enforcement.
Penalties become real. Breach notification becomes mandatory.

**Source**: India Briefing DPDP Timeline Analysis
[https://www.india-briefing.com/news/india-dpdp-compliance-timeline-enforcement-2026-27-44740.html/](https://www.india-briefing.com/news/india-dpdp-compliance-timeline-enforcement-2026-27-44740.html/)

### Digital India Act

Still in consultation. Not introduced in Parliament.
No set date. Will replace IT Act 2000 entirely.
Covers platform regulation, AI, cybercrime modernization.
Monitor but do not wait for it.

**Source**: India AI Rulebook
[https://indiaairulebook.com/learn/platforms-internet/digital-india-act](https://indiaairulebook.com/learn/platforms-internet/digital-india-act)

---

## DPDP Act Requirements (Build These Into Your Software)

Your bakery customer is the "Data Fiduciary."
They collect data from their end customers, suppliers, employees.
Your software must give them the tools to be compliant.

### 1. Privacy Notice Display

Your software must let the bakery display a privacy notice.
Shown before collecting any personal data.
Must include:
- What data is being collected
- Why it is being collected
- Contact details of the person handling data
- How to withdraw consent
- How to file a complaint

Must be available in English or any of 22 Indian languages if requested.

**Source**: DPDP Act Section 5, DPDP Rules Rule 4
[https://www.meity.gov.in/static/uploads/2024/06/2bf1f0e9f04e6fb4f8fef35e82c42aa5.pdf](https://www.meity.gov.in/static/uploads/2024/06/2bf1f0e9f04e6fb4f8fef35e82c42aa5.pdf)

### 2. Consent Collection

Your software must record:
- Who gave consent
- What they consented to
- When they consented
- If/when they withdrew consent

Consent must be free, specific, informed, unconditional, unambiguous,
and through clear affirmative action.

Withdrawal must be as easy as giving consent.

**Source**: DPDP Act Section 6
[https://www.dpdpa.com/dpdpa2023/chapter-2/section6.html](https://www.dpdpa.com/dpdpa2023/chapter-2/section6.html)

### 3. Data Retention Controls

Your software must let the bakery set retention periods.
Automatically delete or flag data when the purpose is served.
Must cease to retain when:
- Purpose is no longer being served
- Data principal withdraws consent
- Upon erasure request

Unless retention is required by law (GST records: 72 months,
Income Tax: 6-7 years, Companies Act: 8 years).

**Source**: DPDP Act Section 8(7), KSK Law Analysis
[https://ksandk.com/data-protection-and-data-privacy/data-retention-and-deletion-under-indias-dpdp-rules/](https://ksandk.com/data-protection-and-data-privacy/data-retention-and-deletion-under-indias-dpdp-rules/)

### 4. Data Subject Rights Portal

Your software must let data subjects exercise these rights:
- **Access**: See what data is stored about them
- **Correction**: Fix incorrect data
- **Erasure**: Delete their data
- **Withdrawal**: Withdraw consent
- **Grievance**: File a complaint
- **Nomination**: Nominate someone to act after death/incapacity

**Source**: DPDP Act Chapter 3 (Sections 11-16)

### 5. Breach Notification

Your software should have:
- Integrity checking on startup
- Breach detection capability
- Templates for notifying the Data Protection Board (72-hour detailed report)
- Templates for notifying affected individuals (without delay)

**Source**: DPDP Rules Rule 7 - Breach notification
[https://www.pib.gov.in/PressReleasePage.aspx?PRID=2190014](https://www.pib.gov.in/PressReleasePage.aspx?PRID=2190014)

### 6. Security Safeguards

Your software must implement "reasonable security safeguards":
- Encrypt the database at rest (SQLCipher or equivalent)
- Application-level authentication (login/password)
- Role-based access control
- Audit logging (who did what, when)
- Secure backup/restore
- Input validation (GSTIN, phone, email formats)

**Source**: DPDP Act Section 8(5) - obligation to implement reasonable security safeguards

### 7. Consent Records for Children (Under 18)

If the bakery serves minors, your software must support:
- Verifiable parental consent mechanism
- No behavioral monitoring or targeted advertising to children

**Source**: DPDP Act Section 9

---

## IT Rules 2021 Requirements (Amended Feb 2026)

### 1. Terms of Service

Your software must have a Terms of Service / EULA that:
- Prohibits users from hosting unlawful content
- Explains consequences of violation
- Is prominently accessible within the app

### 2. Privacy Policy

Must be published and accessible within the app.
Must comply with DPDP Act notice requirements.

### 3. Grievance Officer

Designate a person with:
- Name
- Email address
- Physical address

Published within the app.
Acknowledge complaints: within 24 hours.
Resolve complaints: within 7 days (reduced from 15 by 2026 amendment).

**Source**: IT Rules 2021 Rule 3(2)(a), amended Feb 2026
[https://www.meity.gov.in/static/uploads/2026/02/550681ab908f8afb135b0ad42816a1c9.pdf](https://www.meity.gov.in/static/uploads/2026/02/550681ab908f8afb135b0ad42816a1c9.pdf)

### 4. Record Retention

Intermediaries must retain records for at least 180 days.
(Longer if required for ongoing investigations.)

**Source**: IT Rules 2021 Rule 3(1)(h)

### 5. Content Takedown

Must remove unlawful content within 3 hours upon court/government order.
(Reduced from 36 hours by 2026 amendment.)
Intimate imagery: 2 hours.

**Source**: IT Rules 2021 Rule 3(2)(b), amended Feb 2026

---

## GST Compliance (Already Standard, But Verify)

Your software must support:

### Invoice Requirements

| Requirement | Details |
|-------------|---------|
| HSN Code | Minimum 4 digits (8 digits for >Rs. 5Cr turnover) |
| Tax slabs | 0%, 5%, 12%, 18%, 28% |
| GSTIN | 15-digit alphanumeric, validate format |
| Invoice fields | Date, buyer name, address, GSTIN, HSN, quantity, rate, tax, total |
| Invoice numbering | Unique, sequential |
| Reverse charge | Track if buyer is unregistered |
| E-invoicing | Required if turnover >Rs. 5Cr |

**Source**: GST Act 2017, CBIC notifications
[https://tutorial.gst.gov.in/downloads/news/hsn_advisory_table_12_2.pdf](https://tutorial.gst.gov.in/downloads/news/hsn_advisory_table_12_2.pdf)

### E-Way Bill

Not a POS concern (logistics/transport).
But if your software ever handles delivery, e-way bill generation
is required for inter-state movement >Rs. 50,000 value.

---

## FSSAI Requirements

Every food business must display FSSAI license/registration number.
Your software should have a field for FSSAI license number
on the bakery profile and print it on invoices/bills.

**Source**: FSSAI Act 2006, FSSAI Display Board Regulations
[https://www.fssai.gov.in/cms/food-safety-display-boards.php](https://www.fssai.gov.in/cms/food-safety-display-boards.php)

---

## RBI Payment Data Localization

If your POS software stores payment card data (even temporarily):
ALL payment transaction data must be stored only in India.
Includes: customer name, card details, transaction amount, timestamp,
OTP, PIN, passwords, beneficiary account details.

If you process payments through a third-party gateway (Razorpay, PayU, etc.),
the gateway handles this. But your app should not store raw payment data.

If you ever store card numbers, CVC, or PINs: you are in violation.
PCI DSS also applies. Do not store this data.

**Source**: RBI Circular "Storage of Payment System Data" October 2018
[https://www.rbi.org.in/commonman/english/scripts/FAQs.aspx?Id=2995](https://www.rbi.org.in/commonman/english/scripts/FAQs.aspx?Id=2995)

---

## CERT-In Directions (April 28, 2022)

If your software becomes cloud-connected in the future:

- Cybersecurity incidents must be reported to CERT-In within 6 hours
- Synchronize system clocks with NTP servers
- Maintain logs for 180 days minimum

Applies to data centers, VPS providers, cloud providers, ISPs.
May apply to your SaaS if you are a cloud provider.

**Source**: CERT-In Directions 2022
[https://www.cert-in.org.in/PDF/CERT-In_Directions_70B_28.04.2022.pdf](https://www.cert-in.org.in/PDF/CERT-In_Directions_70B_28.04.2022.pdf)

---

## Intermediary Classification

If your software is sold as a desktop app (offline), you are a software vendor.
Not an intermediary under IT Act.

If your software becomes cloud/SaaS, you host data on behalf of bakeries.
You become an "intermediary" under IT Act Section 2(1)(w).
This means:
- You get safe harbour under Section 79 (conditional immunity for third-party content)
- You lose safe harbour if you initiate transmission, select receiver, modify content,
  or fail to act on court/government orders
- You must follow ALL IT Rules 2021 requirements above

**Source**: IT Act Section 2(1)(w), Section 79
[https://www.dpdpindia.in/itact-intermediary.html](https://www.dpdpindia.in/itact-intermediary.html)

---

## What You Must Build: Minimum Checklist

### Database & Storage

- [ ] Database encryption at rest (SQLCipher or equivalent)
- [ ] Encrypted backup/restore
- [ ] GSTIN format validation (15-digit: `^[0-9]{2}[A-Z]{5}[0-9]{4}[A-Z]{1}[1-9A-Z]{1}Z[0-9A-Z]{1}$`)
- [ ] Phone/email format validation
- [ ] Audit log table (who, what, when, old values, new values)
- [ ] Consent records table (who, what, when, withdrawal)
- [ ] Retention scheduling (auto-purge or flag old data)
- [ ] FSSAI license number field for food businesses
- [ ] Never store payment card numbers, CVC, or PINs

### Authentication & Access

- [ ] Application-level login (admin password)
- [ ] Password hashing (argon2 or bcrypt)
- [ ] Role-based access (Admin, Cashier, at minimum)
- [ ] Session management

### User-Facing Features

- [ ] First-launch privacy notice display
- [ ] Consent collection mechanism
- [ ] Data subject rights portal (access, correct, delete, withdraw, complain)
- [ ] Data export in machine-readable format (JSON/CSV)
- [ ] Delete all data / wipe function
- [ ] Grievance officer contact displayed in app

### Legal Documents (Ship with the app)

- [ ] End User License Agreement (EULA)
- [ ] Privacy Policy
- [ ] In-app privacy notice
- [ ] Data retention policy document
- [ ] GST-compliant invoice template
- [ ] Terms of Service

### GST Features

- [ ] HSN code field for each item (min 4 digits)
- [ ] Tax slabs: 0%, 5%, 12%, 18%, 28%
- [ ] GSTIN validation on entry
- [ ] Unique sequential invoice numbering
- [ ] Invoice fields: date, buyer name, address, GSTIN, HSN, qty, rate, tax, total
- [ ] Tax breakdown on invoice (CGST + SGST or IGST)
- [ ] Support for reverse charge mechanism
- [ ] Track purchase tax credit (ITC)

### Security

- [ ] Input sanitization on all fields
- [ ] SQL injection prevention (parameterized queries)
- [ ] CSP set properly (not null)
- [ ] No hardcoded secrets or keys
- [ ] Secure random number generation for invoice IDs

---

## Penalty Exposure

These penalties apply to your customer (the bakery) as Data Fiduciary.
But if you do not build compliance tools, they cannot comply.
Your liability depends on your contract with them.

| Violation | Max Penalty |
|-----------|-------------|
| Failure to implement reasonable security safeguards | Rs. 250 crore |
| Failure to notify breach to Board and affected individuals | Rs. 200 crore |
| Children's data obligations breach | Rs. 200 crore |
| Failure to delete data when purpose served | Rs. 50 crore |
| General non-compliance (per violation) | Up to Rs. 250 crore |

**Source**: DPDP Act Schedule of Penalties
[https://www.dpdpa.com/theschedule.html](https://www.dpdpa.com/theschedule.html)

IT Act penalties (if applicable as intermediary):
- Section 43: Damages up to Rs. 1 crore
- Section 66: Imprisonment up to 3 years + fine up to Rs. 5 lakh

---

## Final Verdict

Building a POS/bakery software for India in 2026-2027 requires
compliance with 6 distinct legal frameworks simultaneously.

**The DPDP Act (fully enforceable May 14, 2027) is the highest priority**.
It carries the largest penalties and the most complex requirements.
Your software must bake consent, notice, access, erasure, and breach
notification into its core - not bolt them on later.

**GST compliance is already table stakes**.
Every POS app in India handles this. If yours does not, it will not sell.

**IT Rules 2021 apply if you ever go cloud/SaaS**.
Desktop offline distribution avoids intermediary classification.
Cloud distribution triggers full intermediary obligations including
grievance officer, takedown, and record retention.

**2027 is the year everything changes**.
May 14, 2027 is the hard deadline.
After that, the Data Protection Board can fine you.
Build compliance before then, not after.

**Disclaimer**: This is a technical compliance guide for software developers.
It does not constitute legal advice.
Consult a qualified Indian lawyer for your specific situation.

---

## Source Links

1. DPDP Act 2023 (Official Text): https://www.meity.gov.in/static/uploads/2024/06/2bf1f0e9f04e6fb4f8fef35e82c42aa5.pdf
2. DPDP Rules 2025 (Wikipedia - Timeline): https://en.wikipedia.org/wiki/Digital_Personal_Data_Protection_Rules,_2025
3. DLA Piper - India Data Protection Laws: https://www.dlapiperdataprotection.com/?t=law&c=IN
4. India Briefing - DPDP Timeline 2026-2027: https://www.india-briefing.com/news/india-dpdp-compliance-timeline-enforcement-2026-27-44740.html/
5. India Briefing - DPDP Compliance 2027 Checklist: https://www.india-briefing.com/news/india-dpdp-compliance-gdpr-comparison-45702.html/
6. IT Rules 2021 (MeitY - with 2026 amendments): https://www.meity.gov.in/static/uploads/2026/02/550681ab908f8afb135b0ad42816a1c9.pdf
7. CERT-In Directions 2022: https://www.cert-in.org.in/PDF/CERT-In_Directions_70B_28.04.2022.pdf
8. RBI Payment Data Localization FAQ: https://www.rbi.org.in/commonman/english/scripts/FAQs.aspx?Id=2995
9. DPDP Act Penalty Schedule: https://www.dpdpa.com/theschedule.html
10. DPDP Act Section 16 (Cross-border): https://www.dpdpa.com/dpdpa2023/chapter-4/section16.html
11. KSK Law - Data Retention Analysis: https://ksandk.com/data-protection-and-data-privacy/data-retention-and-deletion-under-indias-dpdp-rules/
12. EY India - DPDP Guide: https://www.ey.com/en_in/insights/cybersecurity/decoding-the-digital-personal-data-protection-act-2023
13. Digital India Act Status: https://indiaairulebook.com/learn/platforms-internet/digital-india-act
14. Intermediary Liability (DPDPIndia): https://www.dpdpindia.in/itact-intermediary.html
15. FSSAI Food Safety Display Boards: https://www.fssai.gov.in/cms/food-safety-display-boards.php
16. GST HSN Advisory: https://tutorial.gst.gov.in/downloads/news/hsn_advisory_table_12_2.pdf
17. PIB DPDP Rules Notification: https://www.pib.gov.in/PressReleasePage.aspx?PRID=2190014
