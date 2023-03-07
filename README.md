# codegen

This repository contains the results of the Master Project 2022/23 "Dispatching Trains from the Cloud". 
It is divided into a few subprojects:

- `code_generation`: A program that given a JSON file from the [interlocking exporter](https://github.com/simulate-digital-rail/interlocking-exporter) generates interlocking code in Rust.
- `grpc_control_station`: A frontend for the generated interlocking that can be used with the EULYNX Live Lab UI.
- `track_element`: A Rust library that provides types and traits for representing driveways and common track elements.
- `ui`: A web-based application to generate interlockings from OpenRailwayMap data.