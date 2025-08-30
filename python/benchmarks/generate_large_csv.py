import csv
import random
import string

# Generate a larger CSV dataset
def generate_large_csv(filename, num_rows=10000):
    with open(filename, 'w', newline='') as csvfile:
        fieldnames = ['name', 'age', 'city']
        writer = csv.DictWriter(csvfile, fieldnames=fieldnames)
        writer.writeheader()
        
        for i in range(num_rows):
            name = ''.join(random.choices(string.ascii_uppercase + string.ascii_lowercase, k=5))
            age = random.randint(18, 65)
            city = random.choice(['New York', 'Los Angeles', 'Chicago', 'Houston', 'Phoenix', 'Philadelphia', 'San Antonio', 'San Diego', 'Dallas', 'San Jose'])
            writer.writerow({'name': name, 'age': age, 'city': city})

if __name__ == "__main__":
    generate_large_csv("large_sample.csv", 10000)