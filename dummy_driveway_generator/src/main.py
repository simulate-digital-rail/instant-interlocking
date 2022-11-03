import json

data = {
    'routes' : [
        {
            'uuid' : 'JohnDoe',
            'vmax' : 60,
        },
        {
            'uuid' : 'JaneDoe',
            'vmax' :  100,
        },
        {
            'uuid' : 'DonJoe',
            'vmax' :  100,
        }
    ]
}

def write_data():
    with open('data.json', 'w') as f:
        json.dump(data, f)
