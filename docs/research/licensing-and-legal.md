# Licence and legal-structure facts for an embeddable open-source simulation engine (research date 2026-10-01)

Every bullet cites a page I opened. Anything tagged **not verified** is from general knowledge or a page that failed to load, and should be checked before you rely on it. Pages were read through a summarising fetcher, so wording in quotes is as returned by that tool; re-check exact text before quoting it in a legal document.

**Pages that would not load:** gnu.org (GPL FAQ, LGPL-3.0 and AGPL-3.0 texts), modeltrademarkguidelines.org (certificate error), EUR-Lex full texts of the CRA and PLD (only page chrome returned), eol.org terms, catalogueoflife.org terms, Godot's trademark policy page, and Luanti's licensing page.

**Corrections to your brief:**
- Recast is zlib-licensed (your brief gave no licence).
- Luanti is LGPL-2.1, not LGPL-3.0.
- NetLogo is GPL-2.0.
- NLnet's NGI Zero Commons Fund closed its final call on 1 June 2026.
- Mozilla MOSS is on indefinite hiatus.
- CZI EOSS is closed.
- NumFOCUS applications are closed.

## 1. Licence options

**MIT**
- The only condition is keeping the copyright and permission notice; the software is provided "AS IS" with no warranty, and authors are not liable for "any claim, damages or other liability". The text has no patent language. https://opensource.org/license/mit

**Apache-2.0**
- Section 3 gives each contributor's "perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable" patent licence. https://www.apache.org/licenses/LICENSE-2.0
- Patent retaliation: if you start patent litigation alleging the Work infringes, your patent licences for that Work terminate on the filing date. (same URL)
- Section 4: give recipients a copy of the licence, mark modified files, retain notices, and if the Work ships a NOTICE file, derivative works must include a readable copy of its attribution notices. (same URL)
- Section 7 disclaims warranties ("AS IS"); section 8 limits contributor liability for direct, indirect, special, incidental and consequential damages. (same URL)
- Compatibility: Apache-2.0 code can go into GPLv3 projects but not the reverse; the FSF has never considered Apache-2.0 compatible with GPLv2, because of the patent termination and indemnification provisions. https://www.apache.org/licenses/GPL-compatibility.html

**Dual MIT OR Apache-2.0 (Rust convention)**
- Rust API Guidelines: Rust itself is dual-licensed MIT or Apache-2.0, and crates wanting maximum ecosystem compatibility are recommended to do the same. https://rust-lang.github.io/api-guidelines/necessities.html
- The same page warns that Apache-only software "cannot be used in some situations where most of the Rust runtime stack can". The page does not spell out the GPLv2 reason; that comes from the Apache compatibility page above.
- Bevy uses this model, with an inbound clause: any contribution intentionally submitted "shall be dual licensed as above". https://github.com/bevyengine/bevy

**MPL-2.0**
- Copyleft is file-level: it applies only to files containing MPL code. New files with no MPL code are not Modifications. https://www.mozilla.org/en-US/MPL/2.0/FAQ/
- The FAQ says this permits MPL code "to be statically linked to and distributed as part of a larger proprietary piece of software". (same URL)
- When distributing executables, recipients must be told where to get the source of the MPL files (section 3.2). (same URL)
- GPL/LGPL/AGPL compatibility is via section 3.3 "Larger Works" and Secondary Licenses, unless the author marks the code "Incompatible With Secondary Licenses" (Exhibit B). (same URL)
- Trade-off: modifications to the engine's own files must be published by third parties, but their game or content code stays theirs.

**LGPL**
- **Not verified** (gnu.org failed to load): LGPL-3.0 section 4(d) requires either shipping the application's object code so users can relink against a modified library, or using a shared-library mechanism; 4(e) requires Installation Information. Read https://www.gnu.org/licenses/lgpl-3.0.html directly.
- **Not verified:** the usual reasoning that this is hard on iOS (static linking, code signing, no user re-installation) and in a single WebAssembly bundle (effectively static linking) is common practitioner analysis; I opened no primary source for it.
- Precedent: Luanti is LGPL-2.1. https://github.com/luanti-org/luanti

