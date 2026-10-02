# Specifications

Everything in this folder is dedicated to the public domain under CC0 1.0, so that anyone may implement it without asking.

| File | Status |
|---|---|
| `pack.schema.json` | JSON Schema for a content pack, engine 0.1 |
| `world.schema.json` | JSON Schema for a starting world, engine 0.1 |
| Data profile | Not started. The plan is in `docs/design.md` under "Data standards" |

## Data profile: plan in brief

The profile will be a documented way to combine existing standards inside one Frictionless Data Package: Darwin Core for sightings and surveys, SOSA and STAplus terms for measurements, GBIF or Wikidata identifiers for species, H3 cells for place. It adds two small schemas where no open standard was found: **garden records** (beds, plantings, harvests, inputs), mapped onto the farmOS asset and log model, and **market listings** (seller, product, offer, origin), mapped onto the Data Food Consortium's terms.

Every record will carry: source type (observed, measured, self-reported, estimated or simulated), an evidence tier, a pseudonymous contributor, a licence, and the location precision the contributor agreed to publish.

These schemas should be proposed inside existing communities (OpenTEAM, the Data Food Consortium, TDWG), not published alone. Nothing has been drafted yet.
