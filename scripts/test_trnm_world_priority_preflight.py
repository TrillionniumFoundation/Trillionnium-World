"""Independent endpoint-policy oracle and source-wiring checks, NOT Rust execution."""
from __future__ import annotations
import ipaddress
import json
from pathlib import Path
import re
import unittest

ROOT=Path(__file__).resolve().parents[1]
POLICY=ROOT/'trillionnium/crates/world-authority/trnm-world-server/src/bin/production_preflight/mod.rs'
BINARY=ROOT/'trillionnium/crates/world-authority/trnm-world-server/src/bin/trnm-world-production-preflight.rs'

def endpoint_host(value: str) -> str:
    if not value or len(value.encode())>512 or not value.isascii() or any(ord(c)<=32 or ord(c)==127 or c in '\\?#%@' for c in value):
        raise ValueError('invalid text')
    if not value.startswith('https://'):
        raise ValueError('invalid scheme')
    rest=value[len('https://'):]
    authority,_,path=rest.partition('/')
    if not re.fullmatch(r'[A-Za-z0-9._~/\-]*',path) or any(s in ('.','..') for s in path.split('/')):
        raise ValueError('invalid prefix')
    def port(suffix):
        if not suffix:return
        if not re.fullmatch(r':[1-9][0-9]{0,4}',suffix) or int(suffix[1:])>65535:
            raise ValueError('invalid port')
    def blocked4(ip):
        return ip.is_loopback or int(ip)==0 or ip.is_multicast or int(ip)==0xffffffff
    if authority.startswith('['):
        literal,sep,suffix=authority[1:].partition(']')
        if not sep:raise ValueError('unclosed IPv6')
        port(suffix)
        ip=ipaddress.IPv6Address(literal)
        if ip.is_loopback or ip.is_unspecified or ip.is_multicast:
            raise ValueError('forbidden address')
        embedded=ip.ipv4_mapped or (ipaddress.IPv4Address(int(ip)) if int(ip)<1<<32 else None)
        if embedded is not None and blocked4(embedded):raise ValueError('forbidden embedded address')
        return ip.compressed
    host,sep,tail=authority.partition(':');port(sep+tail)
    try:
        ip=ipaddress.IPv4Address(host)
    except ValueError:
        ip=None
    if ip is not None:
        if blocked4(ip):raise ValueError('forbidden IPv4')
        return str(ip)
    host=host.lower();last=host.rsplit('.',1)[-1]
    if not host or len(host)>253 or re.fullmatch(r'(?:[0-9]+|0x[0-9a-f]+)',last) or host=='localhost' or host.endswith('.localhost'):
        raise ValueError('forbidden hostname')
    if any(not re.fullmatch(r'[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?',label) for label in host.split('.')):
        raise ValueError('invalid label')
    return host

class PreflightPolicyTests(unittest.TestCase):
    def test_every_rust_negative_endpoint_vector_against_independent_oracle(self):
        source=POLICY.read_text()
        block=source.split('for value in [',1)[1].split('] {',1)[0]
        values=[json.loads(match) for match in re.findall(r'"(?:\\.|[^"\\])*"',block)]
        self.assertGreaterEqual(len(values),40)
        for value in values:
            with self.subTest(value=value),self.assertRaises(ValueError):endpoint_host(value)

    def test_positive_hosts_and_alias_normalization(self):
        for value,expected in (
            ('https://NAKAMA.example.com:443/api/v1','nakama.example.com'),
            ('https://nakama.svc.cluster.local/','nakama.svc.cluster.local'),
            ('https://192.0.2.4:8443','192.0.2.4'),
            ('https://[2001:0db8:0:0:0:0:0:1]:8443/v1','2001:db8::1')):
            self.assertEqual(endpoint_host(value),expected)

    def test_runtime_binary_calls_strict_endpoint_and_bounded_read(self):
        source=BINARY.read_text()
        self.assertIn('production_preflight::endpoint_host(value)',source)
        self.assertIn('production_preflight::read_bounded_config(&path, MAX_CONFIG_BYTES)',source)
        self.assertNotIn('fs::read(&path)',source)
        self.assertIn('"dns_egress_verified": false',source)
        self.assertIn('"filesystem_checks_are_point_in_time": true',source)

    def test_ancestor_symlink_check_is_wired_to_secret_and_evidence_paths(self):
        source=BINARY.read_text()
        self.assertIn('ordinary_absolute_path(path, false)',source)
        self.assertIn('ordinary_absolute_path(&config.evidence_root, true)',source)
        policy=POLICY.read_text()
        self.assertIn('path.components()',policy)
        self.assertIn('file_type().is_symlink()',policy)
        self.assertIn('file.take(cap)',policy)

if __name__=='__main__':unittest.main()
