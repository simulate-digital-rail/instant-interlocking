
from railwayroutegenerator.generator import generate_from_planpro

filename = "Scheibenberg.ppxml"

def main():
    generate_from_planpro(filename, output_file_name="routes.json")

if __name__ == "__main__":
    main()
