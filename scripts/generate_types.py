#!/usr/bin/env python3
"""
Generate Rust types from the OpenAPI specification.

The script groups schemas into semantic modules and emits one file per module
under `src/types/generated`.
"""
from __future__ import annotations

import json
import re
from collections import defaultdict
from pathlib import Path
from textwrap import indent

ROOT = Path(__file__).resolve().parent.parent
SPEC_PATH = ROOT / "spec.json"
OUT_DIR = ROOT / "src/types" / "generated"

# Types that are already implemented manually and should not be re-generated.
MANUAL_TYPES = {
    "ApiVersion",
    "ExternalAccountStatusEnum",
    "ExternalAccountSubTypeEnum",
    "ExternalAcquiringPaymentTypeEnum",
    "ExternalBalanceStaticTypeEnum",
    "ExternalBalanceTypeEnum",
    "ExternalConsentTypeEnum",
    "ExternalCreditDebitIndicatorEnum",
    "ExternalTransactionStatusEnum",
    "ExternalTransationTypeEnum",
    "FinancialInstitutionIdentificationEnum",
    "Measure",
    "OrderType",
    "PaymentMethod",
    "PaymentObject",
    "VatType",
}

RUST_KEYWORDS = {
    "as",
    "break",
    "const",
    "continue",
    "crate",
    "else",
    "enum",
    "extern",
    "false",
    "fn",
    "for",
    "if",
    "impl",
    "in",
    "let",
    "loop",
    "match",
    "mod",
    "move",
    "mut",
    "pub",
    "ref",
    "return",
    "self",
    "Self",
    "static",
    "struct",
    "super",
    "trait",
    "true",
    "type",
    "unsafe",
    "use",
    "where",
    "while",
    # future reserved
    "async",
    "await",
    "dyn",
}


def load_spec() -> dict:
    with SPEC_PATH.open("r", encoding="utf-8") as fh:
        return json.load(fh)


def sanitize_type_name(name: str) -> str:
    """Convert schema name to a Rust-friendly PascalCase identifier."""
    parts = re.split(r"[^A-Za-z0-9]+", name)
    parts = [p for p in parts if p]
    ident = "".join(p[:1].upper() + p[1:] for p in parts)
    if ident and ident[0].isdigit():
        ident = f"T{ident}"
    return ident or "Unnamed"


def sanitize_variant_name(value: str, used: set[str]) -> str:
    parts = re.split(r"[^A-Za-z0-9]+", value)
    parts = [p for p in parts if p]
    ident = "".join(p[:1].upper() + p[1:] for p in parts) or "Variant"
    if ident[0].isdigit():
        ident = f"V{ident}"
    candidate = ident
    suffix = 2
    while candidate in used:
        candidate = f"{ident}{suffix}"
        suffix += 1
    used.add(candidate)
    return candidate


def sanitize_field_name(name: str) -> str:
    snaked = re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name)
    snaked = re.sub(r"[^A-Za-z0-9]+", "_", snaked).strip("_").lower()
    if not snaked:
        snaked = "field"
    if snaked in RUST_KEYWORDS:
        snaked += "_"
    return snaked


def module_for_schema(name: str) -> str:
    lower = name.lower()
    if lower.startswith("application__invoice__"):
        return "invoice"
    if lower.startswith("application__sbp__"):
        return "sbp"
    if lower.startswith("application__open_banking__"):
        return "account"
    if "acquiring" in lower:
        return "acquiring"
    if "cashbox" in lower:
        return "cashbox"
    if "qr" in lower:
        return "qr"
    if "sbp" in lower:
        return "sbp"
    if "merchant" in lower:
        return "merchant"
    if "invoice" in lower:
        return "invoice"
    if "payment" in lower:
        return "payment"
    if "statement" in lower:
        return "statement"
    if "account" in lower:
        return "account"
    if "balance" in lower:
        return "balance"
    if "consent" in lower:
        return "consent"
    if "customer" in lower:
        return "customer"
    if "webhook" in lower:
        return "webhook"
    if "card" in lower:
        return "card"
    if "receipt" in lower:
        return "receipt"
    if "contractor" in lower:
        return "contractor"
    if "tax" in lower:
        return "tax"
    if "document" in lower or "shipment" in lower or "packing" in lower:
        return "document"
    if "legal" in lower:
        return "legal"
    if "external" in lower:
        return "external"
    if "order" in lower:
        return "order"
    if "content" in lower:
        return "content"
    if "closing" in lower:
        return "closing"
    if "error" in lower:
        return "errors"
    return "common"