**GPL-3.0 / AGPL-3.0**
- **Not verified** (gnu.org returned HTTP 429): AGPL-3.0 section 13 requires offering Corresponding Source to users interacting with a modified version over a network. Read https://www.gnu.org/licenses/agpl-3.0.html.
- Redis and Elastic both describe AGPL as OSI-approved. https://redis.io/blog/agplv3/ and https://www.elastic.co/blog/elasticsearch-is-open-source-again

**Source-available licences and OSI's position**
- OSI (19 Jan 2021): SSPL is not open source; it was submitted and then withdrawn when it became clear it would not be approved. Calling such "fauxpen" licences open source is "deception, plain and simple". https://opensource.org/blog/the-sspl-is-not-an-open-source-license
- BSL 1.1: created by MariaDB; source-available; converts to a GPL-2.0+-compatible "Change License" within at most 4 years; has an optional "Additional Use Grant"; not recognised by OSI as open source. https://en.wikipedia.org/wiki/Business_Source_License (secondary source)
- OSI calls this pattern "Delayed Open Source Publication": proprietary first, open source later, by plan. https://opensource.org/blog/delayed-open-source-publication
- FSL: restricts competing uses, converts to Apache-2.0 or MIT after two years per release, and describes itself as "Fair Source", not open source. https://fsl.software/
- PolyForm: seven licences (Strict, Noncommercial, Free Trial, Internal Use, Small Business, Perimeter, Countdown); the project does not present them as open source. https://polyformproject.org/licenses/
- Elastic License 2.0: three limits — no offering as a hosted or managed service, no circumventing licence-key functionality, no removing notices. https://www.elastic.co/licensing/elastic-license
- Defold says outright that it does not use the term "Open Source" because OSI's definition requires that the code can be commercialised. https://defold.com/open/

## 2. App store compatibility

**Apple**
- FSF (26 May 2010): the App Store terms' Usage Rules (five-device limit, mandatory terms of service) are "further restrictions" forbidden by GPLv2 section 6. Apple removed GNU Go rather than change its terms. https://www.fsf.org/blogs/licensing/more-about-the-app-store-gpl-enforcement
- VLC: a GPLv2 iOS port was accepted in September 2010, then withdrawn in January 2011 after VLC developer Rémi Denis-Courmont complained of the licence conflict. From October 2011 the engine was relicensed from GPL-2.0+ to LGPL-2.1+. The app returned in July 2013 after relicensing under MPL-2.0. https://en.wikipedia.org/wiki/VLC_media_player (secondary source)
- Current status of the Apple/GPL conflict: **not verified**. I found no later statement from Apple or the FSF.
- MPL-2.0 on iOS: Firefox for iOS is MPL-2.0. https://github.com/mozilla-mobile/firefox-ios

**Steam**
- Valve's Steamworks documentation: you warrant you have the rights to distribute; an application with incompatible open-source code must not be distributed on Steam. https://partner.steamgames.com/doc/sdk/uploading/distributing_opensource
- Licences Valve names as fine: MIT, BSD 3-clause and 4-clause, Apache 2.0, WTFPL. (same URL)
- "Any license that has a so-called 'copyleft' element will be problematic when combining code with the Steamworks SDK"; GPL is the named example. (same URL)
- GPL titles do exist on Steam where the GPL code's author has given permission. Valve does no due-diligence review and says to consult a lawyer. (same URL)
- Example: OpenTTD (GPL-2.0) is on Steam, GOG and the Microsoft Store. https://github.com/OpenTTD/OpenTTD

## 3. Precedents

