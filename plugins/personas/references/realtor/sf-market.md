# San Francisco Market Snapshot

Reference for the `realtor` persona. **As of 2026-09-27 — a snapshot, and markets move monthly.** Use it for shape (which segments are hot, how SF prices, what's seasonal), and refresh any number before quoting it: search the latest monthly SFAR-MLS broker report and Freddie Mac's PMMS, and say the date of whatever you cite.

Confidence labels on every number:
- [F] = read on a fetched page (WebFetch returned the page text; the extraction was done by a summarizer model, so exact digits are high-confidence but not hand-checked)
- [S] = search-engine snippet only; the page itself was not fetched. Treat as lower confidence.

Blocked sources: Redfin (city and county pages returned HTTP 403), Zillow (403), The Real Deal (403). The Compass Bay Area reports site (bayareamarketreports.com) did not resolve (DNS failure). NPR timed out. So no Redfin/Zillow numbers were verified. The SF numbers below come from SFAR MLS-based broker reports (Vanguard, Compass press release, local brokers) and C.A.R.

## 1. Citywide snapshot, August 2026 (latest closed month)

Reports disagree slightly because each draws a different cut of the MLS (closings versus a trailing window, and whether TICs count as condos). Treat each figure as roughly ±3%.

| Metric | Single-family homes | Condo/TIC/co-op | Source |
|---|---|---|---|
| Median sale price | $1,855,000 (+23.7% YoY) | $1,225,000 (+20.1% YoY) | [F] [Vanguard / McHale, Sep 2026 report, SFAR MLS data](https://www.markdmchale.com/blog/san-francisco-real-estate-market-report-september-2026) |
| $/sqft | $1,108 (+18.6% YoY) | $1,129 (+18.1% YoY) | [F] same |
| Median days on market | 13 | 14 (down 34 days YoY) | [F] same |
| Avg sale-to-list | 124.1% (+9.4 pts YoY) | 106.3% (+6.2 pts) | [F] same |
| % sold over asking | 86.4% (69.9% a year earlier) | 53.5% (28.6% a year earlier) | [F] same |
| Closed sales | 154 | 215 (+16.2% YoY) | [F] same |
| Month-end inventory | 131 (−28.4% YoY) | 415 (−22.4% YoY) | [F] same |
| Months of supply | 1.0 | 1.9 | [F] [Prism Group, Sep 2026](https://www.prism-re.com/blog/september-2026-san-francisco-market-report) |
| DOM (Prism's measure, likely average) | 21 | 44 | [F] Prism |
| Condo median (Compass) | n/a | $1.23M (+22% YoY), 214 sales (+11%), active listings −36% | [F] [Compass press release, 2026-09-09](https://www.compass.com/newsroom/press-releases/1WFq1F1inC6ezxL2Zm8Jq9/) |

C.A.R. (all property types blended, so it falls between the two segments): SF county median $1,875,000 in Aug 2026, −8.5% MoM, +25.0% YoY (Aug 2025: $1,500,000); sales −3.2% YoY; SF active listings −53.6% YoY, the steepest drop in the state. [F] [C.A.R. August 2026 release](https://www.car.org/aboutus/mediacenter/newsreleases/2026releases/august2026sales)

Prism, Aug 2026: citywide active listings fell 35% YoY to about 570 even though new listings rose 10%. SFH absorption was 108%, meaning more contracts than active listings. [F] Prism

### July 2026 (the prior month, a higher-priced print)
- SFH median $2.05M (+25% YoY vs $1,641,750); condo median $1.25M (+13.6% YoY per Helena, +8.9% per Legacy). The two reports use different condo cuts. [F] [Helena 7x7, Aug 2026 report](https://www.helena7x7.com/august-2026-san-francisco-market-report/); [F] [Legacy SF, Aug 2026](https://legacysfhomes.com/blog/san-francisco-housing-market-update-august-2026)
- SFH $/sqft $1,181; SFH inventory 156 (−41% YoY); condo inventory 417 (−42.5%). [F] Helena
- SFH months of supply 0.8, condo 1.7; SFH sold about 26% over original ask. [F] Legacy
- Example: a Pacific Heights house listed at $4M sold for $6.6M after 15 offers. [F] Helena

### Trend 2024 → 2026
- 2024: SFH median about +4% YoY; condos roughly flat to down (−8% YoY at mid-2024). [S] [Compass trend page via search](https://www.bayareamarketreports.com/trend/san-francisco-home-prices-market-trends-news) (site unreachable when fetched)
- 2025: full-year SFH median about $1.7M, condo about $1.15M. District 7 house median $6M, +20% vs 2024. The condo turn began around April 2025. [S] same Compass page, plus [SF Chronicle](https://www.sfchronicle.com/realestate/article/pacific-heights-homes-21280454.php)
- 2026: the citywide house median crossed $2M in March and reached a record $2.15M in Q1 (per Compass/Carlisle, +18% YoY), with condos at $1.36M (+27%) [S] ([Insurance Journal/Bloomberg, 2026-04-07](https://www.insurancejournal.com/news/west/2026/04/07/864649.htm), [The Real Deal, 2026-04-08](https://therealdeal.com/san-francisco/2026/04/08/san-franciscos-ai-boom-pushes-home-prices-past-2-million/)). The house median peaked around $2.16M in May, and August fell back to about $1.86M. Brokers attribute the August drop to product mix, not weaker demand: over-asking share and sale-to-list both rose. [F] McHale ("The median moved down. Competition moved up.")
- Direction: still sharply up YoY in both segments. The monthly median has come off the spring peak, as it does every year. Inventory is at multi-year lows.

### The AI-boom effect
- Compass's Patrick Carlisle credits AI startup hiring and wealth creation (OpenAI, Anthropic and others expanding offices with in-office requirements) for a market that stays "extremely heated" despite higher rates. He also cites economic volatility, including an Iran war. [S] [Real Deal / Insurance Journal, Apr 2026](https://www.insurancejournal.com/news/west/2026/04/07/864649.htm)
- NPR, 2026-09-01: "AI wealth is creating a 'mansion shortage'". [S] [NPR](https://www.npr.org/2026/09/01/nx-s1-5930432/ai-san-francisco-housing-market-real-estate) (fetch timed out; headline only)
- Helena: demand is driven by "wealth creation rather than financing costs", with AI hiring fueling relocation buyers. [F] Helena
- Countervailing factors: Prism notes five consecutive months of Bay Area job losses, contracting national tech employment, and 30-year rates near 6.8% (now above 7%, see §4). SF net in-migration turned positive for the first time in years. [F] Prism
- Framing: equity- and liquidity-rich buyers (IPO, tender offers, RSUs) are less rate-sensitive, which is why SF runs hot while the state is flat (C.A.R. statewide median +0.1% YoY).

## 2. District breakdown (SF MLS districts 1–10)

District to neighborhood mapping, with sub-district codes: [F] [HighNote360 SF MLS district guide](https://www.highnote360.com/san-francisco-ca-homes-for-sale/san-francisco-neighborhood-map/)

House figures: [F] [Team K.Ho / Vanguard district survey](https://team-kho.com/san-francisco-housing-market-update-2026/). Window is 2025-07-15 to 2026-07-16, 4,898 total closings, SF MLS data. House figures were confirmed on a verbatim re-read. Condo medians come from a summarized pass of the same page, so they are lower confidence. Note that DOM here is days to close, not days to contract.

| District | Neighborhoods | House median | Houses sold | % houses over list | Days | Condo median | Read |
|---|---|---|---|---|---|---|---|
| 1 Northwest | Richmond (Inner/Central/Outer), Jordan Park/Laurel Hts, Lake St, Sea Cliff, Lone Mountain | $2.49M | 185 | 88% | 43 | ~$1.50M | Very hot; median about $522K over list |
| 2 Central West | Sunset (Inner/Central/Outer), Parkside, Golden Gate Hts | $1.80M | 395 | 89% | 43 | ~$1.18M | Hottest; sale-to-list about 128%. The classic "list low, bid up" district |
| 3 Southwest | Lake Merced, Stonestown, Merced Manor/Hts, Ingleside, Oceanview, Pine Lake Park | $1.40M | 142 | 86% | 40 | ~$692K | Hot, entry-level houses |
| 4 Twin Peaks West | West Portal, St. Francis Wood, Forest Hill, Miraloma, Sunnyside, Westwood Park, Diamond Hts | $2.25M | 300 | 86% | 48 | ~$754K | Hot family market |
| 5 Central | Noe Valley, Castro/Eureka Valley, Glen Park, Haight, Cole Valley, Duboce Triangle, Mission Dolores | $2.80M | 290 | 82% | 50 | ~$1.50M | Hot at a high price point. Noe (5C): 110 houses, $2.88M median, median about $605K over list [S] |
| 6 Central North | Hayes Valley, Lower Pac Hts, NoPa, Alamo Sq, Western Addition, Anza Vista | $3.44M | 27 | 67% | 77 | ~$1.25M | Thin house volume; mostly a condo/TIC district |
| 7 North | Pacific Hts, Presidio Hts, Marina, Cow Hollow | $6.35M | 99 | 54% | 63 | ~$1.64M | Luxury. Priced more by negotiation, but setting records (2512 Union closed at $15M). Pacific Heights 7B median ~$6.85M [S] |
| 8 Northeast | Russian Hill, Nob Hill, North Beach, Telegraph Hill, FiDi, Downtown, Van Ness/Civic Center, Tenderloin | $4.24M | 30 | 43% | 72 | ~$1.01M | Condo-dominated; buyers have the most leverage |
| 9 Central East | SoMa, South Beach, Mission Bay, Yerba Buena, Mission, Potrero Hill, Bernal Hts, Dogpatch | $1.80M | 236 | 85% | 61 | ~$1.02M | Two markets: Bernal/Potrero/Mission houses are hot; SoMa/South Beach towers are soft |
| 10 Southeast | Bayview, Excelsior, Portola, Visitacion Valley, Outer Mission, Crocker-Amazon, Hunters Pt | $1.12M | 452 | 81% | 54 | ~$709K | Highest volume; where first-time buyers concentrate |

Citywide over the same window: house median $1.85M, median $360.5K over list, 83% of houses sold over list. Condo median about $1.185M with sale-to-list about 100%. TIC median about $1.09M across 312 closings. [F] Team K.Ho

### Hot vs soft
- Hottest: mid-priced houses in D2/D1/D3/D4/D10, which run 81–89% over list. Sunset/Parkside is the purest bidding-war market. Noe/Castro houses are hot at $2.5–3M+.
- Softest: downtown and high-rise condos in D8/D9 (about 31–38% over list [F, summarized], longer to close) and D10 condos (about 106 days to close [F, summarized]). Mid-tier condos have seen slight price dips and rising HOA dues [S] ([rebeccarealtor.com](https://www.rebeccarealtor.com/blog/san-francisco-condo-market-2026/)). SoMa median about $834K per Redfin, via snippet only [S] ([Redfin SoMa](https://www.redfin.com/neighborhood/11212/CA/San-Francisco/SoMa/housing-market)).
- Nuance: the condo segment is recovering hard overall (+20–22% YoY in Aug; SF was "the only county where condos showed both meaningful price and closed transaction growth" [F] Compass). The $3M+ full-service luxury condo tier is reportedly strong [S]. Softness is concentrated in ordinary 1–2BR tower units with high HOA dues, not in all condos.
- D7 ultra-luxury: prices are at records, but most homes sell at or near list after negotiation rather than through bidding wars.

### $/sqft by neighborhood (Redfin)

The fairer way to compare neighborhoods, since price swings with size. Single-family homes, closed sales June 2025–May 2026: four non-overlapping 90-day windows from Redfin's neighborhood tracker, weighted by sales, with fewer than 8 house sales left out. Condo $/sqft is shown where there were 8+ condo sales. [F] [Redfin Data Center](https://www.redfin.com/news/data-center/), `neighborhood_market_tracker.tsv000.gz`, file dated 2026-06-02.

How to read it:
- The data ends in May 2026 and the market rose after; scale up by the citywide YoY change before pricing a home today.
- Each figure is a median across all conditions and sizes. Renovated homes sell well above it, and $/sqft falls as size rises, so a big house prices below its neighborhood's figure per foot.
- $/sqft is rounded to the nearest $10, and medians to $0.1M; a neighborhood with ~10 sales is a rough guide, not a price.
- Redfin's neighborhood boundaries don't match MLS sub-districts.

| Neighborhood | House $/sqft | House median | Sales | Sale/list | Condo $/sqft (sales) |
|---|---|---|---|---|---|
| Sea Cliff | $1,650 | $5.4M | 13 | 112% | — |
| Cow Hollow | $1,640 | $7.0M | 21 | 107% | $1,420 (22) |
| Presidio Heights | $1,610 | $6.4M | 28 | 110% | $1,300 (28) |
| Pacific Heights | $1,570 | $6.4M | 66 | 103% | $1,210 (172) |
| Ashbury Heights | $1,530 | $4.4M | 14 | 116% | $1,250 (19) |
| Presidio Terrace | $1,520 | $5.9M | 8 | 102% | — |
| Marina Gardens | $1,510 | $3.5M | 19 | 108% | $1,460 (17) |
| Eureka Valley | $1,500 | $2.9M | 58 | 118% | $1,210 (22) |
| Telegraph Hill | $1,500 | $4.1M | 13 | 99% | $1,020 (39) |
| Ashbury Heights / Parnassus | $1,500 | $4.1M | 26 | 114% | $1,240 (26) |
| Marina District | $1,490 | $3.7M | 40 | 105% | $1,360 (59) |
| Noe Valley | $1,480 | $2.6M | 150 | 117% | $1,140 (66) |
| Lake Street | $1,450 | $5.3M | 12 | 117% | — |
| Laurel Heights-Jordan Park | $1,430 | $3.1M | 17 | 116% | $1,270 (11) |
| Dolores Heights | $1,400 | $2.7M | 37 | 118% | $1,270 (33) |
| Corona Heights | $1,360 | $2.5M | 11 | 111% | $1,300 (18) |
| St. Francis Wood | $1,320 | $3.7M | 27 | 109% | — |
| Buena Vista | $1,310 | $3.2M | 16 | 111% | $1,140 (26) |
| Parnassus Heights | $1,300 | $2.8M | 10 | 107% | — |
| Russian Hill | $1,300 | $2.0M | 43 | 102% | $1,240 (91) |
| Fairmount | $1,290 | $2.5M | 20 | 114% | $1,060 (11) |
| Inner Sunset | $1,280 | $2.5M | 12 | 122% | $1,010 (13) |
| Forest Hill | $1,250 | $3.1M | 13 | 116% | — |
| Clarendon Heights | $1,210 | $2.7M | 22 | 113% | — |
| Lone Mountain | $1,190 | $2.7M | 15 | 110% | $1,070 (14) |
| Glen Park | $1,190 | $2.1M | 63 | 118% | $1,100 (10) |
| Potrero Hill | $1,180 | $1.8M | 50 | 118% | $960 (80) |
| Duboce Triangle | $1,160 | $1.8M | 15 | 105% | $1,080 (20) |
| Peralta Heights | $1,160 | $2.0M | 22 | 117% | — |
| Midtown Terrace | $1,160 | $1.8M | 26 | 122% | — |
| Upper Market | $1,150 | $1.8M | 31 | 111% | $980 (19) |
| Lower Pacific Heights | $1,130 | $2.6M | 11 | 111% | $1,130 (37) |
| West Portal | $1,120 | $2.6M | 18 | 116% | — |
| Bernal Heights | $1,120 | $1.6M | 181 | 118% | $1,050 (19) |
| Outer Sunset | $1,110 | $1.7M | 255 | 125% | $830 (8) |
| Miraloma Park | $1,110 | $1.9M | 41 | 119% | — |
| Golden Gate Heights | $1,110 | $2.1M | 79 | 123% | $610 (8) |
| Inner Richmond | $1,100 | $2.4M | 59 | 117% | $1,110 (34) |
| Holly Park | $1,100 | $1.6M | 22 | 115% | — |
| Sutro Heights | $1,090 | $2.2M | 20 | 123% | — |
| Laguna Honda | $1,080 | $2.4M | 29 | 114% | — |
| Sunnyside | $1,080 | $1.5M | 65 | 121% | — |
| West Portal Park | $1,070 | $3.0M | 8 | 120% | — |
| Westwood Park | $1,060 | $2.1M | 16 | 111% | — |
| Balboa Terrace | $1,060 | $2.6M | 12 | 126% | — |
| Lakeshore Acres | $1,050 | $1.9M | 18 | 121% | — |
| Diamond Heights | $1,050 | $2.5M | 23 | 118% | $740 (32) |
| Monterey Heights | $1,050 | $2.9M | 10 | 109% | — |
| Parkside | $1,040 | $1.8M | 87 | 126% | — |
| Sherwood Forest | $1,030 | $2.2M | 18 | 116% | — |
| Forest Knolls | $1,020 | $1.9M | 15 | 114% | — |
| Lower Haight | $1,020 | $1.7M | 15 | 112% | $1,130 (35) |
| Alamo Square | $1,020 | $1.8M | 10 | 115% | $1,010 (18) |
| Outer Richmond | $1,010 | $2.0M | 76 | 118% | $820 (25) |
| Panhandle | $990 | $1.7M | 10 | 109% | $1,150 (41) |
| Nob Hill | $980 | $1.1M | 33 | 102% | $1,020 (69) |
| Mount Davidson Manor | $980 | $2.3M | 16 | 110% | — |
| Mission Dolores | $980 | $1.3M | 19 | 112% | $1,110 (57) |
| St. Marys Park | $970 | $1.8M | 15 | 119% | — |
| Ingleside Terraces | $960 | $2.4M | 21 | 108% | — |
| Balboa Hollow | $950 | $2.6M | 31 | 117% | $1,110 (17) |
| Mission District | $950 | $1.4M | 60 | 112% | $890 (96) |
| Cayuga | $920 | $1.4M | 25 | 118% | — |
| Mission Terrace | $880 | $1.2M | 46 | 117% | — |
| Ingleside | $860 | $1.3M | 24 | 115% | — |
| Portola | $840 | $1.1M | 53 | 115% | — |
| Excelsior | $830 | $1.2M | 96 | 117% | — |
| University Mound | $790 | $1.3M | 28 | 115% | — |
| Crocker Amazon | $780 | $1.2M | 48 | 114% | — |
| Visitacion Valley | $740 | $1.0M | 33 | 111% | — |
| Oceanview | $740 | $1.3M | 30 | 110% | — |
| Outer Mission | $710 | $1.1M | 21 | 107% | — |
| Silver Terrace | $670 | $1.0M | 40 | 110% | — |
| Bret Harte | $660 | $0.9M | 19 | 106% | — |
| Bayview | $660 | $0.9M | 27 | 105% | — |
| Hunters Point | $630 | $1.0M | 9 | 104% | $600 (18) |

## 3. Seasonality
- SF has twin peaks: spring (April–May) and fall (September–October). The quietest periods are August and the winter holidays. Fall listings arrive in a burst on the weekends after Labor Day, and most fall listings are on the market by early November. [S] [Team K.Ho seasonality doc](https://www.kevinandjonathan.com/docs/seasonality/) (URL now redirects to team-kho.com, not fetched)
- December is consistently the lowest month for new listings, about 4–5% of the annual total (Dec 2023: 137 new listings vs 937 in September). [S] [vivrerealestate.com](https://vivrerealestate.com/blog/busiest-times-of-the-year-for-real-estate-in-san-francisco-a-data-driven-look)
- Pattern in recent years: October closings are strong (Oct 2025 had the most sales since spring 2022, about +28% YoY). November drops around Thanksgiving, December is the quietest month, and activity resumes in mid-January. [S] [Helena Dec 2025](https://www.helena7x7.com/san-francisco-real-estate-market-update-december-2025/)
- What this means for Oct–Dec 2026: late September to October is the second selling window of the year. Expect a burst of new listings, then thin choice from Thanksgiving through New Year. Sellers who miss early November usually wait until January or February. Buyers find the least competition in December, but also the fewest homes.

## 4. Financing
- Freddie Mac PMMS, 2026-09-24: 30-yr fixed 7.03% (6.95% on 9/17, 6.76% on 9/10, 6.71% on 9/3; 6.30% a year ago). 15-yr fixed 6.42% (5.49% a year ago). Rates are rising through September. [F] [Freddie Mac release via GlobeNewswire](https://www.globenewswire.com/news-release/2026/09/24/3368592/0/en/mortgage-rates-average-7-03.html); earlier weeks [S] [freddiemac.com/pmms](https://www.freddiemac.com/pmms)
- Jumbo: national average 30-yr jumbo APR about 7.34% on 2026-09-27. [S] [Bankrate](https://www.bankrate.com/mortgages/jumbo-loan-rates/) (snippet)
- 2026 conforming limits for SF County (FHFA high-cost ceiling): 1-unit $1,249,125; 2-unit $1,599,375; 3-unit $1,933,200; 4-unit $2,402,625. [F] [Sammamish Mortgage](https://www.sammamishmortgage.com/san-francisco-conforming-loan-limits/). National baseline 1-unit: $832,750 [S] ([JVM Lending](https://www.jvmlending.com/blog/california-conforming-loan-limits/)). Ignore snippets quoting $1,149,825, which is an older year's ceiling.
- Down payment math: at the August SFH median of $1.855M, 20% down is about $371K and leaves a $1.484M loan, which is jumbo. Keeping the loan at the conforming limit would take about $606K down (about 33%). A median condo ($1.225M) with 20% down ($245K) needs a $980K loan, which is conforming. Jumbo programs exist at 10% down, and some at 5%, but the best pricing needs 20% or more. [S] ([JVM Lending jumbo guide](https://www.jvmlending.com/blog/jumbo-loan-rates-california/), [Bay Area jumbo 2026](https://mycahomeloan.com/bay-area-jumbo-mortgage-rates-2026/))
- Competitive reality: winning offers commonly waive contingencies and go 20%+ over list for houses (sale-to-list about 124%). Appraisal gaps are covered in cash, so the real cash need is well above the nominal down payment. This is inferred from the sale-to-list data above; no source quantifies it.

## 5. Bay Area comparison (one line each)

| Area | Median | Competitiveness | Source |
|---|---|---|---|
| Bay Area region (C.A.R.) | $1,272,000 Aug 2026, −0.2% YoY; sales −4.2% YoY | Flat overall; SF and San Mateo are carrying it | [F] C.A.R. |
| San Mateo County | $2,250,000 Aug 2026, +13.2% YoY | Hot, second only to SF on price growth | [F] C.A.R. |
| Burlingame | ~$2.775M median, ~flat YoY, ~106.6% sale-to-list | Competitive, with a softer price trend than the county | [S] [Houzeo](https://www.houzeo.com/housing-market/california/burlingame) |
| Palo Alto | $4.1M median Jan–Aug 2026 (+7.8% vs $3.8M); 270 sales; new listings −9.2% | About one third of homes sold $500K+ over ask | [F] [Palo Alto Online, 2026-09-21](https://www.paloaltoonline.com/neighborhoods/2026/09/21/palo-alto-neighborhoods-mapped-see-where-median-prices-jumped-inventory-dipped-and-sales-surged-in-2026/) (MLSListings) |
| Santa Clara County | $1,900,000 Aug 2026, 0.0% YoY; sales −13% | Cooling | [F] C.A.R. |
| Alameda County | $1,285,000 Aug 2026, +1.3% YoY; sales −8.8% | Steady, not heating | [F] C.A.R. |
| Oakland | ~$809K (3 months to Aug 2026, +6.5% YoY), sale-to-list ~109% | Moderately competitive | [S] [Redfin Oakland](https://www.redfin.com/city/13654/CA/Oakland/housing-market) |
| Berkeley | ~$1.55M median, ~flat YoY; ~88.7% sold over asking, sale-to-list ~124–127% | Very competitive, a bidding-war town | [S] [Houzeo Berkeley](https://www.houzeo.com/housing-market/california/berkeley), [Loqol](https://loqol.ai/blog/berkeley-housing-market-2026) |
| Piedmont | Redfin median $2.45M (Feb 2026, 9 sales, 11 DOM); Zillow typical value ~$2.53M (+12.1% YoY) | Very competitive, tiny volume | [S] [Zillow Piedmont](https://www.zillow.com/home-values/6495/piedmont-ca/) |
| Alameda (city) | Unknown. No reliable 2026 figure found | Unknown | none |
| Contra Costa County | $875,000 Aug 2026, +2.9% YoY | Steady | [F] C.A.R. |
| Marin County | $1,650,500 Aug 2026, +8.4% YoY (−6.8% MoM); sales +13.6%; active listings −28.6% YoY | Firming; DOM around 36–39, slower than SF | [F] C.A.R.; DOM [S] [Redfin Marin](https://www.redfin.com/county/323/CA/Marin-County/housing-market) |

Statewide for context: C.A.R. median $901,420 Aug 2026 (+0.1% YoY), unsold inventory index 3.7 months, median DOM 28, sale-to-list 98.9%. [F] C.A.R.

## 6. Gaps and caveats
- Redfin and Zillow listing pages block fetches. Their published data files don't: the $/sqft table above comes from Redfin's. There's no Redfin Compete Score here.
- District condo medians are summarizer output, not verbatim.
- Median prices swing month to month with product mix (August's drop is the example). Anchor on YoY and on sale-to-list and over-asking share rather than on any single monthly median.
- The macro backdrop has two conflicting stories: rising rates (7%+), Bay Area job losses and the war-driven volatility sit against AI wealth concentration. Present both rather than a single narrative.