def ref_type_path(ref: str, type_map: dict[str, str]) -> str:
    name = ref.split("/")[-1]
    rust_name = type_map[name]
    if rust_name in MANUAL_TYPES:
        return f"crate::types::{rust_name}"
    return f"crate::types::generated::{rust_name}"


def rust_type(schema: dict, type_map: dict[str, str]) -> tuple[str, bool]:
    if "$ref" in schema:
        return ref_type_path(schema["$ref"], type_map), False
    if "anyOf" in schema or "oneOf" in schema:
        return "Value", True
    typ = schema.get("type")
    if typ == "array":
        inner, used_value = rust_type(schema.get("items", {}), type_map)
        return f"Vec<{inner}>", used_value
    if typ == "string":
        return "String", False
    if typ == "integer":
        return "i64", False
    if typ == "number":
        return "f64", False
    if typ == "boolean":
        return "bool", False
    return "Value", True


def render_enum(name: str, schema: dict) -> tuple[str, bool]:
    used: set[str] = set()
    lines = [
        "#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]",
        f"pub enum {name} {{",
    ]
    for raw in schema.get("enum", []):
        variant = sanitize_variant_name(str(raw), used)
        lines.append(f'    #[serde(rename = "{raw}")]')
        lines.append(f"    {variant},")
    lines.append("}")
    return "\n".join(lines), False


def render_struct(name: str, schema: dict, type_map: dict[str, str]) -> tuple[str, bool]:
    props: dict = schema.get("properties", {})
    required = set(schema.get("required", []))
    uses_value = False
    field_lines = []
    for prop_name in sorted(props):
        prop_schema = props[prop_name]
        field_type, used_value = rust_type(prop_schema, type_map)
        uses_value = uses_value or used_value
        if prop_name not in required:
            field_type = f"Option<{field_type}>"
        field_name = sanitize_field_name(prop_name)
        field_lines.append(f'    #[serde(rename = "{prop_name}")]')
        field_lines.append(f"    pub {field_name}: {field_type},")
    doc = schema.get("title")
    header = []
    if doc:
        header.append(f"/// {doc}")
    header.append("#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]")
    header.append(f"pub struct {name} {{")
    footer = ["}"]
    return "\n".join(header + field_lines + footer), uses_value


def generate():
    spec = load_spec()
    schemas = spec["components"]["schemas"]
    type_map = {name: sanitize_type_name(name) for name in schemas}

    modules: dict[str, list[str]] = defaultdict(list)
    module_uses_value: dict[str, bool] = defaultdict(bool)

    for schema_name in sorted(schemas):
        rust_name = type_map[schema_name]
        if rust_name in MANUAL_TYPES:
            continue
        module = module_for_schema(schema_name)
        schema = schemas[schema_name]
        if schema.get("type") == "string" and "enum" in schema:
            rendered, uses_value = render_enum(rust_name, schema)
        elif schema.get("type") == "object":
            rendered, uses_value = render_struct(rust_name, schema, type_map)
        else:
            alias_to, uses_value = rust_type(schema, type_map)
            rendered = f"pub type {rust_name} = {alias_to};"
        modules[module].append(rendered)
        module_uses_value[module] = module_uses_value[module] or uses_value

    OUT_DIR.mkdir(parents=True, exist_ok=True)
    mod_lines = []

    for module_name, definitions in sorted(modules.items()):
        file_path = OUT_DIR / f"{module_name}.rs"
        uses_value = module_uses_value[module_name]
        header = ["use serde::{Deserialize, Serialize};"]
        if uses_value:
            header.append("use serde_json::Value;")
        header.append("")
        content = "\n\n".join(definitions)
        file_path.write_text("\n".join(header) + content + "\n", encoding="utf-8")
        mod_lines.append(f"pub mod {module_name};")
        mod_lines.append(f"pub use {module_name}::*;")

    (OUT_DIR / "mod.rs").write_text("\n".join(mod_lines) + "\n", encoding="utf-8")


if __name__ == "__main__":
    generate()