| Project | Licence | Governance / notes | Source |
|---|---|---|---|
| Godot | MIT | Games are "entirely yours"; shipping the binary requires the MIT notice. Godot Foundation is a Dutch non-profit ("Stichting Godot"), founded 23 Aug 2022, donation-funded, and "does not own the Godot Project" | https://godotengine.org/license/ ; https://godot.foundation/ |
| Bevy | MIT OR Apache-2.0 | Bevy Foundation is a Washington-state non-profit, 501(c)(3) public charity (announced 25 Sep 2024), five-member board | https://github.com/bevyengine/bevy ; https://bevy.org/foundation/ ; https://bevy.org/news/bevy-foundation-501c3/ |
| Rapier (Dimforge) | Apache-2.0 | — | https://github.com/dimforge/rapier |
| Box2D | MIT | Single author (Erin Catto) | https://github.com/erincatto/box2d |
| Jolt Physics | MIT | Used in Horizon Forbidden West and Death Stranding 2 | https://github.com/jrouwe/JoltPhysics |
| Flecs | MIT | — | https://github.com/SanderMertens/flecs |
| Recast/Detour | zlib | — | https://github.com/recastnavigation/recastnavigation |
| Defold | Defold License (Apache-2.0-derived) | Forbids commercialising the engine "as a Game Engine Product"; games are free to sell, no royalties; owned by the Defold Foundation; self-described "source available" | https://defold.com/license/ ; https://defold.com/open/ |
| O3DE | Apache-2.0 (MIT licence file also in repo) | Linux Foundation / Open 3D Foundation governance **not verified** on the pages I opened | https://github.com/o3de/o3de |
| Luanti (Minetest) | LGPL-2.1 | — | https://github.com/luanti-org/luanti |
| OpenTTD | GPL-2.0 | Distributed on Steam | https://github.com/OpenTTD/OpenTTD |
| Mesa ABM | Apache-2.0 | NumFOCUS link not shown on repo page (**not verified**) | https://github.com/mesa/mesa |
| NetLogo | GPL-2.0 | Commercial licence availability **not verified** | https://github.com/NetLogo/NetLogo |
| Agents.jl | MIT | — | https://github.com/JuliaDynamics/Agents.jl |

**Relicensing events**
- Unity: Runtime Fee announced September 2023, cancelled 12 September 2024. https://discussions.unity.com/t/a-message-to-our-community-unity-is-canceling-the-runtime-fee/1517714
- HashiCorp: 10 Aug 2023, MPL-2.0 to BSL 1.1 for future product releases (APIs, SDKs and most libraries stayed MPL). Stated reason: vendors who "take advantage of pure OSS models… without providing material contributions back". https://www.hashicorp.com/en/blog/hashicorp-adopts-business-source-license
- That move triggered the OpenTofu fork. https://en.wikipedia.org/wiki/Business_Source_License
- Redis: moved to SSPL in March 2024 (the RSALv2 half of that dual licence is **not verified** from the page); added AGPLv3 as an option from Redis 8 on 1 May 2025, citing community backlash. https://redis.io/blog/agplv3/
- Elastic: left Apache-2.0 for SSPL and Elastic License in 2021; added AGPL as an option on 29 Aug 2024. https://www.elastic.co/blog/elasticsearch-is-open-source-again
- VLC's GPL to LGPL/MPL relicensing (section 2) is the example of relicensing to reach app stores.

## 4. Contributor terms

