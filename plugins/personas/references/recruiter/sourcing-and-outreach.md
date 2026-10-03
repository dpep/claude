# Sourcing and Outreach

Reference for the `recruiter` persona: where engineering candidates come from, how to find the ones who don't apply, how to write to them so they answer, how to follow up and close the loop, which pipeline numbers mean anything at a single hiring manager's scale, and what privacy law says about the data you collect along the way. Written for a hiring manager at a US tech company. As of October 2026. Every claim is tagged:

- **[D]** documented: statute, government agency, peer-reviewed study
- **[P]** practitioner: recruiting blogs, vendor reports (ATS/CRM vendors publishing their own customer data). Real, but self-interested or anecdotal
- **[?]** conflicting or unverified: sources disagree, or only a weak source. Verify before you state it as fact

Vendor benchmarks are aggregates over thousands of companies, and mostly companies that bought a sourcing tool. Use them to find the shape of the curve, not to set targets for one req.

---

## 1. Channels and what they yield

### Referrals
- **The best-evidenced channel.** Nine large firms in call centers, trucking and high tech: referred applicants were more likely to be hired and more likely to accept offers despite similar measured skills. Productivity was similar on most measures (more patents in high tech, fewer accidents in trucking). Referred workers were "substantially less likely to quit," and they produced higher profit per worker, mostly through lower turnover and recruiting cost. [D] ([Burks, Cowgill, Hoffman & Housman, QJE 2015](https://econpapers.repec.org/article/oupqjecon/v_3a130_3ay_3a2015_3ai_3a2_3ap_3a805-839.htm))
- In a single US firm, referred candidates were more likely to be hired and stayed longer. Their initial wage premium faded over time, and the effects were stronger at lower skill levels. [D] ([Brown, Setren & Topa, J. Labor Econ. 2016](https://www.newyorkfed.org/research/staff_reports/sr568.html))
- Across studies, lower turnover is consistent, while the productivity evidence is mixed. Referral quality depends on the firm still screening referred candidates. [D] ([Hoffman, IZA World of Labor 2017](https://wol.iza.org/articles/the-value-of-hiring-through-employee-referrals-in-developed-countries/long))
- Ashby data: referrals are about 17–18% of hires, and that share holds regardless of how much inbound a job gets. 52% of referred candidates pass the initial screen, against 35% overall. [P] ([Ashby, Oct 2025](https://www.ashbyhq.com/talent-trends-report/reports/inbound))
- **The diversity cost is real.** People refer people like themselves, so a referral-heavy pipeline reproduces the team's existing demographics. Rubineau & Fernandez model referrer behavior as the mechanism that sustains job segregation. [D] ([Management Science 2013](https://dx.doi.org/10.1287/mnsc.2013.1717)) The IZA review records gender and race disadvantages in who receives referrals. [D] ([Hoffman 2017](https://wol.iza.org/articles/the-value-of-hiring-through-employee-referrals-in-developed-countries/long))
- In PayScale's 2017 survey (n≈53,200), compared with white men, white women were 12% less likely to have received a referral, men of color 26% less likely, and women of color 35% less likely. [P] (self-reported survey data, not peer reviewed) ([PayScale](https://www.payscale.com/compensation-trends/referrals-hurt-diversity/), [SHRM](https://www.shrm.org/resourcesandtools/hr-topics/employee-relations/pages/job-referrals-.aspx))
- **What to do about it:** use referrals, but don't fill a loop with them alone. Ask referrers for specific profiles ("who's the best on-call debugger you've worked with?") rather than "anyone looking?" Run sourced outreach alongside referrals so the slate isn't one network deep. Hold referred candidates to the same rubric. [P]

### Inbound applications
- Inbound volume has exploded. Applications per hire roughly tripled since 2021, and the average open role now gets 300+ applications. Americas technical roles average about 610 applications per hire. [P] ([HR Dive on Ashby data, 2026](https://www.hrdive.com/news/recruiters-see-job-applications-triple-to-more-than-300-per-role/820096/), [Ashby/PR Newswire](https://www.prnewswire.com/news-releases/new-data-from-ashby-reveals-emea-recruiters-are-absorbing-rising-application-volume-without-slowing-hiring-302871548.html))
- Inbound produced 52% of hires in Q2 2025, a four-year high. For executive roles the figure is about 35%. [P] ([Ashby](https://www.ashbyhq.com/talent-trends-report/reports/inbound))
- Inbound is the largest source of hires and the lowest-yield per candidate. The cost is reviewer time, and that cost is now mostly AI-assisted mass applications. Greenhouse reports recruiter workload up 26% in one quarter. [P] ([Greenhouse, Dec 2024](https://www.greenhouse.com/blog/greenhouse-2024-state-of-job-hunting-report))
- Screen inbound against the role's must-haves (section 2), not keywords. Respond to everyone (section 5).

### Outbound sourcing
- Tech roles get more of their hires from sourcing: about 24%, against 18% for business roles. For jobs with little inbound, sourcing is the top source of hires. [P] ([Ashby](https://www.ashbyhq.com/talent-trends-report/reports/inbound))
- Outbound is the only channel where you choose who enters the funnel. That makes it your main lever on slate quality and diversity.

### Communities, open source, conference speakers
- These people have public, inspectable work (commits, talks, posts, issue threads), so you can personalize in a way generic outreach can't. They also get the most recruiter spam. [P]
- Open source maintainers and speakers are usually employed and not looking. Expect lower response rates and longer cycles. The pitch is a specific problem, not a job ad. [P]
- Show up before you need something: sponsor or host a meetup, have engineers give talks, contribute upstream. Relationship lead time is months. [P]

### Alumni and boomerangs
- In one large healthcare organization (2,053 boomerangs, 10,858 new hires), returning employees outperformed external new hires in their first spell back. The edge was larger in coordination-heavy jobs. [D] ([Keller, Kehoe, Bidwell, Collings & Myer, Academy of Management Journal 2021](https://faculty.wharton.upenn.edu/wp-content/uploads/2021/12/In-with-the-old.pdf)) Other work finds boomerangs perform on par with employees who never left, not better. [D] ([EurekAlert summary](https://www.eurekalert.org/news-releases/761999))
- Treat good leavers as a pipeline. Know why they left, and whether that reason has changed.

### Agencies
- **Contingency** (paid only on hire): commonly 15–25% of first-year base, with 20% the most cited figure. Senior engineers run toward 22–25%. [P] ([techhiringcost.com](https://techhiringcost.com/contingency-recruiter-fee), [Pin](https://www.pin.com/blog/recruitment-agency-commission-structures/))
- **Retained / executive search**: 25–35%, often paid in thirds (at kickoff, at slate, at hire). [P] ([techhiringcost.com](https://techhiringcost.com/recruiter-fees), [Dover](https://www.dover.com/blog/contingency-recruiting-placement-fee-incentives))
- Before you sign, check: the guarantee or replacement period (often 60–90 days) [P], whether the fee base is base salary or total comp, and candidate ownership (how long a submitted resume "belongs" to the agency). The ownership term causes most fee disputes when the same person also comes in through a referral. [P]
- Contingency incentives favor speed and volume over fit, because the agency is paid only if it wins the race. Use agencies for hard-to-fill or confidential roles. Don't use them as a substitute for your own outreach. [P]

---

## 2. Sourcing technique

### Build the target profile from the role, not the JD
- Start from the role definition (see the intake reference): the problems this person solves in the first 6–12 months, and the 3–5 must-have capabilities that follow from those problems. The job description's wish list is a marketing document, and it over-filters.
- Turn each capability into **observable evidence**. "Has run a zero-downtime Postgres migration at scale" becomes a search for talks, blog posts, OSS commits, or prior employers with that problem.
- List **disqualifiers explicitly** (location or timezone, visa, level), so sourcing time doesn't go to profiles that will never pass.

### Calibrate before scaling
- Pull 5–10 sample profiles and have the hiring manager mark each one yes/no/maybe **with a reason**. The reasons are the real spec. Repeat until the HM's yes rate on your picks is high, then scale. [P]
- Recalibrate after the first 3–5 phone screens. Interview signal often reveals a must-have nobody wrote down.

### Adjacent searches
- **Adjacent titles:** "platform engineer" ≈ "infrastructure engineer" ≈ "SRE" ≈ "production engineer"; "staff engineer" vs "tech lead" vs "principal". Titles don't map across company sizes: a "senior" at a 20-person startup may be a "staff" elsewhere, and the reverse.
- **Adjacent industries:** the same technical problem shows up in other domains. Low-latency systems appear in adtech, trading and gaming; compliance-heavy data in fintech and health tech. Search by problem, not by industry.
- **Non-obvious pools:** bootcamp grads with 3+ years of experience, career changers, people at companies that just had layoffs, and returners after career breaks.

### X-ray and Boolean patterns
Run these in a general search engine, by hand. Combine `site:`, quoted phrases, `OR` and `-`.
```
site:linkedin.com/in ("platform engineer" OR "infrastructure engineer") (kubernetes OR k8s) ("San Francisco" OR remote) -recruiter
site:github.com "{technology}" "{city}"                          # profiles and READMEs
site:stackoverflow.com/users "{city}" "{tag}"
site:sessionize.com OR site:speakerdeck.com "{topic}"            # speaker profiles, slides
"{conference name}" {year} speakers "{topic}"                    # then read the talk list
site:medium.com OR site:dev.to "{specific technical problem}"     # people who wrote about it
```
- **GitHub's native search** (users by `location:` and `language:`, repos by topic, a repo's contributor list) is often better than X-ray for OSS. Look at the work. Stars and commit counts are poor proxies. [P]
- Conference talk lists, program committees, and maintainers of libraries your team actually uses are the highest-signal lists you can build. [P]

### LinkedIn tiers and limits
- **Free accounts** hit an undisclosed monthly "commercial use limit" on searches. LinkedIn won't display or raise it. Practitioners report free accounts running out mid-month. [P] ([noon.ai](https://www.noon.ai/blog/articles/242-linkedin-limits-for-recruiters)) The exact number is [?].
- **Recruiter Lite**: 30 InMail credits per month, which accumulate up to a cap. Response-rate-based credit refunds apply. [D-ish, vendor help docs] ([LinkedIn Help](https://www.linkedin.com/help/recruiter/answer/a1675961))
- **Recruiter (full)**: larger search, projects, and pipeline tools. LinkedIn may warn or restrict accounts whose InMail response rate falls below **13% over 14 days** (on 100+ InMails). Spray-and-pray is penalized directly. [D-ish] ([LinkedIn InMail policy](https://www.linkedin.com/help/recruiter/answer/a413279))
- A hiring manager's personal connection request with a note is free, comes from a real person, and often outperforms an InMail. Use it sparingly.

### Respect site terms: no scraping
- **LinkedIn** prohibits automated scraping in its user agreement. In *hiQ v. LinkedIn*, a court held in Nov 2022 that hiQ breached those terms by scraping and by using fake profiles. hiQ accepted a permanent injunction and $500K in damages. [D] ([Morgan Lewis](https://www.morganlewis.com/blogs/sourcingatmorganlewis/2022/12/linkedin-v-hiq-landmark-data-scraping-suit-provides-guidance-to-data-scrapers-and-web-operators), [Proskauer](https://newmedialaw.proskauer.com/2022/11/11/court-finds-hiq-breached-linkedins-terms-prohibiting-scraping-but-in-mixed-ruling-declines-to-grant-summary-judgment-to-either-party-as-to-certain-key-issues/))
- **GitHub's Acceptable Use Policy** forbids using information from the service "whether scraped, collected through our API, or obtained otherwise" for spam, "including for the purposes of sending unsolicited emails to users or selling personal information, such as to recruiters, headhunters, and job boards." [D] ([GitHub AUP §7](https://docs.github.com/en/site-policy/acceptable-use-policies/github-acceptable-use-policies)) So don't harvest commit emails in bulk. Individual, hand-researched, relevant outreach is the defensible line. Even then, use a contact address the person published for that kind of contact, and don't mine git logs for emails.
- No browser-automation tools that auto-visit profiles or auto-send connection requests. They break the terms and get accounts restricted.

---

## 3. Outreach that gets answered

### What the data says [P unless noted]
- **Personalization works.** LinkedIn: personalized InMails perform 20% better than bulk ones (2021 data), and 200–400 character InMails are 16% more likely to get a response. [P] ([LinkedIn Talent Solutions](https://business.linkedin.com/talent-solutions/resources/talent-acquisition/refine-outreach-inmail)) Ashby: campaigns with AI personalization tokens averaged 35.3% reply against 24.1% without. That is correlational; teams that personalize differ in other ways too. [P] ([Ashby](https://www.ashbyhq.com/talent-trends-report/reports/candidate-sourcing))
- **Warm signals:** candidates already connected to someone at your company are 46% more likely to accept an InMail (2015 data). Company followers are 81% more likely to respond. [P] ([LinkedIn](https://business.linkedin.com/talent-solutions/resources/talent-acquisition/refine-outreach-inmail)) Mention the mutual connection when it's real.
- **Engineering is a hard audience.** In Ashby's data, engineering reply rates (~21%) are among the lowest of all job families. [P] ([Ashby](https://www.ashbyhq.com/talent-trends-report/reports/candidate-sourcing))
- **Hiring-manager-sent vs recruiter-sent.** Every source agrees on the direction and none gives rigorous magnitude. Gem says sending on behalf of the HM "has been shown to double response rates" and that some customers "quadrupled" them, with no methodology given. [P/?] ([Gem](https://www.gem.com/blog/collaborative-recruiting), [Gem 2022 benchmarks](https://www.gem.com/blog/benchmarks-and-best-practices-for-recruiting-email)) LinkedIn cites a single HM case study at a 34% response rate. [P] No controlled comparison found. **Defensible claim:** HM-sent messages likely get meaningfully more replies, especially from senior engineers. Don't quote a multiplier.
- **Subject lines:** Gem found personalization tokens in the subject line raised open rates by 4.8% (2021–22, ~8M sequences). [P] Opens are a weak metric, though, because Apple Mail Privacy Protection inflates them. Optimize replies, not opens. [P]
- **Send time barely matters.** Gem's open rates for Sunday through Friday fell within 61.2–61.6%. [P] LinkedIn says weekday mornings work best and Saturday is worst. [P] Don't spend effort here.

### Rules for the message
1. **Lead with their work, specifically.** "Your talk on taming Kafka consumer lag at {Conf}" works. "Your impressive background" does not. If you can't name something specific, you haven't done the research and shouldn't send yet.
2. **Then the problem and who they'd work with.** Engineers respond to a hard problem and a credible manager more than to a company pitch.
3. **Include the comp range.** It respects their time and filters early. California, Colorado, New York, Washington and others require pay ranges in job postings. California also requires giving the pay scale to an applicant who asks. [D] ([CA SB 1162, Labor Code 432.3, eff. 2023 — Ogletree on the DIR FAQ](https://ogletree.com/insights-resources/blog-posts/california-labor-agency-posts-faqs-relating-to-new-pay-scale-posting-requirements/)) Putting it in outreach goes beyond the law, but it signals you aren't hiding anything.
4. **Keep it short:** 75–150 words for email, 200–400 characters for InMail. One idea per paragraph.
5. **Make a low-commitment ask** that is honest about what it is: "Open to a 20-minute call with me about the role?" Not "quick chat" when it's actually a screen.
6. **Make it easy to say no** ("If the timing's wrong, just say so and I won't follow up"). This raises trust, and with it the response rate.

### Avoid
- Generic flattery ("your impressive profile"), "rockstar/ninja/10x", walls of text, emoji-heavy subject lines.
- Fake urgency ("only 2 spots left," "need to hear back today").
- **Bait-and-switch:** a "casual chat" that turns out to be a technical screen, or a "we should connect" with no role mentioned. It burns trust with the candidate and their network.
- Mass-merge tells such as wrong name, wrong pronoun, `{FIRST_NAME}` left in, or praising a repo they forked but never committed to.
- Pitching a role clearly below their level. Check their current scope first.

---

## 4. Follow-up cadence

- **Follow-ups produce most replies.** Gem (Jun 2021–May 2022, ~8M sequences): cumulative reply rate was 8.3% after email 1, 15.8% after email 2, and 21.3% after email 5, with nothing gained past stage 5. So email 1 accounts for only about 40% of eventual replies. [P] ([Gem](https://www.gem.com/blog/benchmarks-and-best-practices-for-recruiting-email))
- Ashby (2022–2024): one-email campaigns got 7% replies, three-email campaigns 23%, and more than three plateaued around 23%. Interested responses are highest on the first email (46% of replies there are interested, ~27–28% on later emails). [P] ([Ashby](https://www.ashbyhq.com/talent-trends-report/reports/candidate-sourcing))
- **Cadence:** 3 touches total (initial plus 2 follow-ups), with a fourth only for high-priority people. Space them **4–7 days apart**. Gem found 6-day spacing gave the best interested rate, partly because it lands on a different weekday. [P] Each follow-up should **add something**, such as a new detail, a link, or a different angle. "Bumping this" alone adds nothing.
- **Stop** after the last touch, after any "no," or after an unsubscribe. Log it, and don't re-sequence the same person for 6+ months unless something material changes (a new role, or they changed jobs). [P]
- **Switch channels on the last touch** (email to LinkedIn, or recruiter to HM) rather than sending a fourth email on the same channel. [P]

---

## 5. Candidate experience and pipeline hygiene

- **Ghosting is the norm, and that is the bar to clear.** 61% of job seekers say they've been ghosted after an interview: 66% of historically underrepresented candidates vs 59% of white candidates (Greenhouse, n=2,500, US/UK/DE, Dec 2024). 42% want better recruiter communication. [P] ([Greenhouse](https://www.greenhouse.com/blog/greenhouse-2024-state-of-job-hunting-report))
- **SLAs to set yourself** [P; house rules, not industry standards]:
  - Reply to a sourced candidate's response within 1 business day.
  - Answer inbound applications within 5 business days (yes, no, or "still reviewing").
  - Give a post-interview decision or a status update within 2–3 business days of the final interview. Never leave silence longer than a week.
  - Offer stage: Ashby's average is about 3 days in stage. Accepted offers close in ~2–3 days and rejected ones take ~6. [P] ([Ashby](https://www.ashbyhq.com/talent-trends-report/reports/2023-trends-report-offer-acceptance-rates)) A long offer stage is a warning sign.
- **Close every loop.** Every candidate who talked to a human gets a human answer. A templated rejection is fine for unscreened inbound.
- **Rejection feedback:** offer it on request, keep it to the rubric, make it specific and behavioral ("the system design round didn't get to failure modes"), and never comparative ("we found someone better"). Some companies restrict post-interview feedback on legal advice, so check your company's policy before promising it. [P]
- **Silver medalists:** strong finalists who didn't get this offer are your cheapest next hire. Tag them in the ATS with the reason (level, timing, a single gap), ask permission to reach out again, and actually do it when the next role opens. [P]
- **Keep warm without spamming:** one personal note every 3–6 months with something real (a launch, a new role, a talk). No newsletter blasts.

---

## 6. Pipeline metrics worth tracking

| Metric | Why | Watch for |
|---|---|---|
| Funnel counts by stage (contacted → replied → screened → onsite → offer → accept) | Shows where candidates are lost | Report **counts** ("3 of 11 onsites → offer"), not "27.3%" |
| Same funnel, split by source | Shows which channel actually produces hires for *this* role | With fewer than ~20 per cell the split is anecdote |
| Time to first contact (application or reply → human response) | Strongest candidate-experience lever you fully control | Measure the median, and also the worst case |
| Time in stage | Finds the bottleneck (usually scheduling or debrief) | One stalled candidate skews a small-n mean |
| Offer acceptance | Tells you whether comp, closing or sell is off | Ashby 2021–24 average 78%; technical roles 73%, business 84%. [P] ([Ashby](https://www.ashbyhq.com/talent-trends-report/reports/2023-trends-report-offer-acceptance-rates)) Small denominators: 2 declines out of 5 offers is not a 60% trend |
| Reasons for declines and withdrawals | Usually more actionable than any rate | Ask every time and write the answer down |

- **Small-n honesty.** A single hiring manager makes a handful of hires a year. A 95% confidence interval on 3 accepts out of 4 offers runs from roughly 20% to 99%. Show counts and the underlying list. Compare to a vendor benchmark only when n is in the dozens, and even then as a sanity check, not a grade.
- **Vanity metrics to skip:** open rates (inflated by privacy proxies), messages sent, total applicants, "pipeline size," and LinkedIn profile views. Volume metrics reward spam.
- **Quality of hire** is the metric that matters, and it can't be read at 90 days. At minimum, note at 6 and 12 months whether each hire met the bar the loop predicted. That closes the loop on rubric calibration.

---

## 7. Privacy: candidate data is personal data

- **California (CCPA as amended by CPRA):** since **Jan 1, 2023**, job applicant data is no longer exempt. Applicants have rights to notice at collection, access, deletion and correction, the same as consumers. [D] ([Katten](https://katten.com/california-consumer-privacy-acts-employee-and-b2b-exemptions-to-expire-on-january-1-2023), [CA Lawyers Ass'n](https://calawyers.org/privacy-law/hr-employee-data-b2b-data-to-come-within-scope-of-ccpa-on-january-1-2023/)) It applies to businesses that meet a threshold, such as annual gross revenue over **$26,625,000** (inflation-adjusted figure for 2025–26). [P] ([Clym](https://www.clym.io/blog/ccpa-applicability-guide)) Confirm the current figure with counsel or the CPPA.
- **EU / EEA candidates (GDPR):** sourcing someone means you obtained their data from somewhere other than them. **Article 14** requires telling them who you are, the purpose, the legal basis, and their rights, and doing it at the latest within one month, or at first contact if that's sooner. [D] ([GDPR Art. 14](https://gdpr-info.eu/art-14-gdpr/)) In practice: include a privacy-notice link and an opt-out in the first message to an EU-based person. UK GDPR mirrors this. [D]
- **Minimize:**
  - Store what you need to evaluate the candidate for the role: resume, links, notes against the rubric, and status.
  - **Never record protected or sensitive characteristics** in sourcing notes or the ATS, inferred or stated: age, race, religion, health, pregnancy, family plans, immigration status beyond what the role requires.
  - **Retention:** set it, and delete or anonymize when it expires. Federal recordkeeping rules require employers to keep applicant records for at least 1 year (EEOC, 29 CFR 1602.14) [D] ([eCFR](https://www.ecfr.gov/current/title-29/subtitle-B/chapter-XIV/part-1602/subpart-C/section-1602.14)), and some states require longer. Retention beyond that needs consent or a documented purpose, which a silver-medalist bench is.
- **Keep candidate data in the ATS.** Don't keep it in personal spreadsheets, personal email, or chat threads, and don't paste candidate PII into tools your company hasn't approved.
- **Respect "no."** Record opt-outs and honor them across every channel and every teammate.

---

## 8. Templates

Short by design. Replace each `{placeholder}` with something true and specific, or cut the line. If a template still reads as generic after you fill it in, don't send it.

**Initial outreach: HM-sent**
```
Subject: {their work} → {problem we have}

Hi {name},

I read {specific: your post on X / your PR to Y / your talk at Z}. {One sentence on what was interesting about it.}

I lead {team} at {company}. We're working on {concrete problem}, and I'm hiring a {role} to {own what}. Range is {$low–$high} base plus {equity/bonus}.

Would you be open to 20 minutes with me to hear more? If the timing's wrong, just say so and I won't follow up.

{HM name}
```

**Initial outreach: recruiter-sent**
```
Subject: {role} on {HM name}'s team, {their work} caught our eye

Hi {name},

I recruit for {team} at {company}. {HM name}, who leads the team, flagged your {specific work}, especially {detail}.

The role: {one line on the problem and scope}. Range {$low–$high} base plus {equity}. {Remote/location}.

Open to a 20-minute intro call with me, and then {HM name} if it's a fit? No prep needed.

{Recruiter name}
```

**Follow-up 1 (day 4–7, same thread, adds something)**
```
Hi {name}, one more detail in case it's useful: {new thing: a recent launch, a blog post on the system they'd own, who else is on the team}. Still happy to set up 20 minutes if you're curious.
```

**Follow-up 2 / breakup (day 10–14, switch channel or sender if possible)**
```
Hi {name}, I'll stop here so I'm not cluttering your inbox. If {problem} ever sounds interesting, or the timing changes, my door's open. Either way, thanks for {their work}. I learned something from it.
```

**Reply to "not looking right now"**
```
Totally fair, thanks for letting me know. Mind if I check back in {3–6 months}? And if there's a kind of problem that *would* get you to look, I'd love to hear it, so I only reach out when it's actually relevant.
```
Log the date, their stated trigger (scope, comp, remote, problem), and set a reminder. Then do what you said.

**Referral ask to your own network**
```
I'm hiring a {role} on my team at {company}: {one line on the problem}, range {$low–$high}. The person I'm looking for has {1–2 must-haves, concretely}.

Who's the best {specific behavior, e.g. "person you've seen debug a production outage"} you've worked with? Not "anyone looking," just who's great. Happy to reach out cold and leave your name out if you prefer.
```

**Rejection after interviews (send within 2–3 business days; a call for finalists)**
```
Hi {name},

Thank you for the time you put into {the loop / the onsite}. We've decided not to move forward for this role.

{Optional, only if true: One thing the team genuinely appreciated was {specific}.}

If you'd like feedback, I'm happy to share what I can. Just reply. {If a silver medalist: I'd like to reach out if a role opens that fits better. Is that OK?}

Thanks again,
{name}
```
