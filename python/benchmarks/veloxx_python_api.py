import csv

class DataFrame:
    def __init__(self, data):
        self.data = data

    def filter(self, func):
        filtered_data = [row for row in self.data if func(row)]
        return DataFrame(filtered_data)

    def group_by(self, column):
        groups = {}
        for row in self.data:
            key = row[column]
            if key not in groups:
                groups[key] = []
            groups[key].append(row)
        return GroupedDataFrame(groups)

def read_csv(file_path):
    with open(file_path, mode='r') as file:
        reader = csv.DictReader(file)
        data = [row for row in reader]
    return DataFrame(data)

class GroupedDataFrame:
    def __init__(self, groups):
        self.groups = groups

    def count(self):
        return {key: len(group) for key, group in self.groups.items()}