- **DCO 1.1:** the contributor certifies (a) it is their original work with the right to submit, (b) or it is based on appropriately licensed work, (c) or it was passed on by someone who certified, and (d) the contribution and sign-off are public record. It grants no extra rights beyond the project licence. https://developercertificate.org/
- **Linux kernel:** uses `Signed-off-by` under DCO 1.1 (`git commit -s`). https://www.kernel.org/doc/html/latest/process/submitting-patches.html
- **GitLab:** switched from CLA to DCO on 1 Nov 2017 because the CLA deterred contributors (Debian and GNOME asked for the change). https://about.gitlab.com/blog/gitlab-switches-to-dco-license/
- **Godot:** CONTRIBUTING.md mentions no CLA, DCO or sign-off. Your brief's claim that Godot uses a DCO is **not verified**. https://github.com/godotengine/godot/blob/master/CONTRIBUTING.md
- **Apache ICLA/CCLA:** a licence grant, not a copyright assignment; contributors keep full rights to their work. An ICLA is required before commit rights; a CCLA does not replace individual ICLAs. https://www.apache.org/licenses/contributor-agreements.html
- **CLA Assistant:** a GitHub pull-request signing tool operated by SAP, itself open source; no price stated. https://cla-assistant.io/
- **GitHub ToS D.6:** "Whenever you add Content to a repository containing notice of a license, you license that Content under the same terms" ("inbound=outbound"); a separate agreement such as a CLA takes precedence. https://docs.github.com/en/site-policy/github-terms/github-terms-of-service
- **Software Freedom Conservancy (Kuhn, 2014):** CLAs are "pointless bureaucracy" that shift patent blame onto contributors; inbound=outbound plus a licence with patent terms (Apache-2.0, GPLv3) or a DCO is enough. https://sfconservancy.org/blog/2014/jun/09/do-not-need-cla/
- **Relicensing implication (inference from the above; I opened no source stating it directly):** under DCO or inbound=outbound, each contributor keeps copyright and licenses only under the project licence. A permissive licence already lets anyone, including the studio, ship in proprietary products. Changing the outbound licence later (as HashiCorp, Redis and Elastic did) needs either broad rights from a CLA or assignment, or every contributor's consent. A CLA or assignment keeps that option open at the cost of contributor friction.

## 5. Trademark

- **Mozilla:** the code licence "does not give permission to use Mozilla's trademarks". Modified builds must not use the Firefox name and must pick their own branding; "based on Mozilla technology" with a non-affiliation disclaimer is allowed. https://www.mozilla.org/en-US/foundation/trademarks/policy/
- **Rust Foundation:** owns the word marks "Rust" and "Cargo" and two logos. The logos are CC-BY for copyright, but trademark rules apply separately. https://rustfoundation.org/policy/rust-trademark-policy/
  - Allowed without permission: "written in Rust", repo names, non-profit merchandise, free events, and limited-modification redistributions.
  - Needs permission: selling merchandise, substantially modified versions under the name, "RustConf"-style event names, and use inside other trademarks.
- **Godot:** the "Godot" word and logo marks were registered while the project was at Software Freedom Conservancy, then transferred to the Godot Foundation in 2023. A public trademark policy was "being developed" as of December 2024. https://godotengine.org/article/godot-foundation-update-dec-2024/
  - The policy page itself returned 404. The logo's CC BY licence is **not verified**.
- **Bevy:** trademark policy **not verified**; the foundation pages I opened do not mention one.
- **USPTO fees (effective 18 Jan 2025, still listed):** https://www.uspto.gov/trademarks/trademark-fee-information
  - Base application: $350 per class.
  - Surcharges per class: $100 insufficient information, $200 free-form text identification, $200 per additional 1,000 characters.
  - Intent-to-use: $150 per class for statement of use, $125 per class for an extension.
  - Maintenance: $325 per class for the section 8 declaration (years 5–6); $650 per class for the combined 10-year renewal.
- **Model Trademark Guidelines** (modeltrademarkguidelines.org): **not verified**, the site's TLS certificate was invalid.

## 6. Fiscal hosts and foundations

- **Software Freedom Conservancy:** https://sfconservancy.org/projects/apply/
  - Rolling applications, slow responses.
  - Project must be exclusively FOSS, under licences that are both OSI-approved and DFSG-free, with a diverse community and at least a year of history.
  - Takes 10% of revenue.
  - Can hold copyrights, trademarks and domains, and gives legal support.
  - Exit is possible, but assets must go to another 501(c)(3).
  - Trade-off for a studio: a project controlled by one for-profit, or wanting any non-FOSS component, does not fit.
- **Open Source Collective (on Open Collective):** https://docs.oscollective.org/welcome-and-introduction-to-osc/fees and https://oscollective.org/projects/
  - A US 501(c)(6).
  - 10% host fee on incoming funds, plus payment-processor fees; no setup fee or minimum.
  - Accepts "most open source projects… anywhere in the world".
  - Can hold trademarks and domains and sign agreements.
  - Detailed eligibility criteria **not verified**.
