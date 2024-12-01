import os, sys, time


def prep_runner():
    os.system("cargo build --release")

def run_day(day_no):
    parent = __import__("aoc.day_{}.x".format(day_no))
    day = getattr(parent, "day_{}".format(day_no))
    x = getattr(day, "x")
    part1 = getattr(x, "part1")
    part2 = getattr(x, "part2")
    cases = os.listdir("aoc/day_{}/testcases".format(day_no))
    cases.sort(reverse=True)
    start = os.getcwd()
    os.chdir("aoc/day_{}/testcases".format(day_no))
    part_1_cases = [case for case in cases if case.startswith("1")]
    part_1_input = [case for case in part_1_cases if case.endswith(".in")]
    part_1_path = "aoc/day_{}/{}".format(day_no, part1())
    os.chdir(start)
    cmd = lambda input_name: "cat aoc/day_{}/testcases/{} | cargo run --release --quiet -- +wat {}".format(day_no, input_name, part_1_path)
    for input in part_1_input:
        with open("aoc/day_{}/testcases/{}".format(day_no, input[:-3] + ".out")) as f:
            expected = f.read().strip()

        start_time = time.time()
        with os.popen(cmd(input)) as f:
            output = f.read().strip()
            end_time = time.time()
            if expected != output:
                print("[{}/{}]\tfailed: expected={} actual={}".format(day_no, input[2:-3], expected, output))
            else:
                print("[{}/{}]\tpassed: time={:.2f}".format(day_no, input[2:-3], end_time - start_time))


    start = os.getcwd()
    os.chdir("aoc/day_{}/testcases".format(day_no))
    part_2_cases = [case for case in cases if case.startswith("2")]
    part_2_input = [case for case in part_2_cases if case.endswith(".in")]
    part_2_path = "aoc/day_{}/{}".format(day_no, part2())
    os.chdir(start)
    cmd = lambda input_name: "cat aoc/day_{}/testcases/{} | cargo run --release --quiet -- +wat {}".format(day_no, input_name, part_2_path)
    for input in part_2_input:
        with open("aoc/day_{}/testcases/{}".format(day_no, input[:-3] + ".out")) as f:
            expected = f.read().strip()
        start_time = time.time()
        with os.popen(cmd(input)) as f:
            output = f.read().strip()
            end_time = time.time()
            if expected != output:
                print("[{}/{}]\tfailed: expected={} actual={}".format(day_no, input[2:-3], expected, output))
            else:
                print("[{}/{}]\tpassed: time={:.2f}".format(day_no, input[2:-3], end_time - start_time))







if __name__ == "__main__":
    day = sys.argv[1] if len(sys.argv) > 1 else None
    days = os.listdir("aoc")
    if day:
        run_day(day)
    else:
        for day in days:
            if day.startswith("day_"):
                day_no = day[4:]
                run_day(int(day_no))
