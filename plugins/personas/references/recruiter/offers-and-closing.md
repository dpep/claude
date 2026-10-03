# Offers and Closing

Reference for the `recruiter` persona: closing a candidate from first contact through start date — tracking motivators, the pre-close, constructing and delivering the offer, references, the US legal steps around an offer, counteroffers and reneges, and pre-boarding. Live comp negotiation tactics belong to the `negotiator` persona; this covers what the offer *is* and the process around it. As of October 2026. Every claim is tagged:

- **[D]** documented — statute, government agency, peer-reviewed study, or a law firm quoting the statute
- **[P]** practitioner — recruiting blogs, vendor content, ATS benchmarks; real but self-interested or anecdotal
- **[?]** conflicting or unverified — verify before stating as fact

Legal notes are orientation, not legal advice. Employment law is state- and city-specific and moves yearly; anything marked "counsel/HR" should go to them before it goes in a letter.

---

## 1. Closing starts at first contact

The offer call should confirm a decision the candidate has mostly already made. If the number is a surprise, the close failed earlier.

### Keep a running close file per candidate
Update after every touchpoint (screen, each interview, every recruiter check-in):
- **Motivators** — ask directly: "What would make this an easy yes?" / "What would you need to see to leave where you are?" Rank them; candidates usually have one or two that dominate (scope, manager, mission, comp, flexibility, title).
- **Concerns** — what would make them say no. Ask again late; concerns change after they meet the team.
- **Competing processes** — companies, stage, expected offer dates. Ask each check-in: "Anything moved on your side?"
- **Timeline constraints** — vesting cliffs, bonus payout dates, notice period, visa timing, family/relocation.
- **Comp expectations** — what they're *targeting* (legal to ask everywhere), never what they *earn* where salary history is banned (§6).
- **Decision-makers** — partner, family, mentor. If someone else has a veto, find out what they care about.

### Who should make the call
- **Hiring manager** delivers the offer for most engineering roles: the candidate is choosing a manager as much as a company, and the HM can speak to scope and growth with authority. [P]
- **Recruiter** owns logistics, the written offer, and the comp mechanics; often runs the pre-close and every follow-up. Two voices, one message — agree the number and the flex before either talks to the candidate.
- **Exec / skip-level** call is a closing tool, not a default. Use it for senior hires, or when the dominant concern is strategy, funding, or company direction that only an exec can credibly address. Spent on everyone, it stops signalling anything.
- **Future peers** are strong for "what's it actually like" concerns; a candidate believes an engineer about on-call and code quality more than a manager.

### The pre-close (before the offer exists)
- One conversation, recruiter or HM, after the debrief says yes and before comp approval is final: "We're working on an offer. If it came in around X–Y, with Z in equity, is there anything that would stop you accepting?" [P]
- Confirms: the number lands, the level is understood, no new competing process, start-date constraints, anything they'd need in writing.
- Purpose is *no surprises in either direction*. The candidate shouldn't be shocked by the number; you shouldn't be shocked by a competing offer you never heard about.
- If the pre-close surfaces a gap, fix it before the call (re-level? more equity? sign-on to cover forfeited bonus?) or explain honestly why you can't. Hand live back-and-forth to the `negotiator` persona.

### Sell with specifics
- Map each ranked motivator to concrete evidence: the first project and why it matters, who they'd work with (by name), how decisions get made, the roadmap they'd own, the promotion path at this level.
- Concede real weaknesses. A candidate who hears the tradeoffs from you trusts the rest; one who discovers them in week three reneges or leaves.
- Don't oversell growth or equity. It's the most common source of early regret and it lands on the HM, not the recruiter.

---

## 2. Constructing the offer

Order matters: **level → base within band → equity → sign-on → everything else.** Level drives the band, the equity guideline, and the expectations the person is measured against for the next year.

### Level first
- Level on the interview evidence against the rubric, not on the candidate's current title or what makes the number work. Up-levelling to fit comp creates a performance problem; down-levelling to save money creates a retention problem.
- If comp can't close at the right level, the honest levers are position-in-band, equity, and sign-on — not the level.

