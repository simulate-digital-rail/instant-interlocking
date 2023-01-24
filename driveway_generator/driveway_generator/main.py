import json

import yaramo.signal as signal
import yaramo.additional_signal as additional_signal
from orm_importer.importer import ORMImporter
from railwayroutegenerator.routegenerator import RouteGenerator


def generate_signal_state(
    signal: signal.Signal, max_speed: int | None
) -> tuple[dict, list[dict]]:
    additional_signals = []
    if max_speed:
        for add_signal in signal.additional_signals:
            if isinstance(add_signal, additional_signal.AdditionalSignalZs3):
                symbol = next(
                    (s for s in add_signal.symbols if s.value == max_speed // 10), None
                )
                if symbol:
                    additional_signals.append(
                        {
                            "uuid": add_signal.uuid,
                            "type": "additional_signal_zs3",
                            "symbols": [s.value for s in add_signal.symbols],
                            "state": symbol.value,
                        }
                    )
            if isinstance(add_signal, additional_signal.AdditionalSignalZs3v):
                symbol = next(
                    (s for s in add_signal.symbols if s.value == max_speed // 10), None
                )
                if symbol:
                    additional_signals.append(
                        {
                            "uuid": add_signal.uuid,
                            "type": "additional_signal_zs3v",
                            "symbols": [s.value for s in add_signal.symbols],
                            "state": symbol.value,
                        }
                    )
    return (
        {
            "uuid": signal.uuid,
            "name": signal.name,
            "type": "signal",
            "state": "Ks1",
        },
        additional_signals,
    )


def generate_driveway_json():
    topology = ORMImporter().run(
        # "52.39385615174401 13.049869537353517 52.3902158368756 13.049440383911135 52.38821222613622 13.073966503143312 52.392153883603726 13.074588775634767"
        "52.393489514923075 13.06776523590088 52.39077008760343 13.067657947540283 52.391062433215595 13.029785156250002 52.394139638403715 13.029677867889406"
    )  # Potsdam Hbf Westseite
    RouteGenerator(topology).generate_routes()
    output = []
    for route in topology.routes:
        previous_node = route.start_signal.previous_node()
        route_json = []
        signal_state, additional_signal_states = generate_signal_state(
            route.start_signal, route.maximum_speed
        )
        route_json.append(signal_state)
        for signal in additional_signal_states:
            route_json.append(signal)
        for edge in route.edges:
            # find out which node comes first on the driveway because edges can be oriented both ways
            if edge.node_a == previous_node:
                current_node = edge.node_b
            else:
                current_node = edge.node_a
            # find out whether the previous point needs to be in a specific position
            match current_node:
                case previous_node.connected_on_left:
                    route_json.append(
                        {"uuid": previous_node.uuid, "type": "point", "state": "left"}
                    )
                case previous_node.connected_on_right:
                    route_json.append(
                        {"uuid": previous_node.uuid, "type": "point", "state": "right"}
                    )
            # find out whether the current point needs to be in a specific position
            match previous_node:
                case current_node.connected_on_left:
                    route_json.append(
                        {"uuid": current_node.uuid, "type": "point", "state": "left"}
                    )
                case current_node.connected_on_right:
                    route_json.append(
                        {"uuid": current_node.uuid, "type": "point", "state": "right"}
                    )
            previous_node = current_node
        output.append(route_json)
    with open("driveways.json", "w", encoding="utf-8") as json_file:
        json.dump(output, json_file)


if __name__ == "__main__":
    generate_driveway_json()
