#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location("route_check", Path(__file__).with_name("check-trnm-world-http-route-inventory.py"))
assert SPEC and SPEC.loader
M = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(M)

SOURCE = '''pub fn build_router() { Router::new()
.route("/health", get(health))
.route("/v1/items/:id", post(api::item))
.layer(DefaultBodyLimit::max(body_limit))
.layer(middleware::from_fn_with_state(state.clone(), production_rate_limit)); }
'''


class RouteTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="world-route-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.write(M.SOURCE, SOURCE)
        inventory = {
            "schema": "trnm_world_http_route_inventory_v1",
            "profile": "world_legacy_local_alpha",
            "canonical_public_authority": False,
            "source": M.SOURCE,
            "body_limit_required": True,
            "rate_limit_required": True,
            "routes": [
                {"method":"GET","path":"/health","handler":"health","surface":"health","canonical_authority":False},
                {"method":"POST","path":"/v1/items/:id","handler":"api::item","surface":"product","canonical_authority":False},
            ],
        }
        self.write(M.INVENTORY, json.dumps(inventory))
        self.write(M.DOC, "# API\n\n| `GET` | `/health` | `health` |\n| `POST` | `/v1/items/:id` | `api::item` |\n")
        for path in M.SCHEMAS:
            self.write(path, json.dumps({"$schema":"https://json-schema.org/draft/2020-12/schema"}))

    def write(self, rel, content):
        path=self.root/rel
        path.parent.mkdir(parents=True,exist_ok=True)
        path.write_text(content)

    def fail(self):
        with self.assertRaises((M.RouteFailure, OSError, ValueError)):
            M.validate(self.root)

    def test_valid(self):
        self.assertEqual(M.validate(self.root),2)

    def test_missing_route(self):
        data=json.loads((self.root/M.INVENTORY).read_text())
        data['routes'].pop()
        (self.root/M.INVENTORY).write_text(json.dumps(data))
        self.fail()

    def test_extra_route(self):
        data=json.loads((self.root/M.INVENTORY).read_text())
        data['routes'].append({"method":"GET","path":"/extra","handler":"extra","surface":"health","canonical_authority":False})
        (self.root/M.INVENTORY).write_text(json.dumps(data))
        self.fail()

    def test_method_drift(self):
        data=json.loads((self.root/M.INVENTORY).read_text())
        data['routes'][0]['method']='POST'
        (self.root/M.INVENTORY).write_text(json.dumps(data))
        self.fail()

    def test_handler_drift(self):
        data=json.loads((self.root/M.INVENTORY).read_text())
        data['routes'][0]['handler']='other'
        (self.root/M.INVENTORY).write_text(json.dumps(data))
        self.fail()

    def test_authority_overclaim(self):
        data=json.loads((self.root/M.INVENTORY).read_text())
        data['canonical_public_authority']=True
        (self.root/M.INVENTORY).write_text(json.dumps(data))
        self.fail()

    def test_body_limit_required(self):
        (self.root/M.SOURCE).write_text(SOURCE.replace('DefaultBodyLimit::max(body_limit)','other'))
        self.fail()

    def test_document_coverage(self):
        (self.root/M.DOC).write_text('# API\n')
        self.fail()


if __name__ == '__main__':
    result=unittest.TextTestRunner(verbosity=1).run(unittest.defaultTestLoader.loadTestsFromTestCase(RouteTests))
    if not result.wasSuccessful():
        raise SystemExit(1)
    print('TRNM World HTTP route inventory negative fixtures: PASS')
