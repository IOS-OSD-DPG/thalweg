import argparse
import csv
import matplotlib.pyplot as plt

def main(inputfile):
    with open(inputfile) as csvfile:
        reader = csv.DictReader(csvfile)
        data = [(float(row["distance"]), -float(row["depth"])) for row in reader]
    x, y = zip(*data)

    _, ax = plt.subplots()
    ax.plot(x, y, color="black")
    ax.fill_between(x, y, min(y), color="grey")

    ax.set_xlabel("Distance (km)")
    ax.set_xlim([x[0], x[-1]])
    ax.set_ylabel("Depth (m)")
    ax.set_ylim([min(y), 0])

    plt.savefig(inputfile.replace(".csv", ".png"))

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Produce section chart")
    parser.add_argument("file", type=str, help="the file contianing section information")
    args = parser.parse_args()
    main(args.file)