- **NumFOCUS:** https://numfocus.org/projects-overview
  - Project must be scientifically oriented, OSI-licensed, with multiple contributors and a code of conduct.
  - Fiscal sponsorship also needs public governance and a roadmap.
  - Applications are currently closed.
  - Fee percentage **not verified**.
- **Linux Foundation:** https://www.linuxfoundation.org/projects/hosting
  - Requires open governance and neutral ownership of trademarks, repos and accounts by the foundation.
  - Requires at least one LF member and five sponsoring organisations.
  - Costs are not published.
- **Commons Conservancy (Netherlands):** https://commonsconservancy.org/faq/
  - A zero-budget foundation, free to join.
  - Provides a legal and governance home and IP guidance, but handles no money.
  - Works with NLnet for funds.
- **Own non-profit (precedents):**
  - Bevy Foundation — 501(c)(3), files Form 990, takes donations via Every.org. https://bevy.org/news/bevy-foundation-501c3/
  - Godot Foundation — Dutch stichting. https://godot.foundation/
  - Defold Foundation — owns the engine; non-commercialisation is guaranteed under Swedish law. https://defold.com/license/
- **Keeping it under the studio's LLC:** no primary source opened. The Linux Foundation and Conservancy pages above imply the trade-off: the studio keeps trademark and control, but third parties get no neutral-ownership assurance, and donations are not tax-deductible. California LLC cost facts are **not verified**.

## 7. Content and data licensing

**Creative Commons**
- CC: "We recommend against using Creative Commons licenses for software" — they have no source or patent terms and are incompatible with the GPL. https://creativecommons.org/faq/
- CC licences can be used on databases; version 4.0 covers sui generis database rights. NonCommercial prohibits commercial use without permission. (same URL)
- **Not verified** (general knowledge, chooser page redirected): CC BY 4.0 allows commercial use with attribution; BY-SA adds share-alike on adaptations; CC0 is a public-domain dedication; BY-NC is non-commercial only.

**Ecological data sources — commercial-use status**

| Source | Licence / terms | Commercial use in a paid product | Source |
|---|---|---|---|
| GBIF | Each dataset is CC0, CC BY or CC BY-NC | Yes for CC0 and CC BY (attribute per owner); no for BY-NC datasets. GBIF leaves "commercial" undefined and does not enforce NC itself | https://www.gbif.org/terms |
| iNaturalist | Default CC BY-NC per user, for photos and observations; only CC0, BY and BY-NC records go to GBIF | No by default; yes only for records users set to CC0 or CC BY | https://help.inaturalist.org/en/support/solutions/articles/151000173511-how-do-licenses-work-on-inaturalist-should-i-change-my-licenses- |
| eBird (Cornell) | Custom terms | No without written permission. "Commercial" includes for-profit entities and non-profits generating revenue. No redistribution of original data; derived datasets must carry the same terms | https://www.birds.cornell.edu/home/ebird-data-access-terms-of-use/ |
| USDA PLANTS | "Our plant information, including the distribution maps, lists, and text, is not copyrighted and is free for any use" | Yes for data. Images vary: some are copyrighted and need permission; credit is always required | https://plants.sc.egov.usda.gov/DocumentLibrary/Pdf/PLANTS_Help_Document_2022.pdf |
| Encyclopedia of Life | **Not verified** (403); per-object CC licences is from general knowledge | Check per object | — |
| TRY plant traits | Public records are CC BY; restricted datasets need contributor permission; the requester must accept the Intellectual Property Guidelines | Likely yes for public CC BY data with citation; the guideline text on commercial use is **not verified** | https://www.try-db.org/TryWeb/Prop0.php |
| GloBI | An index of openly shared datasets; no single licence stated; must attribute original contributors and cite datasets | Depends on each source dataset (**not verified** per dataset) | https://www.globalbioticinteractions.org/data |
| Open Tree of Life | "All data are published with a CC0 public domain dedication" | Yes | https://tree.opentreeoflife.org/about/licenses |
| Wikidata | CC0 mandatory for all structured data | Yes | https://www.wikidata.org/wiki/Wikidata:Licensing |
| IUCN Red List | Neither the data nor "any work derived from or based upon" it may be put to commercial use without IUCN's written permission; all reposting and redistribution prohibited | No. IBAT as the commercial route is **not verified** | https://www.iucnredlist.org/terms/terms-of-use |
| FishBase | CC BY-NC 4.0; photos are separately copyrighted | No without permission. SeaLifeBase is **not verified** (assumed the same) | https://www.fishbase.se/search.php |
| Catalogue of Life | Citation of COL and contributing datasets required; the CC BY 4.0 licence is **not verified** (terms page 403) | Probably yes with attribution (**not verified**) | https://www.catalogueoflife.org/about/colusage |

