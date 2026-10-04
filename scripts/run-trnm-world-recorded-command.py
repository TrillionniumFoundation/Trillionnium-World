#!/usr/bin/env python3
"""Bounded POSIX command evidence; no status, review or release authority.

Programs must redact their own output. Metadata contains no command arguments
or environment values. Point-in-time source and path checks are not protection
against a hostile concurrent writer. Escaped process groups require runner or
cgroup containment; the recorder bounds its own drain and makes no claim that
all descendants were killed.
"""
from __future__ import annotations

import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import re
import selectors
import signal
import subprocess
import sys
import time


def utc() -> str:
    return datetime.now(timezone.utc).isoformat()


def git_identity() -> dict:
    def git(*args: str) -> str | None:
        try:
            return subprocess.check_output(['git', *args], stderr=subprocess.DEVNULL,
                                           timeout=5).decode().strip()
        except (OSError, subprocess.SubprocessError, UnicodeError):
            return None
    commit = git('rev-parse', 'HEAD')
    tree = git('rev-parse', 'HEAD^{tree}')
    dirty = git('status', '--porcelain=v1', '--untracked-files=no')
    valid = bool(commit and tree and re.fullmatch(r'[0-9a-f]{40}', commit)
                 and re.fullmatch(r'[0-9a-f]{40}', tree))
    return {'commit': commit if valid else None, 'tree': tree if valid else None,
            'tracked_clean': dirty == '' if dirty is not None else False}


def safe_output(path: Path) -> Path:
    if '..' in path.parts:
        raise ValueError('parent traversal is not accepted for evidence paths')
    path = path.absolute()
    current = Path(path.anchor)
    for part in path.parts[1:-1]:
        current /= part
        if current.is_symlink():
            raise ValueError('linked evidence directory')
        current.mkdir(exist_ok=True)
        if not current.is_dir():
            raise ValueError('evidence parent is not a directory')
    if path.is_symlink():
        raise ValueError('linked evidence output')
    return path


