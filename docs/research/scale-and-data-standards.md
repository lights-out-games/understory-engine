# Research report: distributed simulation + converged data standards (compiled 2026-10-01)

Method and caveats
- Every bullet with a URL was opened this session unless tagged **[NV]** = not verified (page not opened, fetch failed, or the page did not state the fact; content is background knowledge and should be re-checked).
- Pages were read through a summarising fetcher, so version numbers and dates are as that tool reported them. A few looked odd and are flagged "date suspect".
- No recommendations given. Trade-offs are those stated in sources. Gaps are collected in section 9.
- Nothing was written to disk or to the project; nothing to save to memory.

---

## A1. Projects at scale

### Agent-based frameworks (HPC / distributed)
| Project | Language | Licence | Architecture | Scale / status | Source |
|---|---|---|---|---|---|
| Repast4Py | Python + C++ (MPI, Numba, NumPy, PyTorch) | BSD-3 | Distributed ABM across MPI ranks; successor line to Repast HPC | v1.2.1, 12 Nov 2025; "Beta"; Python 3.9 to <3.14; needs `mpicxx` at install | https://pypi.org/project/repast4py/ |
| Repast HPC | C++ (Boost, NetCDF, MPI) | Licence file present, type not shown | MPI | 273 commits; README still cites Boost 1.61 (looks low-activity) | https://github.com/Repast/repast.hpc |
| FLAME GPU 2 | CUDA C++ / Python | AGPL-3.0-only, commercial licence available | GPU ABM; MPI used for distributed *ensembles* (1 MPI worker per node), not one partitioned model | Still "pre-release (release candidate)"; needs CUDA 12+ | https://github.com/FLAMEGPU/FLAMEGPU2 |
| krABMaga | Rust (99.6%) | MIT | Bevy-based visualisation, WASM visualisation in browser, MPI feature for distributed model exploration, experimental parallel scheduling | v0.6.2 (19 Jun 2026), active | https://github.com/krABMaga/krABMaga |
| krABMaga distributed (paper) | Rust + MPI | n/a | K-D tree field partitioning, halo regions, MPI scatter/gather plus non-blocking send/receive | 1M to 10M agents, up to 128 Azure cores; 93.05x speedup at 128 procs (5,206.9 s to 55.9 s). Stated limits: no load balancing; not benchmarked against other engines. Sept 2024 | https://ceur-ws.org/Vol-3785/paper124.pdf |
| Agents.jl | Julia | "Free and open source" (licence type not on page; MIT is [NV]) | Docs list "distributed computing" but do not say a single model can be partitioned across nodes | v7.0.3 (23 Jun 2026) | https://juliadynamics.github.io/Agents.jl/stable/ |
| BioDynaMo | C++ | Apache-2.0 | Shared-memory 3D ABM | Active (1,791 commits). CERN origin and "billions of agents" claim are [NV] | https://github.com/BioDynaMo/biodynamo |
| TeraAgent | Extends BioDynaMo | [NV] | Distributed engine; serialisation that lets agents be read/mutated directly from receive buffers; delta encoding between iterations | "Half a trillion agents", 84x over BioDynaMo. arXiv 2509.24063 (Sept 2025 by ID). MPI use and core counts not stated in abstract | https://arxiv.org/abs/2509.24063 |
| D-MASON | Java | Apache-2.0 | Distributed MASON | Marked "unsupported"; functionality said to be in main MASON now | https://github.com/isislab-unisa/dmason |
| GAMA | Java (JDK 21) | GPL-3.0 | Spatially explicit ABM, headless mode | Active; no native distributed partitioning shown on page | https://github.com/gama-platform/gama |
| Pandora | C++ / Python | LGPL-3.0 + GPL-3.0 | MPI, GIS support, Cassandra GUI | 675 commits; activity level unclear | https://github.com/xrubio/pandora |
| EMOD | C++ (Python front ends) | MIT | Epidemiological ABM (malaria, HIV as of v2.22) | Repo states projects "no longer actively maintained or supported by IDM or the Gates Foundation" unless noted; moved to EMOD-Hub | https://github.com/InstituteforDiseaseModeling/EMOD |
| ROSS | C | BSD-3 | Optimistic PDES (Time Warp), reverse computation; conservative and optimistic modes | v8.1.1 (22 Aug 2025), active | https://github.com/ROSS-org/ROSS |
| OpenABM (Oxford COVID sim) | n/a | n/a | n/a | [NV], not opened. "OpenABM" also names the CoMSES model library (A3) | n/a |