### Base salary within the band
- Place by evidence: interview strength relative to the level bar, scarcity of the skill, and where existing peers at that level sit.
- Leave room to grow in band — a hire at the top of band has no merit headroom without a promotion.
- **Internal equity.** Before finalising, compare against current employees at the same level and location. Breaking band for one hire means (a) existing peers are now underpaid relative to a newcomer, which surfaces eventually through pay-transparency postings and pay-data reports; (b) the next candidate's anchor moves; (c) the exception becomes the band. If the market has moved, move the band for everyone. [P] Pay-transparency laws (§6) make posted ranges public and visible to current staff, so out-of-band hires are much harder to keep quiet than they used to be. [D, by implication of the posting requirements]

### Equity
**RSUs (public or late-stage private)** — grant of shares that vest over time; taxed as income at vest. Value is roughly shares × current price; the honest caveat is price volatility. Private-company RSUs often have a double trigger (time-vest plus a liquidity event) so they aren't taxed before they can be sold. [P]

**Stock options (startups)** — the right to buy shares at a fixed strike price.
- **Strike price** must be at least fair market value on the grant date to avoid IRC §409A penalties; private companies set FMV via an independent **409A valuation**, which also gives them a presumption of reasonableness. [D, IRC §409A / Treas. Reg. 1.409A-1(b)(5)(iv)] A 409A value is typically well below the preferred-share price in the last round, because common stock lacks the preferences. [P]
- **Vesting** — the common structure is four years with a one-year cliff, monthly thereafter. [P] Some companies use back-weighted or one-year-refresh schedules; state the schedule plainly.
- **ISO vs NSO** — ISOs get favourable tax treatment only if exercised within **three months of leaving** (12 months for disability). [D, IRC §422; [mystockoptions](https://www.mystockoptions.com/content/no-longer-with-my-company-job-loss-disability-death-do-statutory-deadlines-dictate-ISO-exercise)] Some companies extend the post-termination exercise window to 1–10 years; anything exercised after 90 days is taxed as an NSO regardless. [P] ([Cooley GO](https://www.cooleygo.com/extending-post-termination-option-exercise-periods-what-you-should-know/))
- The **exercise window** matters to a candidate more than most realise: a 90-day window means leaving requires writing a cheque (strike × shares, plus possible AMT) or forfeiting.

**Explaining equity value honestly**
- Give the candidate the numbers they need to model it themselves: shares granted, strike, latest 409A FMV, fully diluted share count (or % ownership), last preferred price, liquidation-preference overhang if material, vesting, exercise window.
- Avoid single-number "your equity is worth $X" built on the last preferred price. That price buys preferences common doesn't get, and paper value ≠ liquidity. Show scenarios (e.g. exit at 1×, 3×, 10× last post-money) with the dilution caveat, and say plainly that most startup equity ends at zero or near it. [P]
- Never project a valuation the board hasn't set. If you wouldn't put it in writing, don't say it on the call.

### Sign-on bonus
- Main use: replace what the candidate forfeits (unvested equity, an upcoming annual bonus). Ask for the forfeiture specifics; sizing to that is defensible and keeps base intact.
- **Clawbacks are now regulated in some states:**
  - **California AB 692** (agreements signed on/after Jan 1, 2026): repayment of sign-on bonuses is restricted — must be in a **separate agreement** from the offer letter, with a **5-business-day** review period and notice of right to counsel, repayment **prorated**, no interest, repayment period capped at **two years**, and no repayment if the employer terminates without misconduct. [D] ([Pillsbury](https://www.pillsburylaw.com/en/news-and-insights/california-ab692-stay-or-pay-repayment-provisions-employment-agreements.html), [Mayer Brown](https://www.mayerbrown.com/en/insights/publications/2026/03/a-deeper-dive-into-californias-new-limitations-on-stay-or-pay-clauses-as-of-january-1-2026))
  - **New York Trapped at Work Act** (signed Dec 19, 2025; amended Feb 13, 2026; effective **Feb 13, 2027**): bars "employment promissory notes"; the amendments carve out sign-on bonus and relocation repayment under conditions, including none if the employee is terminated other than for misconduct. [D] ([Mayer Brown](https://www.mayerbrown.com/en/insights/publications/2026/02/updates-to-new-yorks-trapped-at-work-act), [Morgan Lewis](https://www.morganlewis.com/pubs/2025/12/new-york-state-enacts-trapped-at-work-act-to-prohibit-use-of-employment-promissory-notes)) Exact carve-out conditions: counsel/HR. [?]
  - Elsewhere, a prorated 12-month clawback for voluntary departure is a common pattern. [P] Route any repayment term through counsel.

### Bonus, benefits, start date, location
- **Bonus**: state target %, what it depends on (company/individual), proration for the first year, and that it isn't guaranteed unless it is.
- **Benefits**: summarise in the offer email (health, 401(k) match, PTO policy, parental leave); link the full guide. Candidates comparing offers often undercount these — make it easy.
- **Start date**: set realistically against notice period (two weeks is a norm, not a law; senior people often give more), vesting/bonus dates at the current employer, and visa timing.
- **Remote / relocation**: if comp is location-adjusted, state the location the offer is pegged to and what happens if they move. Relocation packages carry the same repayment considerations as sign-on.

### Comp data sources
Know what each source is so you can weigh it, and so you aren't blindsided when a candidate cites one:
- **[Levels.fyi](https://www.levels.fyi/)** — crowdsourced, level-mapped total comp for tech, strongest for big-tech and well-known startups; some entries verified with offer letters. Skews to high-paying employers and self-selected reporters. [P]
- **[Pave](https://www.pave.com/)** — benchmarks drawn from participating companies' HRIS/cap-table integrations; near-real-time, venture-backed company heavy. [P]
- **[Carta Total Compensation](https://carta.com/)** — benchmarks from companies on Carta's cap-table platform; strong on startup equity by stage. [P]
- **Radford (Aon)** — survey of participating companies' HR data, the long-standing reference for large tech comp committees; subscription. [P]
- **[Payscale](https://www.payscale.com/)** — mix of crowdsourced and employer-submitted data; broader than tech, weaker at senior engineering levels. [P]
- All of these are biased toward their customer base. Triangulate; don't quote any single source as "the market."

---

## 3. The offer call and the written offer

### Offer call
- Live voice or video, never the email first. HM delivers; recruiter on or immediately after.
- Lead with *why them* (specific interview evidence), then the role, then the numbers, then next steps.
- Don't ask for an answer on the call. Ask what questions they have and when they'll decide.
- Send the written summary within the hour.

### What goes in the offer letter
- Title, level (if levels are visible), manager, start date, location / remote status, exempt/non-exempt classification.
- Base salary (annualised and pay frequency), bonus target and terms, sign-on (reference the separate repayment agreement where required — see CA AB 692).
- Equity: number of shares/units, type (RSU/ISO/NSO), vesting schedule and cliff, "subject to board approval" and the plan documents. Strike is set at the grant date, not the offer date.
- Benefits summary by reference.
- **At-will statement**: either party may end employment at any time, with or without cause; nothing in the letter is a contract for a fixed term. Avoid language implying guaranteed duration ("permanent position", "annual" guarantees). [P] Montana is the notable exception to at-will after a probationary period (Wrongful Discharge from Employment Act). [D] Wording: counsel.
- **Contingencies**: background check (with FCRA steps, §6), reference checks if not yet done, Form I-9 employment-eligibility verification by day 3 of work, signing a confidentiality / invention-assignment agreement. [D for I-9 timing: [USCIS](https://www.uscis.gov/i-9-central)] In fair-chance jurisdictions the offer must be *conditional* before criminal history is asked about (§6).
- **Expiration date** — see below.
- No non-compete for California employees (§6); be cautious elsewhere.

### Expiration windows: reasonable beats exploding
- **Experimental evidence**: in lab job-offer games, proposers frequently chose exploding offers even though it lowered their own payoffs, mainly because responders who accepted exploding offers reciprocated negatively afterward; proposers anticipated the backlash and used them anyway. [D] (Lau, Bart, Bearden, Tsetlin, *Decision Analysis*, [SSRN](https://papers.ssrn.com/sol3/papers.cfm?abstract_id=1934128); [INSEAD Knowledge](https://knowledge.insead.edu/career/employee-disengagement-starts-job-offer))
- Sondak & Bazerman (1989, *OBHDP*) found exploding offers reduced matching quality in an experimental market by about 8–13%. [D, as summarised by [Quartz](https://qz.com/166271/employers-ruin-good-job-offers-with-strong-arm-tactics) / [PON](https://www.pon.harvard.edu/daily/negotiation-skills-daily/dear-negotiation-coach-dealing-exploding-offers-nb/); original not read]
- Niederle & Roth show market-wide norms on exploding offers change how early and how badly a market unravels. [D] ([Market Culture](https://web.stanford.edu/~niederle/MarketCulture.pdf))
- Campus-recruiting norm: NACE's advisory opinion treats one to two weeks as common and less as potentially undue pressure. [P, industry body] ([NACE](https://www.naceweb.org/career-development/organizational-structure/advisory-opinion-setting-reasonable-deadlines-for-job-offers))
- Ashby data (230K offers, 2021–Mar 2024): accepting candidates decided in ~2–3 days, decliners took ~6. [P] ([Ashby](https://www.ashbyhq.com/talent-trends-report/reports/2023-trends-report-offer-acceptance-rates)) Read as: speed correlates with acceptance; a long wait is information, not something to cut off.
- Practice: set a date (5–7 business days is a common experienced-hire window [P]), say why if there's a real reason (a backup candidate, a team start date), and extend for a stated reason ("I'm finishing another process that ends Friday"). Use the time to keep selling, not to threaten.

---

## 4. Reference checks

### Evidence
- Schmidt & Hunter (1998) put reference-check validity at about **.26** for job performance — modest. [D] ([summary](https://firstpersonnel.org/wp-content/uploads/2013/10/Summary-Schmidt-Hunter-1998.pdf)) Sackett et al. (2022) could not re-estimate it because the underlying studies lacked the data; treat .26 as old and uncertain. [D] ([Sackett et al.](https://gwern.net/doc/statistics/meta-analysis/2021-sackett.pdf), [Master International](https://www.master-hr.com/insights/new-study-providing-updated-validity-estimates/))
- Structured reference checks (fixed job-relevant dimensions, rating scales) discriminate better and reduce the leniency of global "would you rehire?" questions. [D] (Taylor et al. 2004, *Personnel Psychology* 57:745–772; [OPM on reference checking](https://www.opm.gov/policy-data-oversight/assessment-and-selection/other-assessment-methods/reference-checking/))
- Use references to **confirm or probe** specific interview signals, not as a pass/fail gate after the decision is made. Late-stage references that never change a decision are theatre.

### Structured questions (same set for every reference)
1. How did you work together, for how long, and at what levels?
2. What was their biggest contribution? What made it hard?
3. On a 1–10 scale against other engineers at their level you've worked with, where would you put them? What would make it a 10?
4. Where did they need the most support from you or the team?
5. How did they handle a disagreement about a technical direction? Give an example.
6. [Probe the specific concern from the debrief.]
7. If you were building a team tomorrow, would you hire them? For what role?
Listen for hedges and faint praise; a reference's silence on a dimension is data. [P]

### Backdoor (back-channel) references
- Contacting people the candidate didn't name. Common in tech; legally untested ground in most states but carries **defamation/privacy exposure** for the person speaking and real harm to a candidate whose current employer learns they're looking. [P] ([Development Guild](https://www.developmentguild.com/executive-search/3-reasons-to-avoid-backdoor-reference-checks/), [JRG Partners](https://www.jrgpartners.com/ethics-back-channel-referencing-executive-search/))
- Rules if you do it at all: **never** contact the current employer without explicit consent; prefer former colleagues; tell the candidate you may back-channel and with whom; weigh one negative back-channel as a reason to probe, not a verdict — it's an unstructured, unverified single rater. Note: a reference report compiled by a third-party agency can be an FCRA "investigative consumer report" with its own notice rules. [D] ([FTC](https://www.ftc.gov/business-guidance/resources/using-consumer-reports-what-employers-need-know))

---

## 5. Counteroffers, competing offers, reneges, declines

### Counteroffers from the current employer
- **The "80% of people who accept a counteroffer leave within 6/12 months" statistic has no traceable primary source.** Attributions (a 1960s study, the WSJ, the "National Employment Association", a software vendor) don't resolve to data; a recruiter who searched for it found none and offered a bounty for a rigorous study. [?] ([Ken Davies](https://www.linkedin.com/pulse/so-do-80-people-who-accept-counteroffers-really-leave-ken-davies), [PRL International](https://www.prlinternational.com/post/what-percentage-of-counter-offers-actually-get-accepted)) Don't quote it to a candidate.
- What you can honestly say: the reasons they started looking (scope, manager, growth) usually aren't fixed by money; ask "if they matched, would the reasons you started looking go away?" [P]
- Pre-empt it in the close file: ask early "If you resigned, would they counter? What would they offer?" — and prepare the candidate for the resignation conversation.
- If they take the counter, accept it gracefully and keep the door open; the evidence doesn't support "they'll be back," but some are.

### Exploding competing offers
- Ask for the deadline and the company. Options: accelerate your process (compress remaining interviews, same-week debrief), give your own honest timeline, or tell the candidate plainly you can't make their date. Don't make an exploding offer of your own to counter one.
- If they ask the other company for an extension, a reasonable employer often grants it. [P] ([PON](https://www.pon.harvard.edu/daily/negotiation-skills-daily/dear-negotiation-coach-dealing-exploding-offers-nb/))

### Reneges
- **Candidate reneges** (accepts, then backs out): almost always a counteroffer, a late competing offer, or cold feet during a long gap. Pre-boarding (§7) is the main control. Ask why; log it; don't burn the bridge.
- **Employer reneges** (rescinds an accepted offer): reputational damage is lasting and public, and in some circumstances a candidate who resigned in reliance may have a promissory-estoppel claim. [D-ish, varies by state; counsel] Rescission based on a background check must follow the FCRA / fair-chance steps (§6). If a headcount freeze forces it, tell them fast, by phone, from the HM, with whatever you can offer (severance-like payment, deferred start).

### Declines
- Ask why, once, without arguing: "It helps us get better — what tipped it?" Record the real reason (comp, level, role, competing company, process) — this is your best calibration data on bands and process.
- Keep warm: a short note, connect on LinkedIn, a check-in at 6–12 months. Silver-medal declines are a strong future pipeline. [P]

---

## 6. US legal checkpoints (as of October 2026)

Not legal advice. Each item is a prompt to involve counsel/HR, not a substitute.

### Salary history bans
- Around twenty states plus DC, and a set of cities/counties, restrict asking about or relying on pay history. State list per practitioner trackers: AL, CA, CO, CT, DE, DC, HI, IL, ME, MD, MA, MN, NV, NJ, NY, OR, RI, VT, VA, WA; local examples include NYC, Philadelphia, Kansas City, St. Louis, Columbus, Cincinnati, Toledo, Cleveland, Louisville, New Orleans, Salt Lake City. [P] ([Paycor](https://www.paycor.com/resource-center/articles/states-with-salary-history-bans/)) Scope differs: Alabama's law bars refusing to hire someone *for declining to provide* history rather than banning the question. [?, verify]
- Virginia's ban on seeking or relying on salary history took effect **July 1, 2026** (SB 215, signed Apr 22, 2026), alongside posting requirements, with a private right of action. [D] ([McGuireWoods](https://www.mcguirewoods.com/client-resources/alerts/2026/5/new-virginia-law-requires-employers-post-wage-ranges-prohibits-requesting-salary-history/), [Virginia DOLI](https://doli.virginia.gov/2026/07/01/employment-law-updates-new-legislation-protecting-virginia-workers-applies-beginning-july-1-2026/))
- Safe default for a multi-state team: **never ask current or past pay, anywhere**. Ask for target comp instead. If a candidate volunteers history, don't rely on it to set the offer — several laws prohibit reliance even when volunteered.

### Pay transparency at offer stage
- A growing set of jurisdictions require a good-faith range in postings (e.g. CA, CO, WA, NY, IL, MD, MN, VT, MA from Oct 29, 2025; NJ from June 1, 2025; VA from July 1, 2026; DE not until Sept 26, 2027). [P/D] ([Kelly](https://www.kellyservices.com/insights/pay-transparency-laws), [DLA Piper on VA](https://knowledge.dlapiper.com/dlapiperknowledge/globalemploymentlatestdevelopments/2026/virginia-pay-transparency-law-goes-into-effect-july-1-2026))
- Some require disclosure on request or before the offer even without a posting (e.g. Connecticut: on request or before an offer, whichever first; California: pay scale on reasonable request, Labor Code §432.3). [D/P]
- Practical rule: the offer should sit inside the range you posted for that role and location. An offer outside the posted range undercuts the "good faith" claim. [P]

### Background checks: FCRA sequence
Applies when a third party (a consumer reporting agency) prepares the report. [D] ([FTC: Using Consumer Reports](https://www.ftc.gov/business-guidance/resources/using-consumer-reports-what-employers-need-know))
1. **Disclosure** — clear and conspicuous, in a document consisting solely of the disclosure. Bundling waivers or extra text into it is a frequent class-action trigger. [D/P] ([SHRM](https://www.shrm.org/topics-tools/news/talent-acquisition/fcra-101-how-to-avoid-risky-background-checks))
2. **Written authorization** before ordering.
3. **Certification** to the agency that you complied and won't misuse it.
4. **Pre-adverse action** — before deciding against the person: copy of the report + "A Summary of Your Rights Under the FCRA", and time to dispute (five business days is a widely used practice, not a federal statutory number [P]).
5. **Adverse action notice** — agency contact info, that the agency didn't make the decision, right to dispute and to a free report within 60 days.
State laws (CA, NY, others) add their own notices; use your background-check vendor's state forms.

### Fair chance / ban-the-box
- **California Fair Chance Act** (Gov. Code §12952; employers with 5+ employees): no criminal-history question until **after a conditional offer**; then an individualized assessment; written preliminary notice with the conviction(s) relied on; at least **5 business days** to respond; written final decision. Civil Rights Council regulations tightened in Oct 2023. [D] ([CA Civil Rights Dept](https://calcivilrights.ca.gov/fair-chance-act/), [JDP](https://www.jdp.com/blog/californias-fair-chance-act-sees-amendments-what-to-know/)) LA, SF, and other localities add ordinances on top. [D] ([Seyfarth](https://www.seyfarth.com/news-insights/san-diego-county-joins-growing-list-of-california-jurisdictions-with-fair-chance-ordinances.html))
- NYC, Illinois, Washington and many others have their own versions with different timing. Design the process so criminal history is only ever checked post-conditional-offer, everywhere — it's compliant in the strictest places and costs little elsewhere.

### Non-competes
- **California**: void and now expressly unlawful to include or try to enforce (B&P §16600, §16600.1, §16600.5, effective Jan 1, 2024); employers had to notify current and former employees (employed after Jan 1, 2022) by **Feb 14, 2024** that such clauses were void; penalties up to $2,500 per violation under the UCL. [D] ([Goodwin](https://www.goodwinlaw.com/en/insights/publications/2024/01/alerts-practices-tsemnc-new-laws-reinforce-californias-hostility-to-non-competes-with-notice), [Crowell](https://www.crowell.com/en/insights/client-alerts/california-employers-did-you-meet-the-employee-noncompete-agreement-notice-deadline))
- **FTC Non-Compete Rule**: set aside nationwide by a Texas district court (Aug 20, 2024); the FTC moved to dismiss its appeals and acceded to vacatur on **Sept 5, 2025**. The rule is not in effect; the FTC now pursues targeted case-by-case enforcement. [D] ([FTC press release](https://www.ftc.gov/news-events/news/press-releases/2025/09/federal-trade-commission-files-accede-vacatur-non-compete-clause-rule), [FTC rule page](https://www.ftc.gov/legal-library/browse/rules/noncompete-rule))
- Elsewhere: state law governs, with wide variation (income thresholds, notice periods, outright bans in a few states). Ask whether the candidate is bound by one at their current employer — it affects start date and role scope. Counsel.

### Immigration and E-Verify
- **H-1B transfer (portability)**: under AC21, an H-1B worker in status can start with the new employer once the new petition is filed (properly received by USCIS), before approval. [D/P] ([Ellis](https://www.ellis.com/resources/understanding-h-1b-transfers)) Build filing time (LCA certification first, ~a week [P]) into the start date. Premium processing shortens adjudication but isn't needed to start.
- **$100,000 H-1B payment** (Proclamation of Sept 19, 2025, effective Sept 21, 2025): applies to new petitions for beneficiaries outside the US / needing consular processing; USCIS guidance exempts change-of-status, extension, and amendment petitions for people already in the US (including most F-1 → H-1B and transfers), unless that request is denied and the person must consular-process. [D] ([Baker Law](https://www.bakerlaw.com/insights/uscis-clarifies-when-100000-h-1b-fee-is-required-exempting-most-f-1-students/))
  - **Status Oct 2026 (moving fast — verify before relying):** D.D.C. upheld it (Dec 2025; D.C. Cir. appeal pending); D. Mass. vacated the implementing actions (June 8, 2026) and the First Circuit refused a stay (July 24, 2026), so it is **not currently being collected**; a Sept 18, 2026 proclamation extends the policy through Sept 21, 2027; DHS proposed (Aug 25, 2026) a separate ~$103K fee rule for cap-subject petitions — proposed only. [D/?] ([Borderless Counsel](https://www.borderlesscounsel.com/blog-news-and-updates/2026/9/29/the-100000-h-1b-fee-where-things-stand-and-what-may-come-next), [Yale OISS](https://oiss.yale.edu/news/presidential-proclamation-extends-h-1b-100000-fee-policy-fee-remains-blocked-by-court-order))
- Ask about sponsorship needs *early* (it's lawful to ask whether someone will now or in future require sponsorship), not at offer. Don't ask about citizenship or national origin. Immigration counsel owns the filing.
- **E-Verify**: case only **after** the offer is accepted and Form I-9 is completed; never to prescreen; create the case no later than the third business day after the start date. [D] ([E-Verify User Manual](https://www.e-verify.gov/book/export/html/2113)) E-Verify is mandatory for some states/federal contractors and voluntary elsewhere. [D]

---

## 7. Pre-boarding: accept to start

The gap is the highest renege risk window: the old employer counters, recruiters keep calling, and the excitement fades.
- **Contact cadence**: HM touchpoint weekly or so; recruiter on logistics. Short and useful, not performative. [P]
- **Give them the team early**: intro to their onboarding buddy, an invite to a team lunch or social (optional, clearly unpaid and unpressured), the team's public docs. Don't assign work before they're on payroll — wage-and-hour issue. [P/D]
- **Handle the resignation**: offer to help them script it; remind them a counter may come, and ask them to call you if it does.
- **Remove friction**: equipment shipped before day one, accounts ready, I-9 instructions, benefits enrolment dates, first paycheck timing.
- **First-week plan** sent before start: who they'll meet, setup, the first real (small) task, and the 30/60/90 outline. A concrete plan is the best signal that the role they were sold is real.
- **Long gaps** (>4 weeks): add a mid-gap check-in with the HM; long gaps are where reneges cluster. [P]

---

## 8. Templates

### Offer-call outline (HM-led, ~20 min)
1. "We'd love you to join. Here's why" — two or three specifics from the interviews.
2. The role: first project, team, who they'll work with, what success looks like at 6 months.
3. The offer: level/title, base, equity (shares, vesting, how to think about value), sign-on, bonus, start date. Pause after the numbers.
4. "What questions do you have? What would you want to see in writing?"
5. "What's your timeline, and is anything else in motion we should know about?"
6. Next steps: written summary today, expiration date and why, who to call (recruiter and HM both).

### Offer email summary
> Subject: Your offer to join [Team] at [Company]
>
> Hi [Name] — thanks for the time today. We're excited about you joining [team] to work on [first project]. Summary of what we discussed:
> - Role: [Title], [Level], reporting to [Manager]; [location / remote]
> - Base: $[amount]/year, paid [frequency]
> - Equity: [N] [RSUs/options], [4]-year vesting with a [1]-year cliff, subject to board approval[; strike set at the 409A value on grant date]
> - Sign-on: $[amount] [— repayment terms in a separate agreement]
> - Bonus: [target]% of base, [terms]
> - Benefits: [one line + link]
> - Start date: [date]
> - Contingent on: [background check, I-9, PIIA]
>
> The formal letter is attached / will arrive via [system]. We'd appreciate a decision by [date]; if you need more time, tell me and why — we'll work with you. Call me or [HM] with any questions.

### Decline acknowledgement
> Hi [Name] — thanks for letting us know, and for the care you put into the decision. We're disappointed, but we understand. If you're open to it, I'd value hearing what tipped it — it helps us improve. I'd like to stay in touch; I'll check in in a few months, and if anything changes on your side, my door's open. Best of luck at [Company].

### Close the loop (candidates who didn't get it)
> Hi [Name] — thank you for the time you spent with us on the [role] process. We've decided to move forward with another candidate. This was a close decision, and it reflects the specific needs of this role right now rather than [strengths: e.g. "the depth you showed in the systems design discussion"]. [If true:] We'd like to keep in touch for future roles that fit — may I reach out? Thanks again, and best of luck.

Send within a few days of the decision, by email for early-stage candidates and by phone first for finalists. Don't give a reason you wouldn't defend in writing; specific feedback is a judgement call for HR/counsel. [P]