## 8. Other legal points

**Warranty and liability**
- See section 1: both MIT and Apache-2.0 disclaim warranty and liability. Whether those disclaimers hold against EU consumers is **not verified**.

**US export controls**
- BIS: "'Publicly available' encryption source code is not subject to the EAR once the email notification per section 742.15(b) is sent." https://www.bis.gov/learn-support/encryption-controls/encryption-items-not-subject-to-ear
- A product that incorporates open-source cryptography is a new item, evaluated as a whole. (same URL)
- EFF: the notification goes to crypt@bis.doc.gov and enc@nsa.gov with the URL of the code. https://www.eff.org/deeplinks/2019/08/us-export-controls-and-published-encryption-source-code-explained
- Published software without cryptography falling outside the EAR under §734.7, and the 2021 narrowing of the notification to non-standard cryptography, are both **not verified**.

**EU Cyber Resilience Act (Regulation 2024/2847)**
- In force 10 Dec 2024; vulnerability and incident reporting from 11 Sep 2026; full application 11 Dec 2027. https://digital-strategy.ec.europa.eu/en/policies/cra-summary
- Open-source software not made available on the market in a commercial activity is out of scope (recital 18). Components meant for integration are covered only if monetised by their original maker. Donations or contributions do not by themselves make development commercial. https://www.european-cyber-resilience-act.com/Cyber_Resilience_Act_Preamble_11_to_20.html (unofficial mirror)
- "Open-source software stewards" (legal persons giving sustained support to FOSS intended for commercial activities, such as foundations) get a lighter regime under Article 24: a documented cybersecurity policy, cooperation with authorities, and reporting of actively exploited vulnerabilities and severe incidents. https://www.european-cyber-resilience-act.com/Cyber_Resilience_Act_Article_24.html
- Stewards cannot affix CE marking and are exempt from fines. https://digital-strategy.ec.europa.eu/en/policies/cra-summary
- Consequence: a company selling a game or app in the EU is the "manufacturer" and carries the full obligations, including due diligence on the components it integrates. A studio that both authors the library and sells products on it is a manufacturer for those products. The official EUR-Lex text was not readable through the tool, so check the exact wording there.

**EU Product Liability Directive 2024/2853**
- Software is a "product". Article 2(2) excludes free and open-source software "not provided in the course of a business activity"; the exclusion is lost if it is supplied for a price or for personal data. https://www.ibanet.org/European-Product-Liability-Directive-liability-for-software (secondary source)
- A manufacturer that integrates OSS is liable for the final product. Micro and small software suppliers may contractually exclude recourse claims (Article 12(2)). (same URL)
- Member states must transpose by 9 Dec 2026. (same URL)

**Children's privacy (COPPA)**
- COPPA obligations attach to "operators" that collect personal information from under-13s. A child-directed service collecting none is outside the core requirements, though the FTC still recommends posting a privacy policy. https://www.ftc.gov/business-guidance/resources/complying-coppa-frequently-asked-questions
- The app operator is responsible for every third-party SDK in a child-directed app and must inquire into their data practices. An SDK provider is liable only with actual knowledge. (same URL)
- So a library with no data collection or network calls adds no COPPA burden; the app maker remains the operator.
- Apple Kids Category and Google Families policy details are **not verified**.

**Patents**
- MIT has no patent grant; Apache-2.0 has a grant plus the retaliation clause (section 1). Whether MIT carries an implied patent licence is **not verified**.

