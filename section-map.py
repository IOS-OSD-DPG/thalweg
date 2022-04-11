import argparse
import csv
import matplotlib.pyplot as plt

THRESHOLD = 50

def next_display_threshold(elevation):
    fifties = int(elevation) // -THRESHOLD
    return -THRESHOLD * (fifties + 1)

def main(inputfile):
    with open(inputfile) as csvfile:
        reader = csv.DictReader(csvfile)
        data = [(float(row["distance"]), -float(row["depth"])) for row in reader]
    x, y = zip(*data)
    bottom = next_display_threshold(min(y))

    _, ax = plt.subplots()
    ax.plot(x, y, color="black")
    ax.fill_between(x, y, bottom, color="grey")

    # bottom is negative, steps must be positive
    # +1 so that the last value is included
    steps = -bottom // THRESHOLD + 1
    ax.set_yticks(
        # labels must go negative
        [n * THRESHOLD * -1 for n in range(steps)],
        [n * THRESHOLD for n in range(steps)]
    )

    ax.set_xlabel("Distance (km)")
    ax.set_xlim([x[0], x[-1]])
    ax.set_ylabel("Depth (m)")
    ax.set_ylim([bottom, 0])

    plt.savefig(inputfile.replace(".csv", ".png"))

if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Produce section chart")
    parser.add_argument("file", type=str, help="the file contianing section information")
    args = parser.parse_args()
    main(args.file)
