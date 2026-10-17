# Evaluating a Property

Reference for the `realtor` persona: walking a buyer from listing to offer price on one specific SF property. Compiled late September 2026; each claim links its source. Cost figures are what sources quote, not quotes for a specific house — many come from contractor marketing pages (flagged "(vendor)"), which skew toward the scary end and should be read as order-of-magnitude. Official sources (SF.gov, SFPUC, CA statute, ASHI) are more reliable than blogs. Where sources disagree, both are given.

Source-quality legend: (official) city/state/standards body; (agent) SF agent or brokerage blog; (vendor) contractor/inspector selling the service; (press) news.

---

## 1. Listing triage — what to read before you ever visit

### 1a. Price history and market signals

- How SF pricing works: list price is usually a lure, not an expectation, for houses under ~$5M. Single-family homes: ~70% sold over asking in the past year, averaging ~11% over; condos ~25% over asking, typically near list. Underpricing stops working above ~$5M where buyers expect to negotiate down. ([SF Standard, Oct 2025](https://sfstandard.com/2025/10/03/home-price-strategies-san-francisco/) (press))
- Varies by district: "8 out of 10 SF districts on average see buyers paying above the list price"; District 2 ~17.7% over; SOMA/Civic Center/FiDi list close to seller's expectation. ([Haven Group](https://havengroupsf.com/the-truth-about-list-prices-in-sf-what-buyers-and-sellers-must-know/) (agent))
- Current tempo: May 2026 SF houses sold at 124.4% of list in 18 days avg; condos 101.4% of list in 39 days. ([Helena 7x7 playbook](https://www.helena7x7.com/sf-buyers-playbook-competitive-market/) (agent))
- Consequence for triage: for a house, a long DOM, price cuts, or a relist is a real signal (it missed the auction window); for a condo, 30–40 days is normal. SocketSite routinely documents stepped price cuts and relists (e.g. [17 Guy Place cut three times](https://socketsite.com/archives/2024/03/another-motivated-seller-and-near-million-dollar-drop.html), [a relist selling ~30% below the prior price](https://socketsite.com/archives/2024/05/at-asking-sale-yields-nearly-30-percent-less.html)) — worth checking the address there and on Redfin/Zillow history for prior sale price, prior listing periods, and withdrawn/relisted cycles.
- What to extract from history (agent practice; inferred, not a single source): prior sale price and date (implied appreciation vs. neighborhood); whether the current list follows a withdrawn listing (DOM reset); number and size of cuts; whether it was a flip (short hold + big jump → scrutinize permits for the remodel).

### 1b. Square footage — assume it's wrong

- "the square foot number has about a 50-50 chance of being accurate," often overstated. The MLS allows nine sources for sqft: appraiser, architect, builder, condo map, agent measurement, owner, graphic artist, tax records, "not available." Agents sometimes omit sqft when the $/sqft looks bad or tax records are clearly wrong. ([Marina Times](https://www.marinatimes.com/lets-talk-square-foot-in-san-francisco-real-estate) (agent/press))
- Why listing, assessor and appraisal disagree: no single standard measurement method; appraisers generally don't count garages or unfinished space and may reject rooms not used as described; unpermitted additions never reach tax records. Bay Area listing agents typically put "buyer to verify square footage" in private remarks. ([SFGate](https://www.sfgate.com/realestate/article/Why-Might-a-Listing-s-Square-Footage-Differ-From-16082593.php) (press), [SDAR legal](https://go.sdar.com/legal/handling-square-footage-differences-in-residential-listings))
- Triage rule: if listing sqft >> assessor sqft, the difference is often unwarranted ground-floor space (see 1d). Compute $/sqft both ways.

### 1c. Photos

General practice (no single authoritative source): note what's missing — no exterior/street shot, no garage/basement, no bathroom, no view from the main windows, only wide-angle. Missing garage/basement photos in a pre-1940 house means you haven't seen the foundation. Missing exterior may hide stucco cracking or a busy street.

### 1d. Phrases that mean something

- "Bonus room", "den", "office", "flex space", "down"/"garden level", "au pair", "in-law", "rec room" → typically unwarranted space: "not shown on the city records, so no one can therefore 'warrant' (guarantee) the legality." ([Jennifer Rosdail](https://www.jenniferrosdail.com/unwarranted-space-in-law-units-and-renting-them-out-short-term-and-otherwise/) (agent))
  - Risks: never inspected for code (electrical/plumbing); value questionable because the city could require change or removal; hard to insure; renting it gives tenants habitability/fraud claims and may make non-payment evictions unenforceable. ([Rosdail](https://www.jenniferrosdail.com/unwarranted-space-in-law-units-and-renting-them-out-short-term-and-otherwise/), [Bornstein Law](https://bornstein.law/adus-and-unwarranted-units/))
  - Legalizing a room inside the existing envelope is often feasible; legalizing a second unit (light, air, fire egress) "can be very hard and get expensive." ([Rosdail](https://www.jenniferrosdail.com/unwarranted-space-in-law-units-and-renting-them-out-short-term-and-otherwise/)) If a pre-1979 building becomes legal multi-unit, both units become rent controlled. ([Inside SF RE](https://insidesfre.com/the-risks-behind-unwarranted-in-law-units/) (agent))
  - Official legalization path: [SF Planning Dwelling Unit Legalization Program fact sheet](https://default.sfplanning.org/plans-and-programs/planning-for-the-city/accessory-dwelling-units/Legalization_FactSheet-091017.pdf) (official).
- "Tenant occupied" → value and possession hinge on rent control and OMI rules (see §2i). Online valuations miss this entirely: "Rent-controlled units swing values by hundreds of thousands." ([SFBlocks](https://www.sfblocks.com/blog/why-your-homes-online-valuation-is-probably-wrong) (agent))
- "Probate" / "court confirmation" → contingencies must be removed before the court date; buyer puts up 10% cashier's check; at the hearing others may overbid at a minimum of 5% + $500 over the accepted offer (e.g. $1,050,500 on $1M); inspection/appraisal costs lost if overbid. ([Inside SF RE probate primer](https://insidesfre.com/probate-sale-primer-for-sf-home-buyers/), [Jennifer Rosdail on court confirmation](https://www.jenniferrosdail.com/courtconfirmation/)) A probate under IAEA full authority skips the court hearing and its overbid risk. ([Own Team](https://www.theownteam.com/blog/probate-vs-trust-sale-california-timeline/))
- "Trust sale" → generally no court confirmation (see Own Team link above), but the trustee often never lived there, so the TDS may be thin (many "unknown"s).
- "As-is" → seller won't make repairs; does not remove disclosure duties. Treat as a request to price in the pre-inspections.

### 1e. Honest $/sqft and comps

- $/sqft is weakest for single-family homes (renovation state varies wildly), better for newer condo buildings with standardized units. ([Marina Times](https://www.marinatimes.com/lets-talk-square-foot-in-san-francisco-real-estate)) Comparing a renovated small unit to a larger dated one "can flip the $ per foot without saying much about value." (via [search summary of SF agent blogs](https://nowhomes.net/blog/how-to-read-a-san-francisco-real-estate-market-report))
- Comp method (SF agent practice): 3–6 sales matching type, size, condition, micro-location; condos start with same-building sales; row houses/flats prioritize same block face; hillsides adjust for grade and view; adjust for sqft, bath count, finish level, view, outdoor space, parking. ([Kia Amini valuation](https://kiaamini.com/blog/what-is-your-san-francisco-home-worth-get-a-pro-valuation) (agent))
- Parking is not a checkbox: deeded vs. leased vs. tandem differ materially. ([SFBlocks](https://www.sfblocks.com/blog/why-your-homes-online-valuation-is-probably-wrong))
- "Comps alone don't tell the story" — each neighborhood has its own over-asking norm. ([Haven Group](https://havengroupsf.com/the-truth-about-list-prices-in-sf-what-buyers-and-sellers-must-know/))
- Procedure (synthesis):
  1. Pull closed sales, same micro-neighborhood (same side of a hill/park if possible), same type (SFH / condo / TIC / 2–4 unit), last 3–6 months; widen to 12 only if thin, and say so.
  2. Use sale price, not list; note sale-to-list % for each (it tells you the neighborhood's underpricing multiple).
  3. Normalize sqft: use legal (assessor/permitted) sqft; add unwarranted space at a discount, not full value.
  4. Adjust for: legal bedroom count (a "bedroom" with no closet/egress/permit is a den), parking (deeded/independent vs. tandem vs. none), private outdoor space, view, condition (turnkey vs. original), light/exposure, busy street, tenant status. TICs: ~10–20% below comparable condos ([Legacy RE](https://legacysfhomes.com/blog/tic-guide-sf) (agent)).
  5. Report a range, not a point, with the number of comps behind it.

---

## 2. Reading the disclosure packet, in order

SF sellers usually provide a full disclosure package with pre-inspections before offers — typically pest, roof, sewer lateral, sometimes underground storage tank and geology reports. ([Front Steps](https://thefrontsteps.com/2025/01/14/navigating-multiple-offers-in-san-francisco-real-estate-scenario-1/) (agent)) Standard CA inspection contingency is ~17 days, but in competitive SF offers buyers often have to rely on these pre-inspections. ([Wynne Morgensen](https://wynnemorgensen.com/blog/the-san-francisco-disclosure-package-explained) (agent)) A reading guide: [Jen Ferland disclosure guide](https://www.jenferland.com/disclosureguidesf) (agent).

### 2a. TDS / SPQ / AVID (seller and agent statements)
- TDS: seller's statement of known facts (roof, plumbing, electrical, water intrusion, mold, pests, improvements, permits). Watch for repeated "unknown" on basic systems, unpermitted work, repairs with no invoices. ([Wynne Morgensen](https://wynnemorgensen.com/blog/the-san-francisco-disclosure-package-explained))
- SPQ: more detail on past repairs, insurance claims, drainage, disputes, neighborhood noise. Read every "yes" with dates; cross-check against TDS for contradictions or evasive moisture/structural answers. ([Cynthia Money](https://cynthiamoney.com/blog/san-francisco-disclosure-packets-what-buyers-receive), [Wynne Morgensen](https://wynnemorgensen.com/blog/the-san-francisco-disclosure-package-explained))
- AVID: listing agent's reasonable visual inspection; flags stains, cracks, sagging, electrical concerns. ([Cooper Family RE](https://cooperfamilyrealestate.com/california-seller-disclosure-forms-tds-spq-nhd-megans-law) (agent))
- Also in packet: SF-specific seller disclosure (SFAR form), lead-based paint disclosure for pre-1978 (buyer gets a default 10-day right to lead inspection under federal Title X), water-heater/smoke detector statement. ([Bonnie Spindler on lead/asbestos](https://bonniespindler.com/blog/lead-paint-and-asbestos-disclosures-what-california-buyers-of-historic-homes-must-know) (agent))
- Water heater: CA Health & Safety Code §19211 requires bracing; seller must certify in writing before transfer. Two straps (upper and lower third, lower ≥4" above controls). Cheap fix. ([San Diego Home Inspection](https://sandiegohomeinspection.com/water-heater-seismic-strapping-california/) (vendor), [CA Seismic Safety Commission guidelines](https://www.ssc.ca.gov/wp-content/uploads/sites/5/2020/08/waterheaterbracing_08-11-04.pdf) (official))

### 2b. NHD (Natural Hazard Disclosure)
- Vendor report on mapped zones (flood, fire, fault, liquefaction, landslide). Maps risk; doesn't confirm actual damage. Use it to drive insurance quotes and targeted inspections. ([Wynne Morgensen](https://wynnemorgensen.com/blog/the-san-francisco-disclosure-package-explained))
- SF liquefaction: Marina and Mission Bay are the city's highest-risk fill areas; Loma Prieta (1989) severely damaged or collapsed ~70 Marina buildings 60 miles from the epicenter. ([Geoengineer.org](https://www.geoengineer.org/education/web-class-projects/ce-179-geosystems-engineering-design/assignments/liquefaction-during-the-loma-prieta-earthquake), [USGS liquefaction maps](https://www.usgs.gov/programs/earthquake-hazards/science/san-francisco-bay-area-liquefaction-hazard-maps) (official), [CGS Note 53](https://www.conservation.ca.gov/cgs/documents/publications/cgs-notes/CGS-Note-53-SFBA-Regulatory-Earthquake-Hazard-Zones-a11y.pdf) (official))

### 2c. Seller's general home inspection
- Operational/safety overview, usually no cost estimates. ([Jen Ferland](https://www.jenferland.com/disclosureguidesf))
- Know its limits: ASHI's Standard of Practice says the inspector is not required to determine condition of components not readily accessible, remaining life expectancy, or "strength, adequacy, effectiveness, and efficiency," nor inspect underground items. ([ASHI SOP 2026](https://www.homeinspector.org/wp-content/uploads/2026/01/ASHI-SOP-2026-approved-01_08_26.pdf) (official)) So "recommend evaluation by structural engineer/electrician/plumber" lines are the ones to chase.
- Remember who paid for it. Consider your own inspection or a walk-through with the seller's inspector.

### 2d. Pest report (Section 1 vs 2)
- Section 1: active infestation/infection or damage from wood-destroying organisms (termites, fungus/dry rot). Lenders (esp. FHA/VA) typically require Section 1 cleared before funding. Section 2: conditions likely to lead to infestation (e.g. excessive moisture, earth-to-wood contact) — not required to close; negotiable. ([Brian Cooper](https://cooperfamilyrealestate.com/blog/california-section-1-section-2-pest-report-explained) (agent), [Veroax](https://www.veroax.com/blog/section-1-vs-section-2-pest-report-california))
- Pest reports assign dollar values to repairs — the one report in the packet that does. ([Jen Ferland](https://www.jenferland.com/disclosureguidesf)) Treat those as contractor bids; opened walls often grow the number (general practice).
- Look for: "further inspection" items (inaccessible areas) — unknown cost; repeated dry rot at windows, decks, stairs, and below showers; "multiple pest issues in damp San Francisco properties" as a pattern flag. ([Jen Ferland](https://www.jenferland.com/disclosureguidesf))

### 2e. Roof report
- Look for remaining life estimate, flashing and parapet condition, and flat-roof drainage. Cost context in §3.

### 2f. Sewer lateral video
- SF has no point-of-sale lateral certificate (unlike San Mateo/Burlingame), so a video is voluntary but standard in SF packets. ([Otter Squad](https://ottersquad.com/sewer-lateral-rules-san-francisco/) (vendor), [Burlingame Properties](https://burlingameproperties.com/articles/the-burlingame-hillsborough-and-san-mateo-sewer-lateral-tests))
- Ownership split: upper lateral (house side) is fully the owner's; lower lateral (to the main) owner maintains, but if a plumber finds defects, report to SFPUC via 311 and SFPUC "under most circumstances" repairs at no cost. ([SFPUC](https://www.sfpuc.gov/learning/emergency-preparedness/sewer-laterals) (official))
- Read for: offsets, root intrusion, cracks, "bellies" (sags holding water), clay vs. cast iron vs. replaced. Cost context in §3.

### 2g. Permit history — the 3R report
- What: Report of Residential Building Record, from DBI: authorized use/occupancy, zoning, number of units, building permit history, condo status. SF housing code requires the seller to provide it. Fee $379 per residential building ($388.48 online), 7–10 business days, valid one year. ([SF.gov](https://www.sf.gov/request3r) (official); older sources cite $286 — superseded.)
- What it doesn't do: "no physical examination of the property"; "contains no history of any plumbing or electrical permits"; makes no representation of legal compliance. ([Paulo Serna](https://www.pauloserna.com/guides/3r-report-san-francisco/) (agent), [sample 3R](https://sfpublicworks.org/sites/default/files/DBI%203R%20Report.pdf))
- Why it matters: it's the gap check between what the City authorized and what you see. "If authorized records list a two-family dwelling but you observe three kitchens … that discrepancy signals the need for further investigation." Confirm remodel permits show as completed/finaled, not just issued. Get plumbing/electrical permit history separately from DBI. ([Paulo Serna](https://www.pauloserna.com/guides/3r-report-san-francisco/), [Jen Ferland](https://www.jenferland.com/disclosureguidesf), [DBI records request](https://www.sf.gov/requestbuildingrecords))
- Checklist: authorized unit count vs. listed; year built; permits for every advertised improvement (kitchen, bath, "new" level, garage conversion, roof deck); open/expired permits; complaints/NOVs (check DBI online permit/complaint search).

### 2h. Condo HOA documents
- Package: budget, reserve study/funding plan, financials, CC&Rs, bylaws, house rules, recent minutes, litigation disclosures. ([Brandi Mayo](https://brandimayo.com/blog/sf-condo-hoas-what-fees-cover-and-why-they-matter) (agent), [Judson Gregory](https://judsongregory.com/buying/disclosures-hoa-documents/) (agent))
- Reserves: Civil Code §5570 requires an annual Assessment and Reserve Funding Disclosure Summary with percent funded and 5-year projections. ([davis-stirling.com §5570](https://www.davis-stirling.com/HOME/Statutes/Civil-Code-5570) (official statute text)) Rule of thumb: >70% funded is okay; <70% suggests under-planning. ([search summary; propfusion](https://www.propfusion.com/law-guide/california-reserve-study-requirements)) Another heuristic: ≥25% of annual budget going to reserves. ([Stephen Bartlett](https://stephenjbartlett.com/blog/understanding-hoas-and-dues-in-san-francisco-buildings) (agent)) Heuristics, not law.
- Minutes: read 12–24 months for special assessments being discussed, deferred projects (roof, elevator, seismic, facade), litigation, leaks, neighbor conflict — minutes "reveal ongoing issues, upcoming projects, and how neighbors interact." ([Jen Ferland](https://www.jenferland.com/disclosureguidesf)) For brick-foundation buildings, check whether the HOA has addressed it. ([Inside SF RE](https://insidesfre.com/should-you-buy-a-property-that-has-a-brick-foundation/))
- Balconies (SB 326, Civil Code §5551): condo associations had to complete wood-supported elevated-element inspections by Jan 1, 2025 (9-year cycle, licensed structural/civil engineer or architect). Ask for the report; missing one is a red flag and a likely assessment. ([Rimkus](https://rimkus.com/article/california-balcony-inspection/), [Bay Legal](https://baylegal.com/sb-326-for-hoas-condominium-balcony-inspection-requirements-after-january-2026/))
- Insurance: master policy covers structure/common areas, not your interior; CA HOAs increasingly carry $25k–$100k+ deductibles; if the master carries earthquake, deductibles can be huge (e.g. $250k spread over 50 units = $5k+ each). Check master deductible and buy HO-6 loss-assessment coverage to match. ([Latent Insurance](https://www.latentinsure.com/hoa-insurance-california/condo-ho6) (vendor), [Aegivra](https://aegivra.com/guides/california-condo-insurance) (vendor))
- Also: rental caps/leasing restrictions in CC&Rs; owner-occupancy ratio (affects financing); pet rules; parking deeded vs. assigned.

### 2i. TICs
- Documents: recorded TIC agreement, house rules, budget/reserves, minutes, insurance, title report, any master mortgage — reviewed by a TIC-savvy attorney. ([Warrin](https://warrin.com/blog/what-is-a-tic-in-san-francisco) (agent), [Suzanne Garcia](https://suzannegarciarealestate.com/blog/tic-financing-in-san-francisco-what-fractional-loans-mean) (agent))
- Fractional loan (each owner's own mortgage) vs. shared master mortgage (joint liability — harder to resell/refi). Lenders look at cross-default clauses. Expect 20–30% down and rates ~0.5–1% above condo loans. ([Suzanne Garcia](https://suzannegarciarealestate.com/blog/tic-financing-in-san-francisco-what-fractional-loans-mean), [Legacy RE](https://legacysfhomes.com/blog/tic-guide-sf))
- Condo conversion: severely restricted since 2013; tenant-occupied buildings frozen out in 2017; many TICs are permanent. Prior Ellis Act evictions can make lenders refuse. Only buildings of ≤4 units can convert. ([Legacy RE](https://legacysfhomes.com/blog/tic-guide-sf), [Housing Rights Committee](https://www.hrcsf.org/sale-of-building))
- Shared obligations (taxes, insurance, reserves) remain joint even with fractional loans — a delinquent co-owner pressures the rest. ([Suzanne Garcia](https://suzannegarciarealestate.com/blog/tic-financing-in-san-francisco-what-fractional-loans-mean))

### 2j. Multi-unit / tenant-occupied — Rent Board records
- Rent Board Housing Inventory: owners must report occupancy/rent for residential units. Ask for it and the rent roll. ([SF.gov Housing Inventory](https://www.sf.gov/rent-board-housing-inventory) (official))
- Estoppels: buyers send tenants estoppel forms; tenants generally aren't required to sign unless their lease says so. Sale doesn't allow eviction; seller/buyer must disclose that in writing. ([SF.gov disclosure of rights on sale](https://www.sf.gov/information--disclosure-rights-tenants-and-after-sale-rental-units) (official), [SFTU](https://sftu.org/sale/))
- Buyouts require a Rent Board disclosure before negotiation. ([HRC](https://www.hrcsf.org/sale-of-building))
- OMI: one unit per building; owner ≥25% ownership, must intend 36 months' residence and move in within 90 days; protected tenants (60+/disabled with 10+ years, terminally ill with 5+ years) mostly can't be OMI'd; no school-year OMI of families with kids. Relocation payments for 2026 up to ~$24,733 per unit base, more for seniors/disabled/children. ([SF.gov OMI](https://www.sf.gov/information--evictions-based-owner-or-relative-move) (official), [Christopher Lee](https://christopherleesf.com/owner-move-in-eviction-san-francisco/) (agent) — verify current relocation figure with the Rent Board.)
- Soft-story: wood-frame, ≥5 units, 3+ stories (or 2 over basement/crawl), pre-1978 → Mandatory Soft Story Retrofit Program. Check the city list and that a Certificate of Final Completion exists; non-compliance → placard, possible orders affecting renting/selling/financing. ([SF ESIP Soft Story](https://www.sfgov.org/sfc/esip/soft-story) (official), [DataSF map](https://data.sf.gov/Housing-and-Buildings/Map-of-Soft-Story-Properties/jwdp-cqyc) (official))

### 2k. Preliminary title report
- Easements, liens, CC&Rs, tax status; flag undisclosed liens or easements. ([Wynne Morgensen](https://wynnemorgensen.com/blog/the-san-francisco-disclosure-package-explained))

---

## 3. Red flags by system (with cost ranges sources give)

Costs are source-quoted ranges, mostly vendor pages; always get a bid for the specific house.

| System | What to look for | Cost context | Sources |
|---|---|---|---|
| Foundation — brick/unreinforced | Brick or crumbling mortar visible in garage/basement; common in pre-1906 and early-1900s stock. Engineering consensus: unreinforced brick needs replacement or reinforcement. Worse in liquefaction zones. | Full replacement quoted $80k–$100k+ (Bay Area), avg ~$160k in SF proper per one contractor, $150k–$250k+ per an SF agent. Footings ~$500/lf; tie to framing ~$100/lf. Interim options: parge coat or concrete buttresses; segmental reinforcement at corners. | [City Structural](https://www.citystructural.com/post/brick-foundations-why-they-re-bad-news-in-the-sf-bay-area-risks-costs-retrofit-guide) (vendor), [Inside SF RE](https://insidesfre.com/should-you-buy-a-property-that-has-a-brick-foundation/) (agent), [Fine Homebuilding forum](https://www.finehomebuilding.com/forum/to-replace-or-not-brick-foundation) |
| Foundation — settlement/cracks | Diagonal wall cracks, sloping floors, sticking doors/windows. | Typical SF foundation repair ~$4.4k–$15.8k (avg ~$9.9k) — not replacement. | [Angi SF](https://www.angi.com/articles/how-much-does-foundation-repair-cost/ca/san-francisco) (vendor aggregator), [Opendoor](https://www.opendoor.com/articles/what-to-look-for-when-buying-a-house) |
| Seismic — bolting/cripple walls | Raised-foundation pre-1980 house without anchor bolts/braced cripple walls. | Brace + bolt ~$3k–$7k Bay Area; state EBB grant up to $3,000 (eligible SF ZIPs). | [CA Earthquake Authority](https://www.earthquakeauthority.com/press-room/press-releases/2024/earthquake-brace-bolt-grants-now-available-to-more-eligible-california-homeowners) (official), [Custom Home](https://www.customhome.us/blog/earthquake-brace-bolt-program-guide) (vendor) |
| Seismic — living over garage (Sunset/Richmond/Bayview tract homes) | Wide garage opening, no steel frame/shear walls — "weak story." | ~$15k–$50k+; engineering $2k–$10k. State ESS grant up to $13,000. | [SF Chronicle](https://www.sfchronicle.com/bayarea/article/calif-offers-13k-single-family-home-earthquake-19839583.php) (press), [Custom Home](https://www.customhome.us/blog/soft-story-retrofit-guide-bay-area) (vendor) |
| Soft-story (5+ units) | On the city list without a CFC. | Mandatory; see §2j. | [SF ESIP](https://www.sfgov.org/sfc/esip/soft-story) (official) |
| Drainage / water intrusion | Efflorescence (white salts) on concrete, damp spots, standing water after rain, stains on garage walls, musty smell; stucco cracks, staining below windows, missing head flashing / weep screed (Sunset stucco stock). | Fixes range from gutters/grading to French drains/sump; no reliable SF range found. | [HBG Civil](https://www.hbgcivil.com/how-to-inspect-basement-for-water-intrusion/) (vendor), [360 Home Inspection SD](https://www.360homeinspectionsd.com/post/stucco-problems) (vendor, stucco generic), [SF Planning Sunset context statement](https://default.sfplanning.org/Preservation/sunset_survey/Adopted_SunsetHCS.pdf) (official — history of stucco over-garage tracts) |
| Roof | Remaining life, flat-roof ponding, parapets, flashing, multiple slopes/chimneys on Victorians. | SF replacement ~$15k–$45k (most $20k–$28k for 1,500–2,000 sf asphalt); torch-down flat roof ~$6k–$18k; flat $8k–$30k+. | [Royal Roofing](https://royalroofingca.com/roof-replacement-cost-in-san-francisco/) (vendor), [Roof Replacement CA](https://www.roofreplacementca.com/torch-down-flat-roofing-how-it-works-costs-and-when-to-replace-it/) (vendor), [Apollo](https://www.apolloroofingcompany.com/how-much-does-it-cost-to-replace-a-flat-roof-in-california/) (vendor) |
| Electrical — knob & tube | Two-prong ungrounded outlets, ceramic knobs/tubes in attic/basement, cloth wire, fuse box. Many insurers decline active K&T → lender won't fund. | Full Victorian rewire ~$20k–$40k (agent); $25k–$75k / ~$20/sf (vendor). One Mission sale needed a $40k credit. | [Bonnie Spindler](https://bonniespindler.com/blog/knob-and-tube-wiring-what-san-francisco-buyers-need-to-know-before-making-an-offer) (agent), [K&T vendor](https://knobandtubewiringreplacement.com/how-much-does-it-cost-to-replace-knob-and-tube-wiring-in-san-francisco/) (vendor) |
| Electrical — panel | Federal Pacific Stab-Lok or Zinsco: breakers can fail to trip; insurers flag. Undersized (e.g. 60–100A) service limits EV/heat pump. | Panel swap ~$2k–$4.5k national; more with service upgrade. No SF-specific figure found. | [Chicago Home Inspector](https://www.chicagohomeinspector.com/blog/flagged-electrical-panels-federal-pacific-zinsco-and-others/) (vendor), [HomeCostLab](https://homecostlab.com/guides/electrical-panel-replacement-cost-by-type/) |
| Plumbing — supply | Galvanized: ~40–50 yr life, corrodes inside → low pressure, leaks. | No SF range found. | [InterNACHI forum](https://forum.nachi.org/t/cast-iron-drain-pipe-life-expectancy/75951) (standards body forum) |
| Plumbing — drains | Cast iron: ~40–60 yr typical, fails from inside; oxidation at stack base, cracked joints. | No SF range found. | [InterNACHI forum](https://forum.nachi.org/t/cast-iron-drain-pipe-life-expectancy/75951) |
| Sewer lateral | Offsets, roots, cracks, bellies on video. | Bay Area replacement ~$8k–$18k; SFPUC typically fixes lower lateral free. | [Next Level Plumbing](https://www.nextlevelplumbingservices.com/blog/sewer-lateral-replacement-bay-area-cost/) (vendor), [SFPUC](https://www.sfpuc.gov/learning/emergency-preparedness/sewer-laterals) (official) |
| Windows | Rot at sills, painted-shut sashes, single pane; historic districts may require wood matching original profile. | Wood double-hung ~$1.1k–$1.5k each installed (range $800–$2.5k+); restoration often ≤ replacement. SF permit rules nearly cost one owner $40k. | [Insight Glass](https://www.insightglass.com/how-much-for-window-replacement-in-san-francisco/) (vendor), [Raven Restoration](https://www.ravenrestorationsf.com/raven-double-hung-windows) (vendor), [SF Chronicle](https://www.sfchronicle.com/realestate/article/permit-nightmare-wood-windows-rules-19567466.php) (press) |
| Dry rot | Pest Section 1; soft wood at windows, decks, stairs, under showers. | Priced in pest report. | [Brian Cooper](https://cooperfamilyrealestate.com/blog/california-section-1-section-2-pest-report-explained) |
| Asbestos / lead | Pre-1978 = assume lead paint; popcorn ceilings, vermiculite, old duct/pipe wrap = suspect asbestos. Matters most if you plan to renovate. | Popcorn abatement ~$3–$8/sf (+$1–$3 refinish); lead abatement ~$8–$15/sf of surface. | [InspectandTest](https://inspectandtest.net/asbestos-lead/) (vendor), [Bonnie Spindler](https://bonniespindler.com/blog/lead-paint-and-asbestos-disclosures-what-california-buyers-of-historic-homes-must-know) (agent) |
| Retaining walls | Leaning, cracking, weep holes not draining; hillside lots (Bernal, Twin Peaks, Potrero). >4 ft (or >3 ft in landslide zones) needs permit/geotech. | $2.5k–$15k+ small walls (vendor); up to ~$1,000/lf for structural walls. | [Tamate Landscaping](https://tamatelandscaping.com/sf-retaining-wall-permits/) (vendor), [City Structural](https://www.citystructural.com/post/brick-foundations-why-they-re-bad-news-in-the-sf-bay-area-risks-costs-retrofit-guide) |
| Unpermitted additions / garage conversions | Ground-floor rooms not on 3R; low ceilings, no egress window, bathroom off garage. | Legalization cost varies; unit legalization may be "very hard and expensive." | [Rosdail](https://www.jenniferrosdail.com/unwarranted-space-in-law-units-and-renting-them-out-short-term-and-otherwise/), [SF Planning legalization](https://default.sfplanning.org/plans-and-programs/planning-for-the-city/accessory-dwelling-units/Legalization_FactSheet-091017.pdf) |

Insurance cross-cut: K&T and FPE/Zinsco can block a homeowner policy, which blocks the loan — get an insurance quote during diligence, not after. ([Bonnie Spindler](https://bonniespindler.com/blog/knob-and-tube-wiring-what-san-francisco-buyers-need-to-know-before-making-an-offer))

---

## 4. Showing / open-house walkthrough (≈20 minutes)

Bring: phone (photos), small flashlight, marble or small ball, outlet tester (optional). ([Opendoor](https://www.opendoor.com/articles/what-to-look-for-when-buying-a-house), [Redfin checklist](https://www.redfin.com/blog/open-house-checklist-for-buyers/))

Suggested sequence (synthesis of the above sources + SF-specific systems):

1. Outside first (2 min): stucco/siding cracks, stains under windows, roofline sag, retaining walls, lot slope toward house, street noise/traffic, neighbors' condition, which way the main windows face.
2. Nose on entry (instant): musty/earthy = moisture; heavy air freshener or candles can mask. ([Opendoor](https://www.opendoor.com/articles/what-to-look-for-when-buying-a-house))
3. Garage/basement (5 min — the most important stop in SF): foundation material (brick vs. concrete), cracks, efflorescence, damp spots, anchor bolts at sill, steel/plywood at garage opening, electrical panel brand/amperage, fuse box, K&T in exposed framing, water heater with two straps, galvanized vs. copper supply, cast iron drain stack condition, sewer cleanout, signs of a ground-floor room built in.
4. Floors and doors (throughout): marble roll test; doors/windows that stick; diagonal cracks from door/window corners → settlement. ([Opendoor](https://www.opendoor.com/articles/what-to-look-for-when-buying-a-house))
5. Ceilings and under sinks: water stains (especially under bathrooms and below roof/decks), warped floors; run faucets/shower for pressure and drainage. ([Opendoor](https://www.opendoor.com/articles/what-to-look-for-when-buying-a-house))
6. Electrical: two-prong outlets, flip switches.
7. Windows: rot at sills, painted shut, single pane; open one to hear the street.
8. Rooms vs. listing: count legal bedrooms (closet, egress window); is the "bedroom" down a garage stair?
9. Light: note time and fog; plan a second visit at a different time. Walk the block at night (general practice).

Questions to ask the listing agent:
- When are offers due? Will the seller consider pre-emptive offers, and on what terms? (Always ask before sending one; listing agents must present all offers promptly.) ([Front Steps](https://thefrontsteps.com/2025/01/14/navigating-multiple-offers-in-san-francisco-real-estate-scenario-1/), [Legacy RE preemptive](https://legacysfhomes.com/blog/preemptive-offers))
- How many disclosure packages have gone out, and how many offers do you expect? (Agents often share e.g. "25 packages, 5–10 offers.") ([Front Steps](https://thefrontsteps.com/2025/01/14/navigating-multiple-offers-in-san-francisco-real-estate-scenario-1/))
- Seller's preferred close timeline / rent-back needs? (Sellers weigh close speed alongside price.) ([Helena 7x7](https://www.helena7x7.com/sf-buyers-playbook-competitive-market/))
- Price guidance / what the seller hopes to see? Why selling? Has it been listed before? Any offers already rejected?
- What's not in the packet (plumbing/electrical permits, engineer reports, HOA balcony report)?

---

## 5. Evaluation framework

### 5a. Must-haves vs. nice-to-haves
- Prioritize location first, then a ranked feature list; judge whether the space works rather than the sqft number. ([Marina Times](https://www.marinatimes.com/lets-talk-square-foot-in-san-francisco-real-estate))
- Have the buyer write ≤5 must-haves (dealbreakers) and score nice-to-haves 1–3 by importance before touring.

### 5b. Fixable vs. unfixable (synthesis; widely held agent heuristic)
- Unfixable or very expensive: location/block, light/exposure, noise, lot size/slope, layout that requires moving stairs/structure, legal unit count, tenant status (under SF rent control), liquefaction/landslide zone.
- Fixable with money (price it in): foundation, seismic, roof, rewire, sewer lateral, windows, kitchens/baths, paint, flooring.
- Fixable only with permission/time: unwarranted space legalization, condo conversion (often impossible for TICs), HOA-level problems (you're one vote).

### 5c. Total cost of ownership (first-year and monthly)
Up-front:
- Price + closing costs + deferred maintenance (sum of priced Section 1 + known big-ticket items from §3, using bids or the ranges above, with a contingency for "further inspection" items).
- Transfer tax is customarily the seller's in SF (negotiable; buyer often pays in probate): 0.68% for $250k–$1M, 0.75% for $1M–$5M, 2.25% for $5M–$10M, applied to the whole price (full table in `buying-in-sf.md`). ([Burgelman Homes](https://burgelmanhomes.com/blog/san-francisco-transfer-tax-rates-credits-and-who-pays) (agent), [Inside SF RE](https://insidesfre.com/probate-sale-primer-for-sf-home-buyers/))

Monthly (PITI + HOA + reserve):
- Principal + interest at the buyer's quoted rate (don't assume a rate).
- Property tax: SF FY2025–26 secured rate 1.18268% of assessed value (price, under Prop 13), plus any direct assessments on the bill. ([EON RE](https://eonre.com/blog/property-tax-2026) (agent), [Burgelman](https://burgelmanhomes.com/blog/property-tax-rate))
- Insurance: homeowner (higher/unavailable with K&T, FPE/Zinsco), earthquake if wanted, HO-6 + loss assessment for condos. K&T example: $2,000/yr policy → $2,600–$3,000. ([K&T vendor](https://knobandtubewiringreplacement.com/how-knob-and-tube-wiring-affects-home-insurance-in-san-francisco/))
- HOA dues (condo) / TIC monthly share; plus expected special assessments from minutes.
- Maintenance reserve for the building age (state the assumption; don't invent a %).
- For tenant-occupied: subtract rent received, and model the cost of possession (relocation payments, OMI constraints) if the buyer intends to occupy.

### 5d. Deciding an offer price
1. Estimate value from comps (§1e) as a range; state how many comps back it.
2. Subtract deferred maintenance the comps didn't carry (a turnkey comp vs. this house with K&T and a brick foundation).
3. Read competition: neighborhood sale-to-list norm, disclosure packages out, expected offers, pre-emptive signals. Houses averaged 124.4% of list in May 2026 vs. condos 101.4% — so list price is a weak anchor for houses. ([Helena 7x7](https://www.helena7x7.com/sf-buyers-playbook-competitive-market/))
4. Set a walk-away number before offer day: SF offers are sealed and simultaneous; "there usually is no second chance to come back with a better number." ([Helena 7x7](https://www.helena7x7.com/sf-buyers-playbook-competitive-market/))
5. Terms are price too: contingencies (financing, appraisal, inspection), close speed, rent-back. Waive only what the buyer's reserves can absorb; escalation clauses reveal your max. ([Helena 7x7](https://www.helena7x7.com/sf-buyers-playbook-competitive-market/))
6. Pre-emptive offers must "blow the socks off" to stop the auction. ([SF Standard](https://sfstandard.com/2025/10/03/home-price-strategies-san-francisco/))
7. Negotiation levers after inspection: seller repairs, price reduction/credit, escrow holdback, or cancellation within contingency. ([Wynne Morgensen](https://wynnemorgensen.com/blog/the-san-francisco-disclosure-package-explained))

---

## Gaps / things not found or low confidence

- No authoritative SF-specific cost ranges for galvanized repipe, cast-iron drain replacement, panel upgrades, or drainage systems — only national or vendor figures. Say "get a bid" rather than quote.
- Brick foundation replacement figures vary 2x between sources ($80k–$250k+); present as a range and recommend a structural engineer.
- OMI relocation figure ($24,733 base for 2026) is from an agent blog; verify against the Rent Board's current schedule.
- HOA "percent funded >70%" and "≥25% of budget to reserves" are industry heuristics, not legal thresholds.
- Photo-reading and "walk the street at night" guidance is general practice; no single authoritative source.
- Couldn't fetch Deniz Kahramaner's "How to Read a SF Pest Report" (Medium 403) — [link](https://denizkahramaner.medium.com/how-to-read-a-san-francisco-pest-report-7016407c2044) for manual reading.
