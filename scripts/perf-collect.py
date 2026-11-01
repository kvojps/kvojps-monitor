"""Sample explicitly identified native processes without reading application content."""

import argparse
import json
import os
from pathlib import Path
import statistics
import time


def read_process(pid):
    directory = Path('/proc') / str(pid)
    # comm may contain spaces and parentheses; fields after its final ')' are fixed.
    fields = (directory / 'stat').read_text().rsplit(')', 1)[1].split()
    pss = next(
        int(line.split()[1])
        for line in (directory / 'smaps_rollup').read_text().splitlines()
        if line.startswith('Pss:')
    )
    return int(fields[19]), int(fields[11]) + int(fields[12]), pss


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--app', type=int, required=True)
    parser.add_argument('--webview', type=int, nargs='*', required=True)
    parser.add_argument('--collectors', type=int, nargs='*', default=[])
    parser.add_argument('--seconds', type=int, default=60)
    parser.add_argument('--mode', choices=['full', 'reduced', 'closed'], required=True)
    args = parser.parse_args()
    groups = {'app': [args.app], 'webview': args.webview, 'collectors': args.collectors}
    pids = [pid for group in groups.values() for pid in group]
    if args.seconds < 1 or min(pids) < 1 or len(pids) != len(set(pids)):
        parser.error('Use positive duration/PIDs and disjoint process groups.')
    if args.mode != 'closed' and not args.webview:
        parser.error('An open-panel round requires explicit webview PIDs.')

    ticks = os.sysconf('SC_CLK_TCK')
    try:
        previous = {pid: read_process(pid) for pid in pids}
        identities = {pid: value[0] for pid, value in previous.items()}
        started = previous_time = time.monotonic()
        samples = []
        for index in range(args.seconds):
            time.sleep(max(0, started + index + 1 - time.monotonic()))
            current = {pid: read_process(pid) for pid in pids}
            now = time.monotonic()
            if any(current[pid][0] != identities[pid] for pid in pids):
                raise RuntimeError('PID identity changed; repeat with the new process set.')
            sample = {'elapsed_seconds': now - started}
            for group, members in groups.items():
                sample[group] = {
                    'cpu_core_percent': 100 * sum(
                        current[pid][1] - previous[pid][1] for pid in members
                    ) / ticks / (now - previous_time),
                    'pss_kib': sum(current[pid][2] for pid in members),
                }
            samples.append(sample)
            previous, previous_time = current, now
        elapsed = previous_time - started
        summary = {}
        for group in groups:
            intervals = [samples[0]['elapsed_seconds']] + [
                right['elapsed_seconds'] - left['elapsed_seconds']
                for left, right in zip(samples, samples[1:])
            ]
            summary[group] = {
                'mean_cpu_core_percent': sum(
                    sample[group]['cpu_core_percent'] * interval
                    for sample, interval in zip(samples, intervals)
                ) / elapsed,
                'mean_pss_kib': statistics.mean(s[group]['pss_kib'] for s in samples),
                'max_pss_kib': max(s[group]['pss_kib'] for s in samples),
            }
        print(json.dumps({'mode': args.mode, 'process_groups': groups,
                          'summary': summary, 'samples': samples}, indent=2))
    except (OSError, StopIteration, ValueError, RuntimeError) as error:
        # No partial result may look like a successful 60-second round.
        parser.exit(1, f'Invalid capture ({type(error).__name__}); check access and process set.\n')


if __name__ == '__main__':
    main()
