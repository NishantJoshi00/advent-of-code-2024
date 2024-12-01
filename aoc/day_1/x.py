import logging, os

logger = logging.getLogger(__name__)

def part1():
    logger.info("Part 1")
    cmd = "cargo build --target wasm32-unknown-unknown --release --features part-1 --quiet 2>/dev/null"
    logger.info(cmd)
    os.system(cmd)
    return "target/wasm32-unknown-unknown/release/day_1.wasm"

def part2():
    logger.info("Part 2")
    cmd = "cargo build --target wasm32-unknown-unknown --release --features part-2 --quiet 2>/dev/null"
    logger.info(cmd)
    os.system(cmd)
    return "target/wasm32-unknown-unknown/release/day_1.wasm"

