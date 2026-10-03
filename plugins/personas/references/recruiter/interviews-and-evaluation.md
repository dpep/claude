# Interviews and Evaluation

Reference for the `recruiter` persona: what actually predicts job performance, how to build structured interviews, rubrics and loops for engineering roles, how to run debriefs, which bias interventions have evidence, and the US/EU legal limits, including the limits on AI in hiring that apply to this persona itself. As of October 2026. Every claim is tagged:

- **[D]** documented: statute, government agency, peer-reviewed study, meta-analysis
- **[P]** practitioner: recruiting blogs, vendor content, company self-reports, books built on one company's experience. Real, but self-interested or anecdotal
- **[?]** conflicting or unverified. Check before stating it as fact

Legal claims carry dates because they move. Validity coefficients are meta-analytic means with wide spread, not guarantees for any single interview.

---

## 1. What predicts job performance

### Schmidt & Hunter (1998) → Sackett et al. (2022)
- **Schmidt & Hunter (1998)**, *Psychological Bulletin* 124(2), was the canonical ranking for 25 years: work samples .54, GMA (cognitive ability) .51, structured interviews .51, job knowledge .48, unstructured interviews .38, reference checks .26, years of job experience .18, years of education .10, graphology .02. GMA was treated as the central predictor. [D]
- **Sackett, Zhang, Berry & Lievens (2022)**, [*J. Applied Psychology* 107(11), 2040–2068](https://gwern.net/doc/statistics/meta-analysis/2021-sackett.pdf), argue that earlier meta-analyses **overcorrected for range restriction**. They applied range-restriction estimates from predictive studies to concurrent studies (current employees), which inflated the validities. Revised operational validities (Table 3) [D]:

| Predictor | S&H 1998 | Sackett 2022 | SD of ρ | Lower 80% CV | Black–White *d* |
|---|---|---|---|---|---|
| Structured interview | .51 | **.42** | .19 | .18 | .23 |
| Job knowledge test | .48 | .40 | .13 | .23 | .54 |
| Empirically keyed biodata | .35 | .38 | .09 | .26 | .33 |
| Work sample test | .54 | .33 | .09 | .21 | .67 |
| Cognitive ability (GMA) | .51 | .31 | .14 | .13 | .79 |
| Integrity test | .41 | .31 | .20 | .05 | .10 |
| Assessment center | .37 | .29 | .09 | .17 | .52 |
| SJT (knowledge) | — | .26 | .10 | .13 | .39 |
| Conscientiousness (overall) | .31 | .19 | .15 | .02 | −.07 |
| Unstructured interview | .38 | **.19** | .16 | −.01 | .32 |
| Job experience (years) | .18 | **.07** | .11 | −.07 | .49 |
| Reference checks, years of education | .26, .10 | excluded: too little data | | | |

- **Read it as**: the top four (structured interview, job knowledge, keyed biodata, work sample) are all **job-specific** measures; "sample beats sign." The mean of the top five fell from .49 to .37. [D] (same)
- **Caveats** [D] (same):
  - Structured interviews have the highest mean but also a large SD (.19). A poorly built "structured" interview can perform far below .42. Ranked by the *lower* credibility bound, the order is keyed biodata, contextualized conscientiousness, job knowledge, work samples, then structured interviews.
  - Work samples and job knowledge tests assume prior training. They suit experienced hires and suit new grads less well.
  - Work samples, job knowledge and GMA show large Black–White differences (adverse-impact risk). Structured interviews, biodata and integrity tests show much smaller ones. Combining predictors reduces group differences.
- **The debate isn't settled.** Oh, Le & Roth (2023), *JAP* 108(8), 1300–1310, argue that substantial range restriction in concurrent studies is more common than Sackett assumes, so GMA's drop may be overstated. [?] The rank order at the top is less contested than the magnitudes. Sackett et al.'s follow-up [IOP focal article (2023)](https://www.cambridge.org/core/journals/industrial-and-organizational-psychology/article/revisiting-the-design-of-selection-systems-in-light-of-new-findings-regarding-the-validity-of-widely-used-predictors/A20984B138319E3D432E643978BF026D) cites 21st-century GMA studies with a mean around .23. [D] ([SIOP summary](https://www.siop.org/tip-article/is-cognitive-ability-the-best-predictor-of-job-performance-new-research-says-its-time-to-think-again/))

### What this means for an engineering loop
- **Lead with structured interviews and job-relevant work samples.** Both are near the top, and structure is the cheapest lever you control. [D]
- **Years of experience is a weak predictor (.07).** A "7+ years required" filter screens mostly on tenure. Specify the *capabilities* the years were meant to stand in for. [D]
- **Education and pedigree:** too little modern data to estimate. Google's people-ops head said GPAs and test scores "don't predict anything" beyond new grads. [P] ([ERE on the 2013 NYT interview](https://www.ere.net/articles/googles-weird-interview-questions-a-complete-waste-of-time))
- **Reference checks:** the only estimate is the dated 1998 .26, too thin to rank. Use them to verify facts and probe specific concerns, not as a scored predictor. [D/?]
- **Unstructured "conversations" (.19, credibility interval crossing zero)** are the weakest common practice still in wide use. [D]

### Myths to call out
- **Brainteasers** ("how many golf balls fit in a bus"): Google's own analysis called them "a complete waste of time … they serve primarily to make the interviewer feel smart." [P] ([ERE](https://www.ere.net/articles/googles-weird-interview-questions-a-complete-waste-of-time)) Interviewers higher in narcissism and sadism were more likely to want to use them. [D] ([Highhouse, Nye & Zhang 2019, *Applied Psychology*](https://iaap-journals.onlinelibrary.wiley.com/doi/10.1111/apps.12163))
- **"Gut feel" / "I know in five minutes":** impressions formed during rapport-building small talk predict later interview ratings and offers ([Barrick, Swider & Stewart 2010, *JAP* 95](https://www.psychologicalscience.org/observer/studying-first-impressions-what-to-consider)). Interviewers then ask questions that confirm those impressions ([Dougherty, Turban & Callender 1994, *JAP* 79](https://scirp.org/reference/referencespapers?referenceid=3993620)). A fast verdict is a contaminant, not a skill. [D]
- **"Airport test" / "culture fit" / "beer test":** in elite firms, "fit" turned out to mean shared leisure activities, self-presentation and background ([Rivera 2012, *Am. Sociological Review* 77(6)](https://www.researchgate.net/publication/273590506_Hiring_as_Cultural_Matching_The_Case_of_Elite_Professional_Service_Firms)). [D] Replace it with named, observable values-related behaviors ("gives direct feedback," "writes things down"), each scored like any other competency.

---

## 2. Structured interviewing

### What "structured" means
- Campion et al. (1997) listed 15 components. In practice structure usually means six of them: job analysis, same questions for every candidate, better question types, anchored rating scales, rating each question, multiple interviewers, and interviewer training. [D] ([Levashina, Hartwell, Morgeson & Campion 2014, *Personnel Psychology* 67](https://onlinelibrary.wiley.com/doi/abs/10.1111/peps.12052))
- Other components in the taxonomy: limit prompting and follow-up, no candidate questions until the end, no discussion between interviewers, take notes, and combine ratings statistically rather than by gut. Levashina et al. add limiting rapport-building, because it is where non-job information leaks in. [D] (same)
- Structure "greatly reduces group differences based on race, gender, and disability" and dampens the effect of candidate impression management. [D] (same)

### Behavioral vs. situational questions
- **Behavioral / past-behavior (PBQ):** "Tell me about a time you …". These measure experience. [D] (same)
- **Situational (SQ):** "What would you do if …". These mostly measure job knowledge. [D] (same)
- Both have acceptable validity. Some meta-analyses find SQ validity drops for high-complexity jobs (e.g. Huffcutt et al. 2004: SQ .27 → .18 low → high complexity, PBQ flat at ~.30, uncorrected) while PBQ holds. Levashina et al. tell researchers to stop arguing about which type is better and to use both. [D] (same) **For senior engineers, lean behavioral and use situational questions for scenarios they haven't faced.**
- Google's re:Work guide gives the same split: behavioral to validate claimed experience, hypothetical for novel problems. [P] ([re:Work, updated Mar 2026](https://rework.withgoogle.com/intl/en/guides/a-guide-to-structured-interviewing-for-better-hiring-practices))
- **Probing / follow-ups:** the evidence is unsettled. Probing may improve accuracy or may add bias. [?] (Levashina 2014) Practical rule: script the follow-ups ("What was your specific role?", "What happened next?", "What would you do differently?") and use the same ones for every candidate.

### Anchored rating scales (BARS)
- Anchored scales raised validity (.35 vs .26) and inter-rater reliability (.77 vs .73) in past-behavior interviews (Taylor & Small 2002 meta-analysis). With BARS, untrained raters matched job experts. Anchors at *every* point reduced disability bias more than anchors at the endpoints only (Reilly et al. 2006). [D] (Levashina 2014) Evidence on how many anchors to use and how to write them is thin. [D]
- Google uses anchored examples of "poor, borderline, solid, outstanding" answers per question. [P] ([re:Work](https://rework.withgoogle.com/intl/en/guides/a-guide-to-structured-interviewing-for-better-hiring-practices))

### Scoring and notes
- **Score each question right after the interview, before talking to anyone.** Weight the questions in advance. Iris Bohnet also recommends comparing candidates horizontally: score question 1 for every candidate, then question 2, and so on. [P/D] ([Bohnet, *What Works*, 2016](https://www.hup.harvard.edu/file/feeds/PDF/9780674986565_sample.pdf); [Quartz interview](https://qz.com/work/1080530/harvard-economist-iris-bohnet-says-to-eliminate-bias-its-easier-to-change-systems-than-change-people))
- Comparing candidates side by side cut gender bias compared with judging each alone ([Bohnet, van Geen & Bazerman 2016, *Management Science* 62(5)](https://gap.hks.harvard.edu/when-performance-trumps-gender-bias-joint-versus-separate-evaluation)). [D]
- **Notes record evidence, not impressions.** Write what the candidate *said and did*: "chose Postgres advisory locks and named the failover risk unprompted." Don't write "smart," "great energy," or "not senior." Every rating needs a quote or observation to back it. [P] ([re:Work](https://rework.withgoogle.com/intl/en/guides/a-guide-to-structured-interviewing-for-better-hiring-practices))
- Google reports that pre-made questions and rubrics save about 40 minutes per interview, and that rejected candidates were 35% more satisfied after structured interviews than unstructured ones. [P] (re:Work, self-reported)

---

## 3. Building the rubric from the role

**Chain:** role outcomes (what this person must deliver in 6–12 months) → **competencies** (3–6, no more) → **signals** (observable behaviors that show the competency) → **questions/exercises** that elicit those signals → **anchored 1–4 scale** per competency.

- **Start from the job, not a template.** Job analysis is the first component of structure, and it is also the legal basis for showing that a criterion is job-related (§7). [D]
- **Scale: 1 Strong no · 2 Lean no · 3 Lean yes · 4 Strong yes.** No neutral midpoint, so every interviewer has to commit to a direction. This is practitioner convention. [P] The survey-methods evidence on midpoints is mixed: removing one forces a choice but can push genuinely uncertain raters toward noise. [?] Mitigation: allow "insufficient signal" as an explicit non-score, which beats a fake 2.5.
- **Anchor every level** with a concrete behavioral description (see §8). Anchors at each point beat endpoint-only anchors. [D] (Reilly et al. 2006, via Levashina 2014)
- **Must-have gates vs. compensatory scoring.** Decide in advance which competencies are hurdles: a score ≤2 on these is a no regardless of other scores. Typical hurdles are core technical ability for the level and integrity/values conduct. The rest are compensatory (weighted sum). Mixing the two models after the fact is how a "but they were so strong at X" hire happens. [P; multiple-hurdle vs. compensatory is standard I-O selection design, D]
- **Weights:** set them before the first candidate, from the role definition. Bohnet recommends pre-assigned weights. [P/D] Equal weights are a fine default. Statistical combination beats holistic judgment (Campion component #15). [D]
- **"Bar" calibration:** write down which level is a hire at *this* level (e.g. senior = 3+ on design and execution). Without that, "lean yes" means different things to different interviewers. [P]

---

## 4. Loop design

### Coverage
- **Each competency has one owner interview** (two at most, if it is critical and you want a second independent read). Every interview has a written brief: competencies owned, questions, rubric. Overlap wastes candidate time. Gaps leave a competency scored on vibes in the debrief. [P]
- **Loop length:** Google's people-analytics team found four interviews gave about 86% of the predictive confidence, and panels of four reached the same decision as larger panels about 95% of the time. Google adopted a "rule of four." [P] (company self-report via [CNBC 2019](https://www.cnbc.com/2019/04/17/heres-how-many-google-job-interviews-it-takes-to-hire-a-googler.html)) Not peer-reviewed, but consistent with diminishing returns on extra raters.
- Typical senior engineer loop: recruiter screen → technical screen (60 min) → onsite or virtual of about 4 sessions (coding/work sample, system design, past-project deep dive, collaboration/values) → hiring manager. [P]
- **Candidate experience:** tell candidates the competencies and format up front. Knowing the dimensions in advance improves applicant reactions. [D] (Day & Carroll 2003, via Levashina 2014) Give a decision within days, and give a real reason when you reject. [P]

### Engineering exercise formats

| Format | Validity basis | Fairness / cost trade-off |
|---|---|---|
| **Live algorithmic coding (whiteboard/LeetCode)** | Weak job-relatedness unless the role is algorithmic | Being watched cut performance by more than half in an RCT (n=48 CS students). It measures performance anxiety as much as skill. [D] ([Behroozi et al., ESEC/FSE 2020](https://dl.acm.org/doi/10.1145/3368089.3409712)) |
| **Take-home** | A work sample, so in the strongest class (.33) if realistic | Less stress. Unpaid hours fall hardest on candidates with caregiving duties or second jobs. Can't verify who did the work without a follow-up review. [P] |
| **Pairing on a realistic task** | A work sample plus observed collaboration | Moderate stress. Needs a trained pair partner who scores against a rubric rather than "would I enjoy working with them." [P] |
| **Code review / debugging an existing codebase** | Job-knowledge and work-sample hybrid, close to the daily work | Short and realistic. Can be biased toward people who know your stack. Allow any language. [P] |
| **System design** | Situational interview for seniors | Prone to "did they say the words I would have said." Anchor on trade-off reasoning, not the reference answer. [P] |

- **Take-home norms** [P]: cap it at 2–4 hours and actually measure the time. Offer it as an alternative to live coding, not an extra hurdle on top. Pay for anything over about 2 hours, or for final-stage work. Follow with a 30–45 min discussion of the submission, which also authenticates it. Pay rates have no standard source. [?]
- **Accommodations:** offer format alternatives (extra time, take-home instead of live, captioning) on request. This is also an ADA obligation (§7). [D]

### Interviewer training, calibration, shadowing
- Training is one of the six usual components of structure, and it improves interviewers' acceptance of structure. [D] (Levashina 2014; [re:Work interviewer training](https://rework.withgoogle.com/intl/en/guides/effective-interviewer-training-for-better-candidate-experiences))
- **Shadow → reverse-shadow → solo:** the new interviewer observes 2+ sessions, then leads while a calibrated interviewer observes and both score independently, then compares ratings before running sessions alone. [P]
- **Calibrate quarterly:** have all interviewers score the same recorded or written answer and discuss differences against the anchors. Track each interviewer's score distribution and pass rate. Persistent outliers in either direction get recalibrated. [P]

---

## 5. Debriefs and decisions

- **Written, independent feedback before the debrief.** Submit it locked: no reading others' scores until yours is in. Discussing between interviews is a structure violation (Campion #13). [D] Kahneman, Sibony & Sunstein (*Noise*, 2021) make the same case: aggregate independent judgments before discussion, or the group amplifies one voice. [P/D]
- **Speaking order:** most junior, or lowest-tenure interviewer, first. Most senior and the hiring manager last. Otherwise everyone anchors on the person who controls their review. [P] (consistent with anchoring research [D])
- **Discussion is about evidence vs. the rubric:** "what did you observe for competency X, and which anchor does it match?" Questions about whether the person is "senior" or "a fit" in general don't count as evidence. Points of disagreement get resolved by looking at the evidence. If there isn't enough, record a gap; don't average the opinions. [P]
- **Changing a score in the debrief** requires citing new evidence you didn't have, and it is logged. [P]
- **Decision rules, fixed in advance:** e.g. "No hire if any gate competency ≤2 or if two or more interviewers are at 1. Hire if the weighted mean is ≥3 and the hiring manager concurs." The hiring manager owns the decision, but must write a rationale when overriding the panel. [P]
- **Amazon Bar Raiser:** a trained interviewer from outside the hiring team joins every loop and chairs the debrief, which keeps the assessment "open, accurate and fair." The stated bar is that a hire should be better than 50% of peers in similar roles. [P] ([About Amazon](https://www.aboutamazon.com/news/workplace/hire-power-how-amazonians-raise-the-bar-with-every-interview)) Veto power over the hire is widely reported by former employees but not stated in that source. [P/?]
- **Google hiring committee:** interviewers write evidence packets, and a separate committee that never met the candidate decides, which separates assessment from advocacy. [P] (Laszlo Bock, *Work Rules!*, 2015) Small-company version: one or two calibrated people outside the team review the packet before an offer.
- **Lightweight alternative for small teams:** one rotating "bar holder" from another team per loop, who has no stake in filling the seat. [P]

---

## 6. Bias: what's real, what works

### Known effects in interviews
- **Halo:** one strong trait raises ratings on unrelated dimensions (Thorndike 1920). Mitigation: score each competency separately against its anchors. [D]
- **Similarity / "culture fit":** see Rivera 2012 above. [D]
- **First impressions + confirmation:** see Barrick 2010 and Dougherty 1994 above. Mitigation: limit unstructured rapport time, and score from notes, not memory. [D]
- **Contrast effects:** a candidate seen right after a very weak or very strong one gets rated relative to that candidate. Mitigation: score against anchors, not the previous candidate. Bohnet's horizontal scoring addresses this directly. [D/P]
- **Anchoring on seniority:** see §5.

### Interventions with evidence
- **Structure itself**: same questions, anchored scales, independent scoring, statistical combination. It reduces group differences and improves validity. [D] (Levashina 2014; Sackett 2022)
- **Joint evaluation** (comparing candidates side by side, per question). [D] (Bohnet et al. 2016)
- **Accountability and ownership:** naming specific managers as responsible for diversity outcomes, mentoring programs, and voluntary training outperformed mandatory training. [D-ish] ([Dobbin & Kalev, HBR 2016](https://hbr.org/2016/07/why-diversity-programs-fail), summarizing their longitudinal firm-level research)

### Interventions with weak evidence
- **One-off implicit/unconscious-bias training.** A meta-analysis of 492 studies (n≈87k) found procedures change implicit measures only weakly (|d|<.30), "generally produced trivial changes in behavior," and that changes in implicit measures did not mediate behavior change ([Forscher, Lai et al. 2019, *JPSP* 117(3)](https://scholarworks.uark.edu/psycpub/1/)). [D]
- All nine tested interventions reduced implicit preference immediately. None lasted beyond hours to days ([Lai et al. 2016, *JEP: General*](https://pubmed.ncbi.nlm.nih.gov/27454041/)). [D]
- A preregistered field RCT (n=3,016) found online diversity training shifted attitudes but did not change behavior in the groups it most targeted ([Chang, Milkman et al. 2019, *PNAS*](https://www.ncbi.nlm.nih.gov/pmc/articles/PMC6475398/)). [D]
- After companies made diversity training mandatory for managers, management diversity did not improve over five years, and some groups' shares fell (e.g. Black women −9%). [D-ish] ([Dobbin & Kalev 2016](https://hbr.org/2016/07/why-diversity-programs-fail))
- **Bottom line:** change the *process*, not minds. Training is useful to teach the process (how to use the rubric), not as a bias cure.

---

## 7. Legal (US federal + selected states, EU)

*Not legal advice. Point the user to employment counsel for anything specific. Dates are when the rule took effect or when it was last checked, as of October 2026.*

### Questions you can't ask (US)
- **Title VII** (race, color, religion, sex including pregnancy, sexual orientation and gender identity per *Bostock*, 2020; national origin), **ADEA** (age 40+), **ADA** (disability), **GINA** (genetic information incl. family medical history), **PWFA** (pregnancy accommodation, eff. June 27, 2023). [D] ([EEOC prohibited practices](https://www.eeoc.gov/prohibited-employment-policiespractices))
- **ADA:** before a conditional offer, no disability-related questions or medical exams. You *may* ask whether the candidate can perform the essential functions, with or without reasonable accommodation. [D] ([EEOC pre-employment guidance](https://www.eeoc.gov/laws/guidance/enforcement-guidance-preemployment-disability-related-questions-and-medical))
- **Off-limits in practice:** age or graduation year, where someone was born or their "accent," citizenship (ask "Are you authorized to work in the US / will you need sponsorship?" instead), religion or holidays, marital or family status, plans for children, pregnancy, health, arrests, and in many states **salary history** (e.g. California Lab. Code §432.3, NYC, and others). [D] Small talk drifts into these topics. Structured interviews keep it from happening.
- **Ban the box / fair chance:** 37+ states, DC, and 150+ cities and counties have a fair-chance policy, 15 of which extend to private employers (as of 2025). [P] ([NELP](https://www.nelp.org/insights-research/ban-the-box-fair-chance-hiring-state-and-local-guide/)) Example: California's Fair Chance Act bars conviction-history inquiries before a conditional offer (employers with 5+ employees) and requires an individualized assessment before rescinding. [D] ([CA CRD](https://calcivilrights.ca.gov/fair-chance-act/))
- **Disparate impact still applies.** EO 14281 (Apr 23, 2025) directs federal agencies to deprioritize disparate-impact enforcement. Title VII's disparate-impact provision is statutory, though, so private suits and state agencies still use it. [D] ([Federal Register](https://www.govinfo.gov/content/pkg/FR-2025-04-28/html/2025-07378.htm); [SHRM](https://www.shrm.org/enterprise-solutions/insights/how-new-executive-order-affects-disparate-impact-enforcement)) The EEOC removed its AI technical-assistance documents on Jan 27, 2025. The guidance never created law, and the underlying statutes are unchanged. [D/P] ([Nat'l Law Review](https://natlawreview.com/article/federal-government-quietly-removed-its-ai-hiring-guidance-four-states-are-writing))

### AI in hiring
- **NYC Local Law 144 (enforced July 5, 2023).** An "automated employment decision tool" (AEDT) is a computational process that issues a score, classification or recommendation used to *substantially assist or replace* discretionary hiring decisions. Under DCWP rules that means relying on it alone, weighting it more than any other criterion, or using it to overrule humans. Requirements: an independent bias audit (impact ratios by sex and race/ethnicity) within one year before use, a published audit summary, and candidate notice at least 10 business days before use. Penalties are $500–$1,500 per violation. [D] ([DCWP](https://www.nyc.gov/site/dca/about/automated-employment-decision-tools.page); [rule](https://rules.cityofnewyork.us/rule/automated-employment-decision-tools-updated/)) A Dec 2025 NY State Comptroller audit called DCWP enforcement "ineffective," and DCWP has committed to tightening it. [D] ([OSC audit](https://www.osc.ny.gov/state-agencies/audits/2025/12/02/enforcement-local-law-144-automated-employment-decision-tools))
- **Illinois AI Video Interview Act (820 ILCS 42, eff. Jan 1, 2020).** If AI analyzes video interviews, the employer must give notice, explain what the AI evaluates, and obtain consent first. Videos may be shared only with people needed to evaluate them, and must be deleted within 30 days of a request. Employers that rely *solely* on AI to choose who gets an in-person interview must report demographic data annually. [D] ([Justia, 820 ILCS 42](https://law.justia.com/codes/illinois/chapter-820/act-820-ilcs-42/))
- **Illinois HB 3773 (Human Rights Act amendment, eff. Jan 1, 2026).** It is a civil-rights violation to use AI that has the effect of discriminating on protected classes in recruitment, hiring and other employment decisions, or to use zip code as a proxy. Employers must give notice when they use AI. IDHR withdrew its proposed notice rules in mid-2026, so the notice mechanics are unsettled. The prohibition is in force. [D/?] ([Seyfarth](https://www.seyfarth.com/news-insights/illinois-department-of-human-rights-temporarily-withdraws-proposed-rules-on-use-of-artificial-intelligence-in-employment.html); [Crowell](https://www.crowell.com/en/insights/client-alerts/artificial-intelligence-in-employment-update-illinois-requires-notice-and-prohibits-discriminatory-impact-in-use-of-ai))
- **Colorado.** The Colorado AI Act (SB 24-205), originally effective Feb 1, 2026, was pushed to June 30, 2026 (SB 25B-004). A federal court stayed its enforcement on Apr 27, 2026 (*xAI v. Colorado*), and it was **repealed and replaced by SB 26-189** (signed May 2026, **effective Jan 1, 2027**). The replacement is notice-based. For employment decisions made with ADMT, the employer must give notice at the point of interaction and, within 30 days of an adverse outcome, a plain-language explanation of the tool's role, plus data correction and human-review appeal rights. It drops SB 205's impact assessments and its affirmative duty to prevent algorithmic discrimination. Constitutional challenges were expected. [D/?] ([McDermott, May 27 2026](https://www.mcdermottlaw.com/insights/colorado-ai-law-in-flux-comprehensive-replacement-bill-signed-after-federal-court-blocks-predecessors-enforcement/)) Re-check the status before advising.
- **California** Civil Rights Council ADS regulations (eff. Oct 1, 2025, employers with 5+ employees): FEHA discrimination rules apply to automated-decision systems. Employers must keep ADS data for 4 years. Anti-bias testing is relevant evidence in a defense, and vendors can be liable as agents. [D] ([Mayer Brown](https://www.mayerbrown.com/en/insights/publications/2025/08/california-adopts-new-employment-ai-regulations-effective-october-1-2025))
- **EU AI Act.** Annex III classifies AI used for recruitment and selection (targeting job ads, filtering applications, evaluating candidates) as **high-risk**. The AI Omnibus (Reg. (EU) 2026/1744, in force July 27, 2026) moved Annex III high-risk obligations from Aug 2, 2026 to **Dec 2, 2027**. Article 50 transparency duties have applied since Aug 2, 2026: disclose that people are interacting with an AI, and disclose emotion recognition. Emotion recognition in the workplace has been **prohibited** since Feb 2, 2025. [D] ([EC](https://digital-strategy.ec.europa.eu/en/news/ai-omnibus-enters-force); [White & Case](https://www.whitecase.com/insight-alert/eu-ai-omnibus-enters-force-amending-ai-act); [Jones Walker](https://www.joneswalker.com/en/insights/blogs/ai-law-blog/yes-august-2-still-matters-the-eu-approved-a-high-risk-ai-delay-but-most-trans.html?id=102nbon))

### What this means for the persona (an AI)
- **Never be the decision-maker, and never produce candidate scores, rankings or pass/fail screens** that a human relies on alone, weights above other criteria, or uses to overrule interviewers. That is the NYC AEDT definition, and similar outputs trigger Illinois, California, Colorado (2027) and EU high-risk duties. [D]
- **Safe:** designing rubrics, questions and loops. Drafting job descriptions. Coaching interviewers. Helping the hiring manager *structure their own* evidence-based write-up. Pressure-testing a decision rationale for bias or missing evidence. Running a debrief agenda.
- **Not safe:** "rate these 40 résumés," "who should we advance," summarizing a candidate into a hire/no-hire, inferring traits from video, voice or writing style, and any protected-class inference. Redirect: offer the rubric and let the human score.
- If the user puts candidate data into the conversation, treat it as personal data. Don't retain or repeat it beyond the task.

---

## 8. Worked example: "Technical design judgment" (Senior Backend Engineer)

**Role outcome:** owns the design of services with clear failure modes, sound data models, and reasoned trade-offs that other engineers can extend. **Owner interview:** system design (60 min). **Gate competency:** yes, ≤2 is a no hire for senior.

**Signals:** states requirements and constraints before proposing anything. Names 2+ viable options and picks one for stated reasons. Anticipates failure modes (partial failure, retries/idempotency, data consistency, load). Sizes the solution to the problem. Updates the design when a constraint changes. Knows what they'd defer.

| Score | Anchor (observable) |
|---|---|
| **1 Strong no** | Jumps to a solution or technology without clarifying requirements. Can't name an alternative or a downside. Misses an obvious failure mode even when prompted. Design doesn't meet the stated requirement. |
| **2 Lean no** | Workable design for the happy path. Considers trade-offs only when prompted. Failure handling is generic ("add retries"). Over- or under-builds relative to scale and can't explain why. |
| **3 Lean yes** | Clarifies the key requirements first. Compares 2+ options with concrete trade-offs (consistency, latency, cost, operability). Raises the major failure modes unprompted and gives specific mitigations. Adapts cleanly when a constraint changes. |
| **4 Strong yes** | Everything in 3, plus: identifies the decision that is expensive to reverse and designs to keep it reversible. Sequences the work (what to build first, what to defer). Ties choices to team and operational cost. Gives a past example where a similar call went wrong and what they changed. |

**Questions (same for every candidate, with scripted follow-ups):**
1. *Situational:* "Design a service that sends webhook notifications to customers' endpoints at a few thousand per second. Some endpoints are slow or down." Follow-ups: "A customer says they got the same event twice, what happened?" · "Traffic grows 20× in a quarter, what breaks first?"
2. *Behavioral:* "Tell me about a design decision you made that you'd now make differently. What did you know at the time, and what changed?" Follow-ups: "What was your specific role?" · "How did you find out it was wrong?"
3. *Behavioral:* "Tell me about a time you argued *against* a more complex design. What did you propose, and what happened?"

**Note-taking prompt for the interviewer:** for each signal, write the candidate's words or the action they took, then map it to the anchor. "Insufficient signal" is a valid entry. Don't fill gaps from impressions.