**Licence metadata and scanning**
- REUSE 3.3: a `LICENSES/` directory with one file per SPDX identifier, per-file `SPDX-FileCopyrightText` and `SPDX-License-Identifier` headers (or `.license` sidecar files, or `REUSE.toml`). https://reuse.software/spec-3.3/
- cargo-deny: checks every crate's SPDX licence expression against your configuration. It reads Cargo.toml and LICENSE files and does not scan all source, so mis-declared licences can slip through. https://embarkstudios.github.io/cargo-deny/checks/licenses/index.html
- license-checker (npm): lists dependency licences; `--failOn`, `--production`, `--json`, `--csv`, `--summary`. `--onlyAllow` is **not verified**. https://www.npmjs.com/package/license-checker
- SBOM formats (SPDX, CycloneDX): **not verified**, nothing opened.

**Attribution in shipped apps**
- MIT requires the notice in copies; Apache-2.0 requires the licence copy plus any NOTICE content; Godot asks for the licence text or a link in game credits or documentation. Sources as in sections 1 and 3 (https://godotengine.org/license/).
- An in-app "licences/acknowledgements" screen is the usual way to satisfy this for binary mobile and Steam builds. Platform specifics are **not verified**.

## 9. Sustainability and funding

**Models seen in the sources**
- Donation-funded foundation: Godot, Bevy.
- Dual licence or source-available restriction on competitors: BSL, FSL, Elastic, Defold's "no engine resale".
- Additive AGPL plus commercial licence: Redis, Elastic.
- Paid support, hosted services and paid content packs: no primary source opened (**not verified** as a documented model for small engines).
- Trade-off the sources state: OSI and the OpenTofu reaction show that restrictive relicensing costs community trust; HashiCorp, Redis and Elastic argue pure permissive licensing invites free-riding.

**Funding channels**
- **GitHub Sponsors:** no fee on sponsorships from personal accounts; up to 6% from organisation accounts (3% card processing plus 3% GitHub, with the card fee avoidable by invoice). https://docs.github.com/en/sponsors/sponsoring-open-source-contributors/about-sponsorships-fees-and-taxes
- **Open Collective via Open Source Collective:** 10% (section 6).
- **Sovereign Tech Fund (Germany):** https://www.sovereign.tech/programs/fund
  - Applications open, in German or English; non-German applicants are accepted.
  - Funds "open digital base technologies" (libraries, protocols and the like); explicitly not user-facing applications or prototypes.
  - Minimum project cost over €50,000; OSI or FSF licence required.
  - Judged on prevalence, relevance and public interest; about six months to contract.
  - A new niche simulation engine likely fails the prevalence test (my inference).
- **NLnet NGI Zero Commons Fund:** grants of €5,000–€50,000 with FOSS outputs, but the thirteenth and final call closed 1 June 2026. https://nlnet.nl/commonsfund/
  - While it ran, non-EU applicants were eligible only with "exceptional quality", "unique technical expertise" and "a clear European dimension", with EU and associated countries given priority. https://nlnet.nl/commonsfund/eligibility/
  - Successor NLnet calls are **not verified**.
- **Mozilla MOSS:** on indefinite hiatus since 2020; Mozilla points to the Mozilla Technology Fund, whose current status is **not verified**. https://www.mozilla.org/en-US/moss/
- **Sloan Foundation Technology program:** funds open source in science; the route is a two-page letter of inquiry; eligibility by organisation type is not stated (whether it is limited to non-profits and universities is **not verified**). https://sloan.org/programs/digital-technology
- **CZI Essential Open Source Software for Science:** closed; cycle 6 was the last listed ($11.7M, with Kavli and Wellcome); the focus was biomedical research software. https://chanzuckerberg.com/eoss/
- **NumFOCUS Small Development Grants:** available only to sponsored or affiliated projects; applications closed. https://numfocus.org/projects-overview
- **Tax-deductible donations and many grants** require 501(c)(3) status, either your own (Bevy) or a fiscal host's. https://bevy.org/news/bevy-foundation-501c3/
