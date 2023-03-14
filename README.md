# codegen

This repository contains the results of the Master Project 2022/23 "Dispatching Trains from the Cloud". 
It is divided into a few subprojects:

- `code_generation`: A program that given a JSON file from the [interlocking exporter](https://github.com/simulate-digital-rail/interlocking-exporter) generates interlocking code in Rust using the [`track_element`](https://github.com/simulate-digital-rail/track_element) crate.
- `grpc_control_station`: A frontend for the generated interlocking that can be used with the EULYNX Live Lab UI.
- `ui`: A web-based application to generate interlockings from OpenRailwayMap data.

## Motivation

Even though digital tools are available, most railway planning still happens with analog methods.
Digital planning data generated from publicly available data sources could serve as a basis for
new building projects. This project explores a pipeline that transforms geodata from public sources
into planning data and generates executable interlocking code from this planning data.

## The Pipeline

This project presents a pipeline for generating executable interlocking code from planning data.

![The Steps of the Pipeline: Geodata, Planning Data, Code Generation, Testing, Run Anywhere](docs/pipeline.png)

The pipeline starts by extracting planning information from geodata (e.g. OpenRailwayMap). This planning
data (which includes information such as where tracks, points, and signals are located) is then used to
generate possible driveways and enriched with the information what states track elements have to be in
for a driveway to be set. We then use this enriched planning data to create executable Rust code that 
uses the `track_element` crate.

Beyond this step, two independent projects explored the viability of using WebAssembly to test and
deploy the generated code.

## yaramo
yaramo is a python library used to extract the planning data from a source file, generate driveways and other information and finally export `topology`, `placement` and `route` json files used by the interlocking. 

### Usage model
yaramo works as an exchange format where there can be an importer turning other formats into yaramo and one or more exporters turning the yaramo object into their specific formats. Moreover, there the yaramo topology can by modified to do things with the planning data, like finding driveways. 

The following projects operate on the yaramo library. Some of them are part of the [instant-interlocking pipeline](#the-pipeline).

**Importers**

- [ORM-Importer](https://github.com/simulate-digital-rail/orm-importer)
- [CLI-Importer](https://github.com/simulate-digital-rail/cli-importer)
- [PlanPro-Importer](https://github.com/simulate-digital-rail/planpro-importer)

**Modifiers**

- [RailwayRoute-Generator](https://github.com/simulate-digital-rail/railway-route-generator)
- [VacancySection-Generator](https://github.com/simulate-digital-rail/vacancy-section-generator)
- [BlockSignal-Generator](https://github.com/simulate-digital-rail/block-signal-generator)

**Exporters**

- [PlanPro-Exporter](https://github.com/simulate-digital-rail/planpro-exporter)
- [Interlocking-Exporter](https://github.com/simulate-digital-rail/interlocking-exporter)

The instant-interlocking pipeline uses the ORM-Importer to retrieve the planning data. Both the RailwayRoute-Generator and the VacancySection-Generator are used to enrich the yaramo object with relevant vacancy section and driveway data. Finally, the Interlocking-Exporter is used to generate the three json files which are the used to configure the interlocking and the UI.
