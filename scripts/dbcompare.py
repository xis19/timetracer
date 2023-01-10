#! /usr/bin/env python3

import argparse
import os
import os.path
import sqlite3
import sys


SIGNIFICANT_DIFF = 1e5


def _get_database_path(path: str):
    full_path = os.path.abspath(path)
    base_dir = os.path.split(full_path)[0]
    return base_dir


def _load_database(path: str):
    content = {"objects": {}, "source": {}, "instantiate_class": {}}
    db_path = _get_database_path(path)

    connect = sqlite3.connect(path)

    for row in connect.execute(
        "SELECT path, total_time, frontend, backend FROM objects"
    ):
        key = row[0]
        if key.startswith(db_path):
            key = key[len(db_path) :]
        content["objects"][row[0]] = {
            "total_time": row[1],
            "frontend": row[2],
            "backend": row[3],
        }

    for row in connect.execute("SELECT path, duration, count FROM source"):
        key = row[0]
        if key.startswith(db_path):
            key = key[len(db_path) :]
        content["source"][key] = {"duration": row[1], "count": row[2]}

    for row in connect.execute("SELECT name, duration, count FROM instantiate_class"):
        content["instantiate_class"][row[0]] = {"duration": row[1], "count": row[2]}

    return content


def _compare_objects_table(objects_table1, objects_table2):
    result = {}

    objects1 = set(objects_table1.keys())
    objects2 = set(objects_table2.keys())

    result["db1_only"] = objects1 - objects2
    result["db2_only"] = objects2 - objects1
    result["time_diff"] = {}

    for item in objects1.intersection(objects2):
        result["time_diff"][item] = {}
        for field in ["total_time", "frontend", "backend"]:
            result["time_diff"][item][field] = (
                objects_table1[item][field] - objects_table2[item][field]
            )

    return result


def _compare_source_table(source_table1, source_table2):
    result = {}

    source1 = set(source_table1.keys())
    source2 = set(source_table2.keys())

    result["db1_only"] = source1 - source2
    result["db2_only"] = source2 - source1
    result["time_diff"] = {}

    for item in source1.intersection(source2):
        result["time_diff"][item] = {
            "duration": source_table1[item]["duration"]
            - source_table2[item]["duration"],
            "count": source_table1[item]["count"] - source_table2[item]["count"],
        }

    return result


def _compare_instantiate_class(instantiate1, instantiate2):
    result = {}
    
    c1 = set(instantiate1.keys())
    c2 = set(instantiate2.keys())

    result["db1_only"] = c1 - c2
    result["db2_only"] = c2 - c1
    result["time_diff"] = {}

    for item in c1.intersection(c2):
        result["time_diff"][item] = {
            "duration": instantiate1[item]["duration"]
            - instantiate2[item]["duration"],
            "count": instantiate1[item]["count"] - instantiate2[item]["count"],
        } 

    return result

def _report_instantiate_class_diff(diff):
    print("Instantiate class in Database 1 only:")
    for item in diff["db1_only"]:
        print(f"\t{item}")
    print("Instantiate class in Database 2 only:")
    for item in diff["db2_only"]:
        print(f"\t{item}")

    print("Instantiate class differences")
    for item, content in diff["time_diff"].items():
        if abs(content["duration"]) > SIGNIFICANT_DIFF:
            print(
                "    {:>80s}    {: 10.6f}    {: 10d}".format(
                    item, content["duration"] / 1e6, content["count"]
                )
            )    

def _report_source_diff(source_diff):
    print("Sources in Database 1 only:")
    for item in source_diff["db1_only"]:
        print(f"\t{item}")
    print("Sources in Database 2 only:")
    for item in source_diff["db2_only"]:
        print(f"\t{item}")

    print("Source differences")
    for item, content in source_diff["time_diff"].items():
        if abs(content["duration"]) > SIGNIFICANT_DIFF:
            print(
                "    {:>80s}    {: 10.6f}    {: 10d}".format(
                    item, content["duration"] / 1e6, content["count"]
                )
            )


def _report_objects_diff(objects_diff):
    print("Objects in Database 1 only:")
    for item in objects_diff["db1_only"]:
        print(f"\t{item}")

    print("Objects in Database 2 only:")
    for item in objects_diff["db2_only"]:
        print(f"\t{item}")

    print("Objects differences")
    for item, content in objects_diff["time_diff"].items():
        if any(
            abs(content[field]) > SIGNIFICANT_DIFF
            for field in ("total_time", "frontend", "backend")
        ):
            print(
                "    {:>80s}    {: 10.6f}    {: 10.6f}    {: 10.6f}".format(
                    item,
                    content["total_time"] / 1e6,
                    content["frontend"] / 1e6,
                    content["backend"] / 1e6,
                )
            )


def _setup_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("db1", nargs=1, type=str, help="First database file")
    parser.add_argument("db2", nargs=1, type=str, help="Second database file")
    return parser.parse_args()


def main():
    args = _setup_args()
    db1 = args.db1[0]
    db2 = args.db2[0]

    db1_content = _load_database(db1)
    db2_content = _load_database(db2)

    objects_diff = _compare_objects_table(
        db1_content["objects"], db2_content["objects"]
    )
    source_diff = _compare_source_table(db1_content["source"], db2_content["source"])
    instantiate_class_diff = _compare_instantiate_class(db1_content["instantiate_class"], db2_content["instantiate_class"])

    _report_objects_diff(objects_diff)
    _report_source_diff(source_diff)
    _report_instantiate_class_diff(instantiate_class_diff)


if __name__ == "__main__":
    sys.exit(main())
