import json

import yaramo.signal as signal
import yaramo.additional_signal as additional_signal
from orm_importer.importer import ORMImporter
from railwayroutegenerator.routegenerator import RouteGenerator

# from vacancy_section_generator import VacancySectionGenerator
from vacancy_section_generator.vacancy_section_generator import VacancySectionGenerator
from collections import defaultdict


def generate_signal_state(signal: signal.Signal, max_speed: int | None) -> dict:
    target_state = {"main": "ks2"}
    supported_states = defaultdict(list)
    supported_states["main"] = [state.name for state in signal.supported_states]

    for add_signal in signal.additional_signals:
        if isinstance(add_signal, additional_signal.AdditionalSignalZs3):
            supported_states["zs3"] = [s.value for s in add_signal.symbols]
            if (
                max_speed
                and (
                    symbol := additional_signal.AdditionalSignalZs3.AdditionalSignalSymbolZs3(
                        max_speed // 10
                    )
                )
                in add_signal.symbols
            ):
                target_state["zs3"] = symbol.value
        elif isinstance(add_signal, additional_signal.AdditionalSignalZs3v):
            supported_states["zs3v"] = [s.value for s in add_signal.symbols]
            if (
                max_speed
                and (
                    symbol := additional_signal.AdditionalSignalZs3v.AdditionalSignalSymbolZs3v(
                        max_speed // 10
                    )
                )
                in add_signal.symbols
            ):
                target_state["zs3v"] = symbol.value
        elif isinstance(
            add_signal, additional_signal.AdditionalSignalZs2
        ) or isinstance(add_signal, additional_signal.AdditionalSignalZs2v):
            # Zs2 not supported in track_element yet
            continue
        else:
            supported_states["additional"] += [
                symbol.name for symbol in add_signal.symbols
            ]

    if "zs3" in target_state.keys() and "hp2" in supported_states["main"]:
        target_state["main"] = "hp2"
    elif "hp2" in supported_states["main"] and not "hp1" in supported_states["main"]:
        target_state["main"] = "hp2"
    elif "hp1" in supported_states["main"]:
        target_state["main"] = "hp1"
    elif "ks2" in supported_states["main"]:
        target_state["main"] = "ks2"
    else:
        raise Exception("Main Signal should support any of (Hp1, Hp2, Ks2)")

    return {
        "uuid": signal.uuid,
        "name": signal.name,
        "type": "signal",
        "supported_states": supported_states,
        "state": target_state,
    }


def generate_driveway_json(polygon="52.393489514923075 13.06776523590088 52.39077008760343 13.067657947540283 52.391062433215595 13.029785156250002 52.394139638403715 13.029677867889406"):
    topology = ORMImporter().run(polygon)
    RouteGenerator(topology).generate_routes()
    VacancySectionGenerator(topology).generate()
    output = []
    for route_uuid, route in topology.routes.items():
        previous_node = route.start_signal.previous_node()
        route_json = {
            "start_signal": route.start_signal.uuid,
            "end_signal": route.end_signal.uuid,
        }
        route_states = []
        signal_state = generate_signal_state(route.start_signal, route.maximum_speed)
        route_states.append(signal_state)
        # for signal in additional_signal_states:
        #     route_json.append(signal)
        for edge in route.edges:
            for vacancy_section in route.vacancy_sections:
                vacancy_section = {
                    "type": "vacancy_section",
                    "uuid": vacancy_section.uuid,
                    "state": "free",
                    "previous_signals": [],
                }
                if len(edge.signals) > 0:
                    try:
                        sig = next(
                            (
                                sig.uuid
                                for sig in edge.signals
                                if sig.kind == signal.SignalKind.Hauptsignal
                            )
                        )
                        vacancy_section["previous_signals"].append(sig)
                    except StopIteration:
                        pass

                route_states.append(vacancy_section)
        for edge in route.edges:
            # find out which node comes first on the driveway because edges can be oriented both ways
            if edge.node_a == previous_node:
                current_node = edge.node_b
            else:
                current_node = edge.node_a
            # find out whether the previous point needs to be in a specific position
            match current_node:
                case previous_node.connected_on_left:
                    route_states.append(
                        {"uuid": previous_node.uuid, "type": "point", "state": "left"}
                    )
                case previous_node.connected_on_right:
                    route_states.append(
                        {"uuid": previous_node.uuid, "type": "point", "state": "right"}
                    )
            # find out whether the current point needs to be in a specific position
            match previous_node:
                case current_node.connected_on_left:
                    route_states.append(
                        {"uuid": current_node.uuid, "type": "point", "state": "left"}
                    )
                case current_node.connected_on_right:
                    route_states.append(
                        {"uuid": current_node.uuid, "type": "point", "state": "right"}
                    )
            previous_node = current_node
        route_json["states"] = route_states
        output.append(route_json)
    with open("driveways.json", "w", encoding="utf-8") as json_file:
        json.dump(output, json_file)


if __name__ == "__main__":
    generate_driveway_json()