### Ecosystem, vegetation and crop models
| Model | Language | Licence | Notes | Source |
|---|---|---|---|---|
| Madingley (refactored) | C# | Custom (Licence.txt) | 49 commits, prototype; NetCDF I/O. A C++ version exists [NV] | https://github.com/Madingley/Madingley |
| LPJ-GUESS | [NV: C++] | Research source downloadable; licence not on page | v4.1 (methane, soil N, permafrost, new fire model); Windows-only educational binary | https://web.nateko.lu.se/lpj-guess/ |
| LANDIS-II | [NV: C#] | "completely open-source" (type not stated) | v8 referenced; hundreds to millions of hectares; download needs user-group login | https://www.landis-ii.org/ |
| iLand | [NV: C++/Qt] | GPL | v2.2; individual trees at 10,000+ ha over centuries; TU Munich | https://iland-model.org/ |
| ED2 | Fortran | CC-BY-4.0 (unusual for code) | Last stable ED-2.2 (Feb 2020); nested grid/polygon/site/patch/cohort | https://github.com/EDmodel/ED2 |
| FATES | Fortran | LICENSE.txt (type not shown) | Hosts: E3SM, CESM, CTSM; 47 releases | https://github.com/NGEET/fates |
| APSIM | [NV: C#, source on GitHub] | Not open source: free General Use for public-good R&D/education; commercial fees FY2026-27 AU$3,638 / 12,128 / 30,319 per year by turnover; improvements go back to APSIM Initiative | Licence restricts embedding in a product | https://www.apsim.info/download-apsim/ |
| DSSAT-CSM | Fortran | BSD-3 | v4.8.5.0 (4 Dec 2024); 45+ crops | https://github.com/DSSAT/dssat-csm-os |
| PCSE / WOFOST | Python | LICENSE present (EUPL is [NV]) | WOFOST 7.2/8.0, LINGRA; GitHub release tag shown is old (5.4.1, 2018); PyPI may be newer [NV] | https://github.com/ajwdewit/pcse |
| AquaCrop-OSPy | Python | Apache-2.0 | Mirrors FAO AquaCrop 7.1 | https://github.com/aquacropos/aquacrop |
| BioCro | C++ / R | C++ framework LGPL-3+, R package MIT | v3.3.1 (9 Feb 2026) | https://github.com/biocro/biocro |
| Daisy | C++ | GPL-2.0 + LGPL-2.1 | Field-scale water, N, C, pesticides | https://github.com/daisy-model/daisy |
| DNDC, CropSyst, pySTEPS | n/a | n/a | [NV], not opened. pySTEPS is precipitation nowcasting, not ecology (background) | n/a |

### Digital twins and forecasting infrastructure
- **Destination Earth**: in an "Enhancement phase" targeting 2026; "full" digital replica by 2030. Two operational twins (Climate Change Adaptation, Weather-induced Extremes). ECMWF builds the twins and engine, ESA the platform, EUMETSAT the data lake. https://destination-earth.eu/
- **BioDT**: CSC Finland coordinating, 25+ partners, LUMI supercomputer, LEXIS orchestration. Ten prototype twins: forest dynamics, grassland dynamics, ecosystem services, real-time bird monitoring from citizen-science data, crop wild relatives, DNA metabarcoding prioritisation, phylogenetic diversity, disease outbreaks, honey bees in agricultural landscapes, invasive alien species. Policy brief April 2025; a final event dated 29 Apr 2025 suggests the project has ended. BEEHAVE as the honey-bee model is [NV]. https://biodt.eu/
- **LTER-LIFE** (NL): 10-year NWO grant from Aug 2023; virtual labs toward ecosystem digital twins; Wadden Sea and Veluwe. https://lter-life.nl/en
- **European Digital Twin of the Ocean**: data lake + HPC + user interface; builds on Copernicus, EMODnet, EuroHPC. https://digitaltwinocean.mercator-ocean.eu/
- **NVIDIA Earth-2**: now described as a "family of open models, libraries, and frameworks" (Medium Range/Atlas, Nowcasting, HealDA assimilation, CorrDiff, FourCastNet 3, Earth2Studio). Specific licences not on page. https://www.nvidia.com/en-us/high-performance-computing/earth-2/
- **NEON Ecological Forecasting Challenge (EFI)**: five themes (aquatics, terrestrial, ticks, phenology, beetles); docs dated 10 Apr 2024. Parquet/S3/STAC details [NV]. https://projects.ecoforecast.org/neon4cast-docs/
- **Pangeo**: xarray + Dask + Zarr + Jupyter; NumFOCUS project; NSF/NASA support. https://pangeo.io/

### Game-industry distributed simulation
- **SpatialOS (Improbable)**: 2019 Unity ToS dispute (US$25M fund with Epic); Worlds Adrift, Lazarus, Mavericks cancelled 2019; Scavengers shut 2022; pivot to MSquared metaverse (2022); defence unit divested June 2023; no longer the primary business. https://en.wikipedia.org/wiki/Improbable_(company)
- **Hadean**: now sells defence synthetic environments (populAI, dominAI, optimAI); NATO DIANA, UK MoD. Not a general game platform. https://hadean.com/
- **Unity Netcode for Entities** 1.5.1: server-authoritative with client prediction, DOTS/ECS, Unity 2022.3 or 6. Ghost relevancy/importance detail [NV]. https://docs.unity3d.com/Packages/com.unity.netcode@1.5/manual/index.html
- **bevy_replicon** (MIT/Apache-2.0): server-authoritative replication, per-client visibility control, no built-in I/O; 0.41 to 0.44 on Bevy 0.19. https://github.com/projectharmonia/bevy_replicon
- **lightyear** 0.29 (MIT/Apache-2.0): prediction + rollback, interpolation, interest management, deterministic input-only replication (lockstep or rollback), WASM via WebTransport. https://github.com/cBournhonesque/lightyear
- **Eco**: persistent always-on server; "the server must always be running to be playable"; world size configurable. https://wiki.play.eco/en/Server
- **EVE Online time dilation**: game clock slows under load so time-bound work shrinks; worked example dips to about 5 to 10%; stated limit: load not tied to time is unaffected, so fight size is not unbounded. https://www.eveonline.com/news/view/introducing-time-dilation-tidi
- **Factorio**: deterministic lockstep, every peer simulates the whole world, only inputs sent; mod checksums compared on join; latency hiding for some actions only (not vehicles/shooting). https://wiki.factorio.com/Multiplayer
- Dual Universe, Dwarf Fortress world-gen, general "big world" technique write-ups: [NV], not opened. Halo cells and K-D partitioning are evidenced in the krABMaga paper; visibility/interest management in replicon and lightyear.

## A2. Building blocks and standards

### Coupling and co-simulation
- **HLA 4 (IEEE 1516-2025)**: approved by IEEE 11 Mar 2025; publication expected late Mar/early Apr 2025. Headlines: security, scalability, extensibility, cloud/container readiness. https://www.sisostandards.org/news/695782/HLA-4-Approved-by-IEEE.htm
- **FMI**: 3.0.2 current (2.0.5 maintained); Model Exchange, Co-Simulation, Scheduled Execution; 290+ tools; layered standards include **FMI-LS-WASM** (WebAssembly), XCP, BUS, STRUCT, REF. https://fmi-standard.org/
- **BMI 2.0 (CSDMS)**: control, info, variable, time, getter/setter, grid functions; official C, C++, Fortran, Java, Python; community JavaScript and Julia. No Rust binding listed. https://bmi.csdms.io/en/stable/
- **OpenMI 2.0**: OGC standard 11-014r3, adopted 2014; runtime data exchange between models. No newer version shown. https://www.ogc.org/standards/openmi/
- **ESMF / NUOPC**: component framework, NUOPC common architecture, ESMX executable, regridding. Version/licence not on page. https://earthsystemmodeling.org/
- **OASIS3-MCT 4.0** (2018; Python/C/C++ interfaces 2021): parallel regridding and exchange; 67 groups, 80+ coupled systems (2019 survey). https://oasis.cerfacs.fr/en/
- **preCICE 3.4.1**, LGPLv3; C++, C, Fortran, Python, Matlab. https://precice.org/
- **Landlab** (MIT): grid engine + process components, regular and irregular grids. BMI link not stated on page. https://github.com/landlab/landlab
- **pywatershed 3.0.0** (CC0-1.0): USGS Python re-implementation of PRMS; flagged "preliminary or provisional". https://github.com/EC-USGS/pywatershed
- DIS: [NV].

### Actors, synchronisation, local-first
- **Orleans 10.0**: virtual actors (grains always exist virtually), silos, durable reminders, ACID multi-grain transactions. https://learn.microsoft.com/en-us/dotnet/orleans/overview
- **Apache Pekko**: fork of Akka 2.6.x made before Akka moved to the Business Source License; cluster sharding. https://pekko.apache.org/
- **Dapr actors**: virtual actor pattern, turn-based access, timers vs persistent reminders, placement service. https://docs.dapr.io/developing-applications/building-blocks/actors/actors-overview/
- Ray: [NV].
- **Automerge 3.0** (July 2025): more than 10x memory reduction (Moby Dick example 700 MB to 1.3 MB). https://automerge.org/blog/automerge-3/
- **Loro** 1.23.2 (MIT, Rust + WASM + Swift): Fugue text, movable tree/list, time travel, shallow snapshots. https://github.com/loro-dev/loro
- **Yjs** (MIT); Rust port `yrs`. https://github.com/yjs/yjs
- **iroh** (MIT/Apache-2.0, Rust): dial by public key over QUIC, hole punching with relay fallback; iroh-blobs (BLAKE3 content addressing), iroh-gossip, iroh-docs. Version and browser support not on page. https://github.com/n0-computer/iroh
- **Ditto**: CRDT document database with BLE / P2P Wi-Fi / LAN mesh and optional server. Proprietary status is [NV]. https://docs.ditto.live/home/about-ditto
- ElectricSQL: site now redirects to electric.ax; not opened [NV].
- Event sourcing, IPFS/IPLD/CIDs, conservative PDES literature: [NV].

### Cloud-native geospatial I/O
- **Zarr v3**: core spec 3.1; v3 accepted 15 May 2023 (ZEP0001); sharding codec; extension points. https://zarr-specs.readthedocs.io/en/latest/v3/core/index.html
- **GeoZarr**: OGC SWG, still draft; conventions for multiscales, projection, spatial transforms; CF-in-Zarr and DGGS-in-Zarr in progress; no versioned release. https://github.com/zarr-developers/geozarr-spec
- **Icechunk** v2.1.1 (8 Jul 2026), Apache-2.0, Rust + Python: transactional, snapshot-isolated, branches/tags, virtual references to HDF5/NetCDF/GRIB. https://github.com/earth-mover/icechunk
- **GeoParquet**: site shows v2.0.0-rc.1; "incubating" OGC standard. 2.0 requires native Parquet GEOMETRY/GEOGRAPHY types, drops the GeoArrow encodings and the bbox covering column of 1.1. Release dates returned (1.1.0 = 19 Jun 2023; 2.0 rc = 19 Jul 2024) are suspect. https://geoparquet.org/ and https://github.com/opengeospatial/geoparquet/releases
- **STAC 1.1.0** (11 Sep 2024): unified `bands`, item_assets in core, SPDX licence expressions. https://github.com/radiantearth/stac-spec/releases
- **PMTiles v3**: single-file tile archive read by HTTP range requests; stated limit: read-only, updates need a full rewrite. https://docs.protomaps.com/pmtiles/
- **FlatGeobuf** (BSD-2): FlatBuffers encoding, optional packed Hilbert R-tree, streaming and range requests. https://flatgeobuf.org/
- **CF conventions 1.13**. https://cfconventions.org/
- **OGC API - EDR 1.1** (published 27 Jul 2023): position, area, cube, trajectory, corridor, radius, items, locations, instances. https://docs.ogc.org/is/19-086r6/19-086r6.html
- **CoverageJSON 1.0**: OGC Community Standard (23 Aug 2023); JSON-LD; aimed at browsers, not bulk transfer. https://www.ogc.org/standards/coveragejson/
- **SensorThings API**: Sensing 1.1, Tasking 1.0, plus **STAplus 1.0** and a WebSub extension. No 2.0 listed. https://www.ogc.org/standards/sensorthings/
- **DGGS**: OGC API - DGGS Part 1 Core 1.0 (21-038r1) is an approved standard; Abstract Spec Topic 21 v1.0 (20-040r3). https://www.ogc.org/standards/dggs/
- **H3**: Apache-2.0, C core, Java/JS/Python bindings. https://github.com/uber/h3 . Rust `h3o`: URL tried returned 404 [NV].
- COG, Arrow/Flight, DuckDB spatial (page was a redirect stub), xarray, S2, rHEALPix, OGC API Features/Processes: [NV].

## A3. Reproducibility and model description
- **ODD 2020 (second update)**: seven elements (Purpose and patterns; Entities, state variables, scales; Process overview and scheduling; Design concepts; Initialization; Input data; Submodels). Adds rationale, summary ODD, nested ODD, delta-ODD for variants, code linkage. Authors say it applies beyond ABMs. https://www.jasss.org/23/2/7.html
- **CoMSES Net**: model library with peer review and Open Code Badge; backs codemeta, DataCite, ODD, TRACE. https://www.comses.net/resources/standards/
- **RO-Crate**: schema.org JSON-LD research packaging; used by WorkflowHub, OME and others. Version not on page. https://www.researchobject.org/ro-crate/
- **FAIR4RS** v1.0, 16 Mar 2022. https://www.rd-alliance.org/group_output/fair-principles-for-research-software-fair4rs-principles/
- CodeMeta, CITATION.cff, PROV-O, SBML analogues: [NV].

---

## B4. Biodiversity
- **Darwin Core Conceptual Model + Data Package Guide (DwC-DP) ratified by TDWG in 2026** (page gave both 26 May and 23 Jun 2026; exact date unclear). Five years of work, six-month public review. DwC-DP removes the Darwin Core Archive "star schema" restriction. Next: schemas and a GBIF version of the guide. https://www.tdwg.org/news/2026/dwc-additions-ratified/
- GBIF's DwC-DP site still says "pre-release / v1.0 pending ratification" (lags the news). Built on Frictionless Data Package; mapping guide, designer tool, SQL schema, JSON profile listed as pending. IPT support [NV]. https://gbif.github.io/dwc-dp/
- **Humboldt Extension** ratified 2024 (doc dated 2024-03-26) for inventories and surveys. https://www.tdwg.org/standards/dwc/
- TDWG current standards list: ABCD, Audiovisual Core, Darwin Core, Latimer Core, TCS, WGSRPD and others. Plinian Core and Camtrap DP not listed there. https://www.tdwg.org/standards/
- **Camtrap DP**: Frictionless package (deployments, media, observations); MIT. https://camtrap-dp.tdwg.org/
- **GBIF** datasets must be CC0, CC BY or CC BY-NC; GBIF says it will not legally enforce NC. https://www.gbif.org/terms
- **GBIF sensitive species guidance** (Chapman 2020): four categories (withhold; 0.1 deg; 0.01 deg; 0.001 deg); prefers generalisation over randomisation; never alter stored records; document with `dwc:dataGeneralizations` / `dwc:informationWithheld`. https://docs.gbif.org/sensitive-species-best-practices/master/en/
- **iNaturalist**:
  - Terms: 13+ unless parental consent; default content licence CC BY-NC; prohibits training commercial AI on its data. https://www.inaturalist.org/pages/terms
  - API: about 1 request/s and 10k/day; media 5 GB/h, 24 GB/day. https://www.inaturalist.org/pages/api+recommended+practices
  - AWS open dataset: 400M+ photos, monthly metadata snapshots, per-photo licences. https://github.com/inaturalist/inaturalist-open-data
  - Geoprivacy: obscured = random point in a 0.2 x 0.2 deg cell, date coarsened to month; automatic for threatened taxa. Note this randomises, which GBIF guidance discourages. https://help.inaturalist.org/en/support/solutions/articles/151000169938-what-is-geoprivacy-what-does-it-mean-for-an-observation-to-be-obscured-
  - Quality grades: Research Grade needs more than 2/3 agreement at species level; **captive/cultivated organisms are Casual**, so garden plantings never reach Research Grade. https://help.inaturalist.org/en/support/solutions/articles/151000169936-what-is-the-data-quality-assessment-and-how-do-observations-qualify-to-become-research-grade-
- **eBird**: research and education only; no commercial use without permission; no redistribution of original data. https://www.birds.cornell.edu/home/ebird-data-access-terms-of-use/
- **FeederWatch**: raw CSVs 1988 to 2024 (1.8M+ checklists), free with attribution; no formal licence stated. https://feederwatch.org/explore/raw-dataset-requests/
- **BirdNET-Analyzer**: code MIT, models CC BY-NC-SA 4.0; 6,512 species. https://github.com/birdnet-team/BirdNET-Analyzer
- **Pl@ntNet API**: 500 IDs/day free; commercial EUR 1,000/yr for 200k. https://my.plantnet.org/terms_of_use
- **GloBI**: TSV/CSV, DwC-A, RDF n-quads, Neo4j, Parquet, SQLite/DuckDB; attribution to original datasets; Zenodo versions. RO vocabulary use [NV]. https://www.globalbioticinteractions.org/data
- **AusTraits**: CC-BY 4.0; 500+ traits, 30,000+ taxa; built with `traits.build`. https://austraits.org/
- **TRY**: v7 reported (22.86M records, 306,701 taxa); "free and open access"; CC BY is [NV]; version date returned (4 Sep 2026) suspect. https://www.try-db.org/TryWeb/Home.php
- **USA-NPN**: CC BY 4.0 (not CC0), fixed acknowledgement text. https://www.usanpn.org/terms
- **PhenoCam**: CC BY 4.0 (v1.0 dataset was CC0); v3.0 covers 2000 to 2023. https://phenocam.nau.edu/webcam/fairuse_statement/
- **NEON**: page gives citation format and DOIs; CC0 for data not stated there [NV]. https://www.neonscience.org/data-samples/data-policies-citation
- ABCD/Audubon/Plinian detail, EML, COL/ChecklistBank (robots-blocked), WFO, Wikidata, Open Tree, BIEN, Merlin, LTER/EDI, MIxS, Movebank, OBIS: [NV].

## B5. Environment, soil, weather
| Dataset | Licence / terms | Notes | Source |
|---|---|---|---|
| SoilGrids 2.0 | CC BY 4.0 | 250 m, six depth intervals; WCS, WebDAV, Earth Engine | https://docs.isric.org/globaldata/soilgrids/SoilGrids_faqs_02.html |
| WoSIS | Per-source licences | 230k+ profiles, 174 countries (mid-2025) | https://www.isric.org/explore/wosis |
| GLOSIS ontology v1.0 | CC BY-NC-SA 3.0 IGO | From ISO 28258 UML model | https://github.com/glosis-ld/glosis |
| SSURGO | US federal, National Geospatial Data Asset; explicit "public domain" wording not on page | 1:12,000 to 1:63,360; Web Soil Survey, Soil Data Access | https://www.nrcs.usda.gov/resources/data-and-reports/soil-survey-geographic-database-ssurgo |
| Open-Meteo | Data CC BY 4.0; code AGPLv3 | Free-tier limits not on page | https://open-meteo.com/en/licence |
| ERA5 | CC-BY | 0.25 deg hourly, 1940 to present, ~5-day latency | https://cds.climate.copernicus.eu/datasets/reanalysis-era5-single-levels?tab=overview |
| PRISM | Free to reproduce/distribute with set attribution; OSU retains ownership | Not advised for very long-term trends | https://prism.oregonstate.edu/terms/ |
| USDA Hardiness Zone Map 2023 | Terms not on downloads page [NV] | PNG/PDF/SVG; GIS terms unverified | https://planthardiness.ars.usda.gov/pages/map-downloads |
| Daymet v4 | Licence not on page | 1 km daily, N. America 1980+ | https://daymet.ornl.gov/ |
| CoCoRaHS | CC BY 4.0 | | https://www.cocorahs.org/Content.aspx?page=datausagepolicy |
| Weather Underground PWS | Personal, non-commercial only; contributors grant TWC a perpetual irrevocable licence; users 18+ | Not reusable as open data | https://trust.weather.com/en-US/privacy/terms-of-use |
| OpenStreetMap | ODbL 1.0: attribution; share-alike on derivative databases; "produced works" may carry other terms; "collective databases" keep non-OSM parts separately licensable; internal use exempt | | https://osmfoundation.org/wiki/Licence/Licence_and_Legal_FAQ |
| Overture Maps | Places: CDLA-Permissive-2.0. Base, Buildings, Divisions, Transportation: ODbL. Addresses: mixed | | https://docs.overturemaps.org/attribution/ |
| ESA WorldCover | CC BY 4.0 | 10 m; 2020 v100, 2021 v200 (not directly comparable); COG | https://esa-worldcover.org/en/data-access |
| Dynamic World V1 | CC-BY 4.0 | 10 m, 9 classes, 2015 to present | https://developers.google.com/earth-engine/datasets/catalog/GOOGLE_DYNAMICWORLD_V1 |
| Annual NLCD | US federal | Collection 1.2 (10 Jun 2026), 1985 to 2025, 30 m | https://www.mrlc.gov/data |
| Ecoregions 2017 (RESOLVE) | CC-BY 4.0 | Shapefile, Earth Engine | https://ecoregions.appspot.com/ |
| One Earth Bioregions | **CC BY-NC 4.0**, download via form | 185 bioregions built from 844 ecoregions | https://www.oneearth.org/bioregions-2023/ |

Not verified: OpenLandMap, ISO 28258 text, NOAA/GHCN, Köppen datasets, CWOP, SMAP, Copernicus land cover, HydroSHEDS (licence page did not render), EPA ecoregions, GLOBE (robots-blocked).

## B6. Agriculture, food, gardening

### Farm and supply-chain standards
- **ADAPT Standard** (AgGateway): MIT; JSON schemas, data type definitions, unit abbreviations for B2B transfer of production data; successor to the 2015 plugin-based ADAPT Framework. Version/date not shown; ISO 11783 relation [NV]. https://github.com/ADAPT/Standard
- **farmOS**: assets and logs as the two main record types, plus quantities, terms, plans, data streams. Docs are CC BY-NC-SA 4.0. GPL and JSON:API are [NV]. https://farmos.org/model/
- **farmOS conventions**: explicitly *not* formally defined or enforced by core; informal and evolving. https://farmos.org/model/convention/
- **OpenTEAM**: fiscally sponsored by Raft Foundation; ecosystem of farmOS, SurveyStack, LiteFarm, LandPKS, Terraso. Ag Data Wallet page 404 [NV]. https://openteam.community/
- **LiteFarm**: GPL-3.0; UBC; React/Node/PostgreSQL. https://github.com/LiteFarmOrg/LiteFarm
- **SurveyStack** (Our Sci): licence not on page. https://www.our-sci.net/surveystack/
- **AgStack Asset Registry**: field boundary (WKT/GeoJSON) to GeoID, S2 cell tokens; licence not shown. https://github.com/agstack/asset-registry and https://agstack.org/
- **Ag Data Transparent**: AFBF principles (Nov 2014), updated 2024 to 16 principles including Ownership and Control, Portability, Data Retention and Deletion, Anonymization and Aggregation. https://www.agdatatransparent.com/principles
- **GiSC**: site in maintenance mode; cooperative status unverifiable. https://www.gisc.cloud/
- **AGROVOC**: 41,400 concepts, 1.219M terms, up to 42 languages; licence not on page [NV: CC BY]. https://www.fao.org/agrovoc/about
- ISO 11783, Crop Ontology, Planteome, ICASA, AgMIP, GODAN, FoodOn, INFOODS, Tania, FarmHack, Trellis/OATS, JoinData, FBN, EU Code of Conduct: [NV].

### Food, traceability, markets
- **FSMA 204**: compliance date moved from 20 Jan 2026 to **20 Jul 2028**. CTEs: harvesting, cooling, initial packing, first land-based receiver, shipping, receiving, transformation. https://www.fda.gov/food/food-safety-modernization-act-fsma/fsma-final-rule-requirements-additional-traceability-records-certain-foods
- Exemptions: produce farms averaging no more than $25,000/yr; retail food establishments no more than $250,000/yr (both inflation-adjusted, 3-year average). https://www.fda.gov/food/food-safety-modernization-act-fsma/frequently-asked-questions-fsma-food-traceability-rule
- **GS1 EPCIS 2.0** (June 2022): JSON/JSON-LD, REST bindings, SensorElement, AssociationEvent. https://ref.gs1.org/standards/epcis/
- **GS1 Digital Link** URI syntax 1.5.0 (June 2024); resolver spec 1.0.0 (Feb 2024). GTIN cost and Sunrise 2027 [NV]. https://www.gs1.org/standards/gs1-digital-link
- **PLU codes** (IFPS): voluntary; 4-digit conventional, 5-digit with 9 prefix organic; 1,500+ codes; public search. https://www.ifpsglobal.com/plu-codes
- **USDA FoodData Central**: CC0; 1,000 requests/hour. https://fdc.nal.usda.gov/api-guide
- **Open Food Facts**: database ODbL, contents DbCL, images CC BY-SA. https://world.openfoodfacts.org/terms-of-use
- **USDA Local Food Directories**: farmers market, CSA, food hub, on-farm market, agritourism; pipe-delimited CSV and REST API with key. Lists venues, not vendors' products. https://www.usdalocalfoodportal.com/fe/datasharing/
- **USDA Market News (MARS API)**: commodity market reports. https://mymarketnews.ams.usda.gov/mymarketnews-api
- **USDA AMS / IC-FOODS Market Data Guidelines for Produce in Regional Food Systems**: "voluntary consensus data standard" template for fresh produce product info, wholesale-oriented. Date and adoption not stated. https://www.ams.usda.gov/services/local-regional/research-publications/market-data-guidelines
- **Data Food Consortium**: French non-profit (Lyon); semantic-web ontology (OWL/RDF) for producers, products, offers; ontology repo AGPL-3.0; business rules "not yet encoded"; no version shown. Connectors in Ruby, PHP, JS/TS; OIDC. Adopters named: Open Food Network, Big Barn, LiteFarm, Locavora, Shopify via Hodmedods connector; Ooooby paused. https://goatech.org/2026/04/23/data-food-consortium/ , https://github.com/datafoodconsortium/ontology , https://fooddatacollaboration.org.uk/technical/ , https://www.datafoodconsortium.org/
- **Open Food Network**: AGPL-3.0, Rails; API v0, v1 and a DFC API. https://github.com/openfoodfoundation/openfoodnetwork
- ReFED Insights Engine: public tools, terms not shown. https://refed.org/
- SNAP at markets (page redirected), cottage food laws, Local Food Marketplace: [NV].

### Plant and garden data
| Source | Status | Licence | Source URL |
|---|---|---|---|
| OpenFarm | **Shut down April 2025**, repo archived 22 Apr 2025 | Data CC0 | https://github.com/openfarmcc/OpenFarm |
| Growstuff | Repo active-looking (12,213 commits); models plantings and harvests; JSON:API Swagger docs exist | Code AGPL-3.0, data CC-BY-SA | https://github.com/Growstuff/growstuff |
| Trefle | Site says operating, ~399k species, 60 req/min free; no corrections accepted in last 12 weeks | Code AGPL-3.0; data under each source's licence | https://trefle.io/ |
| Perenual | Commercial freemium API, 10,000+ species | Proprietary terms | https://perenual.com/docs/api |
| PFAF | Online | Text CC BY 4.0; images CC BY-NC-ND; site code copyrighted | https://pfaf.org/user/cmspage.aspx?pageid=136 |
| openplantdb | Small hobby repo, 294 plants, JSON/CSV/SQL | CC0 | https://github.com/cwfrazier1/openplantdb |
| Permapeople | Fetch returned 429 | [NV] | n/a |
| USDA PLANTS | Redirected, not opened | [NV] | n/a |

- **OSSI**: pledge-based (not a licence); 550+ varieties, 52 breeders, 78 seed companies. https://osseeds.org/
- **Community Seed Network**: run by Seed Savers Exchange and SeedChange; US/Canada. https://www.communityseednetwork.org/
- **Farming Concrete**: Data Collection Toolkit with 20 methods in five categories (food production, environmental, social, health, economic); "Barn" for entry, "Mill" for download; Mill data CC BY-SA 4.0; site on WordPress 4.7 (maintenance looks minimal). https://farmingconcrete.org/ , https://farmingconcrete.org/mill/ , https://farmingconcrete.org/toolkit/
- **Budburst** (Chicago Botanic Garden): CC BY-NC-SA 4.0; 143,632 observations. https://budburst.org/
- **Homegrown National Park**: 52,594 participants, 189,695 acres self-reported; no open data access described. https://homegrownnationalpark.org/
- **Bionutrient Institute**: says all data is "open source and in the commons"; licence not specified. https://www.bionutrientinstitute.org/
- GreenThumb NYC open data (page did not render), ACGA, Five Borough Farm, Great Sunflower Project, NWF certification, iSeeChange, soil citizen science, Practical Plants, Seed Savers data, garden-planner exports: [NV].

## B7. Citizen-science standards and quality
- **PPSR Core** (Citizen Science Association working group): Common, Project, Dataset, Observation models; docs still on an alpha Docusaurus build. https://core.citizenscience.org/
- **STAplus 1.0** (OGC 22-022r1, 2023): adds Party, License, ObservationGroup, Relation, Campaign to SensorThings for many-contributor settings; backward compatible. https://docs.ogc.org/is/22-022r1/22-022r1.html
- **XLSForm**: spreadsheet form standard compiling to ODK XForms; ODK, KoboToolbox, Ona, Survey123, Enketo, CommCare, SurveyCTO, CyberTracker. https://xlsform.org/en/
- **SciStarter API**: project finder, participation tracking, PPSR-Core alignment. https://scistarter.org/api
- **Zooniverse**: classification platform with Project Builder. https://www.zooniverse.org/about
- **ECSA Ten Principles**: text is in a Zenodo PDF, not opened [NV for content]. https://www.ecsa.ngo/10-principles/
- Anecdata (page gave only metadata), CitSci.org, Epicollect5, COST Action ontology, OGC CS Interoperability Experiment: [NV].

## B8. Cross-cutting semantics and governance
- **schema.org Observation**: observationAbout, measuredProperty, observationDate, value, unitCode, measurementMethod; under 1,000 domains use it. https://schema.org/Observation
- **schema.org Taxon**: parentTaxon, childTaxon, taxonRank; from Bioschemas; minimal adoption. https://schema.org/Taxon
- **SOSA/SSN "2023 Edition"**: still a W3C Working Draft (not a Recommendation); adds ObservationCollection/ActuationCollection, system capabilities, broadens sensors to include humans and simulations; RDF implementation of ISO 19156:2023 (OMS). Draft date returned (30 Sep 2026) suspect. https://www.w3.org/TR/vocab-ssn-2023/
- **DCAT 3**: W3C Recommendation 22 Aug 2024; dataset series, versioning, checksums. https://www.w3.org/TR/vocab-dcat-3/
- **Data Package v2** (26 Jun 2024): Data Package, Data Resource, Table Dialect, Table Schema; extension mechanism (Camtrap DP; DwC-DP builds on it). Parquet support not stated. https://datapackage.org/
- **Croissant 1.0** (1 Mar 2024): schema.org/Dataset + JSON-LD. A 1.1 was not seen. https://docs.mlcommons.org/croissant/docs/croissant-spec.html
- **CARE Principles**: created 8 Nov 2018 (Gaborone); people- and purpose-oriented complement to FAIR. Sub-principle text not on page. https://www.gida-global.org/care
- **Local Contexts TK and BC Labels**: provenance, protocol, permission labels applied through the Local Contexts Hub. https://localcontexts.org/labels/about-the-labels/
- **CDLA**: Permissive-2.0 (June 2021) explicitly leaves results of computational analysis unrestricted; Sharing-1.0 has share-alike terms (detail not on page). https://cdla.dev/
- **COPPA**: under-13; photos, audio and geolocation precise to street + city are personal information; verifiable parental consent; rule amended 22 Apr 2025. https://www.ftc.gov/business-guidance/resources/complying-coppa-frequently-asked-questions
- **EU Data Act** (Reg. 2023/2854): in force 11 Jan 2024; applies 12 Sep 2025; connected-product design obligation Sep 2026. https://digital-strategy.ec.europa.eu/en/policies/data-act
- JSON-LD 1.1, OBO ontologies, QUDT/UCUM, GeoJSON RFC 7946, ISO 19115, CSVW, Solid, GDPR location data: [NV].

Licence-mixing implications evidenced in opened sources
- CC0 (FoodData Central, OpenFarm, pywatershed): no conditions.
- CC BY (SoilGrids, ERA5, Open-Meteo, WorldCover, Dynamic World, Ecoregions 2017, AusTraits, USA-NPN, CoCoRaHS): attribution chains accumulate.
- Share-alike (OSM and Open Food Facts under ODbL; Growstuff and Farming Concrete under CC BY-SA): derivative databases inherit the licence; ODbL separates "collective databases" and "produced works".
- Non-commercial (iNaturalist default CC BY-NC, Budburst, BirdNET models, One Earth Bioregions, GLOSIS ontology, eBird terms, PFAF images, Weather Underground PWS, APSIM): incompatible with small-business / farmers-market commercial use.

## 9. Does a gardener or market-vendor standard exist?

What exists
- **Growstuff**: a working data model and JSON:API for crops, plantings, harvests, seeds, with CC-BY-SA data. It is one application's API, not a ratified standard.
- **Farming Concrete toolkit**: 20 documented measurement methods for community gardens with CC BY-SA aggregated data. It is a methods protocol with a web app, not a machine-readable schema.
- **farmOS model + conventions**: general asset/log/quantity model that can hold plantings and harvests; conventions are explicitly informal and unenforced.
- **Data Food Consortium**: the closest thing to a vendor standard (producer, product, catalogue, offer; OIDC; JSON-LD), used by Open Food Network and a handful of platforms. Aimed at online short-supply-chain platforms; no version number surfaced; AGPL ontology.
- **USDA AMS / IC-FOODS produce guidelines**: voluntary product-attribute template, wholesale-oriented.
- **Generic pieces**: schema.org Observation/Taxon (low adoption), GS1 Digital Link and EPCIS 2.0, PLU codes, STAplus and PPSR Core for citizen-science observations, Darwin Core + Humboldt + DwC-DP for biodiversity surveys, USDA Local Food Directories (venues only).

What was not found
- No open, governed data standard specifically for home or community-garden records (beds, plantings, harvest weights, inputs, observations). Two searches surfaced only Growstuff, a hobby CC0 plant table and method toolkits.
- No open standard for in-person farmers-market vendor listings, provenance or sales. Search returned only DFC, the AMS guidelines and proprietary POS vendors.
- No mapping was found between garden/harvest records and Darwin Core, farmOS or DFC.
- The main open crop-knowledge commons (OpenFarm) closed in April 2025; remaining plant-care sources are share-alike, NC-restricted, proprietary or very small.
- iNaturalist's quality model classes cultivated organisms as Casual, so garden plantings sit outside the main biodiversity quality tier.
- No Rust binding for BMI is listed; FMI has a WebAssembly layered standard but nothing equivalent was seen for BMI or HLA.
- No ABM framework reviewed combines deterministic execution, browser/WASM runtime and distributed partitioning in one engine; krABMaga is nearest (Rust, WASM visualisation, MPI) and its paper reports no load balancing.
- GeoZarr and SOSA/SSN 2023 are still drafts; PPSR Core docs are on an alpha site; farmOS conventions are unenforced.

Items requested but not covered at all (not opened): Dual Universe, Dwarf Fortress world-gen, DIS, Ray, IPFS/IPLD, event sourcing, S2, rHEALPix, COG, Arrow Flight, xarray, CodeMeta, CFF, PROV-O, EML, WFO, Open Tree of Life, BIEN, Merlin, MIxS, Movebank, OBIS, OpenLandMap, GHCN, CWOP, SMAP, Köppen, HydroSHEDS, EPA ecoregions, ISO 11783, Crop Ontology, Planteome, ICASA, AgMIP, GODAN, FoodOn, INFOODS, Tania, FarmHack, Trellis/OATS, JoinData, FBN, EU Code of Conduct on ag data, ACGA, Five Borough Farm, Great Sunflower Project, NWF certification, iSeeChange, soil citizen-science projects, Epicollect5, CyberTracker detail, CitSci.org, COST Action ontology, JSON-LD 1.1, OBO ontologies, QUDT/UCUM, GeoJSON, ISO 19115, CSVW, Solid, GDPR, cottage food laws, SNAP authorisation.