def record(command: list[str], log: Path, metadata: Path, max_bytes: int,
           timeout_seconds: float, kill_grace: float, require_source: bool = False,
           expected_commit: str | None = None, expected_tree: str | None = None) -> int:
    if os.name != 'posix':
        raise ValueError('POSIX process groups required')
    if (not command or not 1 <= max_bytes <= 256 * 1024 * 1024
            or not 0 < timeout_seconds <= 21600 or not 0 < kill_grace <= 30):
        raise ValueError('invalid command or resource budget')
    for expected in (expected_commit, expected_tree):
        if expected is not None and re.fullmatch(r'[0-9a-f]{40}', expected) is None:
            raise ValueError('invalid expected Git identity')
    require_source = require_source or expected_commit is not None or expected_tree is not None
    log, metadata = safe_output(log), safe_output(metadata)
    if log == metadata:
        raise ValueError('log and metadata must differ')
    # Exclusive creation never overwrites an earlier attempt. Trusted private
    # runner directories are still needed to exclude concurrent path replacement.
    with log.open('xb') as output, metadata.open('x', encoding='utf-8') as meta:
        before = git_identity()
        expected_matches = (before['commit'] is not None and before['tracked_clean']
                            and (expected_commit is None or before['commit'] == expected_commit)
                            and (expected_tree is None or before['tree'] == expected_tree))
        precondition_failed = require_source and not expected_matches
        started, start = utc(), time.monotonic()
        observed = retained = 0
        digest = hashlib.sha256()
        interrupted: list[int] = []
        handlers = {}
        proc = None
        selector = selectors.DefaultSelector()
        timed_out = orphaned_pipe = forced_close = False
        terminated_at = exit_observed_at = None
        child_code = None
        command_started = False
        failure_class = None

        def forward(number: int) -> None:
            if proc is not None:
                try:
                    os.killpg(proc.pid, number)
                except ProcessLookupError:
                    pass

        try:
            if not precondition_failed:
                for sig in (signal.SIGTERM, signal.SIGINT):
                    handlers[sig] = signal.getsignal(sig)
                    signal.signal(sig, lambda number, _frame: interrupted.append(number) if not interrupted else None)
                proc = subprocess.Popen(command, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
                                        stderr=subprocess.STDOUT, start_new_session=True, bufsize=0)
                command_started = True
                assert proc.stdout is not None
                selector.register(proc.stdout, selectors.EVENT_READ)
                while selector.get_map() or proc.poll() is None:
                    now = time.monotonic()
                    if now - start >= timeout_seconds:
                        timed_out = True
                    if proc.poll() is not None and exit_observed_at is None:
                        exit_observed_at = now
                    if (selector.get_map() and exit_observed_at is not None
                            and now - exit_observed_at > kill_grace):
                        orphaned_pipe = True
                    if (timed_out or interrupted or orphaned_pipe) and terminated_at is None:
                        forward(signal.SIGTERM)
                        terminated_at = now
                    if terminated_at is not None and now - terminated_at >= kill_grace:
                        forward(signal.SIGKILL)
                    # A descendant may escape the process group and retain the
                    # output FD. Never wait for its EOF beyond this hard bound.
                    if terminated_at is not None and now - terminated_at >= kill_grace + min(kill_grace, 1.0):
                        forced_close = bool(selector.get_map()) or proc.poll() is None
                        break
                    for key, _ in selector.select(timeout=0.05):
                        chunk = os.read(key.fd, 64 * 1024)
                        if not chunk:
                            selector.unregister(key.fileobj)
                            continue
                        observed += len(chunk)
                        kept = chunk[:max(0, max_bytes - retained)]
                        output.write(kept)
                        output.flush()
                        digest.update(kept)
                        retained += len(kept)
                child_code = proc.wait(timeout=min(kill_grace + 1, 5))
        except (OSError, ValueError, subprocess.SubprocessError) as error:
            failure_class = type(error).__name__
        finally:
            try:
                if proc is not None:
                    forward(signal.SIGKILL)
                    if proc.poll() is None:
                        proc.wait(timeout=min(kill_grace + 1, 5))
                    if child_code is None:
                        child_code = proc.poll()
            except (OSError, subprocess.SubprocessError) as error:
                failure_class = failure_class or type(error).__name__
            finally:
                if proc is not None and proc.stdout is not None:
                    proc.stdout.close()
                selector.close()
                for sig, handler in handlers.items():
                    signal.signal(sig, handler)

        truncated = observed > retained
        result = 70 if child_code is None else child_code if child_code >= 0 else 128 - child_code
        if failure_class:
            result = 70
        elif interrupted:
            result = 128 + interrupted[0]
        elif timed_out:
            result = 124
        elif (truncated or orphaned_pipe or forced_close) and result == 0:
            result = 125
        after = git_identity()
        source_bound = before == after and expected_matches
        binding_failed = precondition_failed or require_source and not source_bound
        # Even optional mode must not turn a changed known source into success.
        source_changed = before['commit'] is not None and before != after
        if precondition_failed or (binding_failed or source_changed) and result == 0:
            result = 125
        output.flush()
        os.fsync(output.fileno())
        summary = {
            'schema': 'trnm_world_recorded_command_v2', 'started_at': started,
            'completed_at': utc(), 'elapsed_seconds': round(time.monotonic() - start, 6),
            'command_started': command_started, 'failure_class': failure_class,
            'child_exit_code': child_code, 'recorder_exit_code': result,
            'timed_out': timed_out, 'interrupted_by': interrupted[0] if interrupted else None,
            'orphaned_output_pipe': orphaned_pipe, 'output_drain_forced_closed': forced_close,
            'output_bytes_observed': observed, 'output_bytes_retained': retained,
            'output_truncated': truncated, 'retained_log_sha256': digest.hexdigest(),
            'source_before': before, 'source_after': after,
            'source_identity_required': require_source, 'source_identity_bound': source_bound,
            'source_precondition_failed': precondition_failed, 'source_changed': source_changed,
            'expected_commit': expected_commit, 'expected_tree': expected_tree,
            'command_completed_successfully': result == 0,
            'raw_command_output_complete': bool(command_started and child_code is not None
                and not failure_class and not truncated and not timed_out and not interrupted
                and not orphaned_pipe and not forced_close),
            'descendant_cleanup': 'not_proved_without_runner_containment',
            'repository_native_admission': 'not_proved_by_local_recorder',
            'independent_acceptance': False, 'production_authorization': 'not_granted',
        }
        meta.write(json.dumps(summary, indent=2) + '\n')
        meta.flush()
        os.fsync(meta.fileno())
        print(json.dumps({key: summary[key] for key in ('recorder_exit_code', 'output_truncated',
            'timed_out', 'source_identity_bound', 'production_authorization')}))
        return result


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--log', required=True, type=Path)
    parser.add_argument('--metadata', required=True, type=Path)
    parser.add_argument('--max-bytes', type=int, default=16 * 1024 * 1024)
    parser.add_argument('--timeout-seconds', type=float, default=18000)
    parser.add_argument('--kill-grace-seconds', type=float, default=5)
    parser.add_argument('--require-source-binding', action='store_true')
    parser.add_argument('--expected-commit')
    parser.add_argument('--expected-tree')
    parser.add_argument('command', nargs=argparse.REMAINDER)
    args = parser.parse_args()
    command = args.command[1:] if args.command[:1] == ['--'] else args.command
    try:
        return record(command, args.log, args.metadata, args.max_bytes, args.timeout_seconds,
                      args.kill_grace_seconds, args.require_source_binding,
                      args.expected_commit, args.expected_tree)
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        print(f'RECORDED_COMMAND=FAIL category={type(error).__name__}', file=sys.stderr)
        return 70


if __name__ == '__main__':
    raise SystemExit(main())
