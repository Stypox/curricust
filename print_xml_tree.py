# prints a stable tree structure of an XML file
# used only to compare two XMLs when debugging them

import xml.etree.ElementTree as ET
from collections import defaultdict

def build_structure(element):
    children_map = defaultdict(list)

    for child in element:
        children_map[child.tag.split('}', 1)[-1]].append(child)
    for attrib in element.attrib:
        children_map[attrib] += []

    structure = {}

    for tag, elements in children_map.items():
        merged_child_structure = {}
        for el in elements:
            child_struct = build_structure(el)
            merge_dicts(merged_child_structure, child_struct)

        structure[tag] = merged_child_structure

    return structure


def merge_dicts(target, source):
    for key, value in source.items():
        if key not in target:
            target[key] = value
        else:
            merge_dicts(target[key], value)


def print_structure(structure, indent=0):
    for key in sorted(structure.keys()):
        print(" " * indent + key)
        print_structure(structure[key], indent + 4)


def xml_to_structure(file_path):
    tree = ET.parse(file_path)
    root = tree.getroot()

    print(root.tag)
    structure = build_structure(root)
    print_structure(structure, indent=4)

if __name__ == "__main__":
    import sys
    if len(sys.argv) != 2:
        print(f"Usage: python {sys.argv[0]} <xml_file>")
        sys.exit(1)
    xml_to_structure(sys.argv[1])
