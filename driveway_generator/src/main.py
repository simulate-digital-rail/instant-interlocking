from railwayroutegenerator.generator import generate_from_planpro
from orm_planpro_converter.converter import ORMConverter

def generate_driveway_json():
    with open("out.ppxml", "w", encoding="utf-8") as planpro_file:
        planpro = ORMConverter().run("52.39385615174401 13.049869537353517 52.3902158368756 13.049440383911135 52.38821222613622 13.073966503143312 52.392153883603726 13.074588775634767") # Potsdam Hbf Westseite
        planpro_file.write(planpro)
    routes = generate_from_planpro("out.ppxml", output_format="python-objects")
    output = []
    for route in routes:
        previous_node = route.start_signal.previous_node
        route_json = []
        route_json.append({"uuid": route.start_signal.uuid, "name": route.start_signal.name, "type": "signal", "state": "Ks1"})
        for edge in route.edges:
            # find out which node comes first on the driveway because edges can be oriented both ways
            if edge.node_a == previous_node:
                current_node = edge.node_b
            else:
                current_node = edge.node_a
            # find out whether the previous point needs to be in a specific position
            match current_node:
                case previous_node.connected_on_left:
                    route_json.append({"uuid": previous_node.uuid, "type": "point", "state": "left"})
                case previous_node.connected_on_right:
                    route_json.append({"uuid": previous_node.uuid, "type": "point", "state": "right"})
            # find out whether the current point needs to be in a specific position
            match previous_node:
                case current_node.connected_on_left:
                    route_json.append({"uuid": current_node.uuid, "type": "point", "state": "left"})
                case current_node.connected_on_right:
                    route_json.append({"uuid": current_node.uuid, "type": "point", "state": "right"})
            previous_node = current_node
        output.append(route_json)
    print(len(output))
    print(output)


if __name__ == "__main__":
    generate_driveway_json()
