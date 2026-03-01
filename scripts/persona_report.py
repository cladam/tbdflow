#!/usr/bin/env python3
"""
tbdflow Persona Feedback Report Generator

Reads choreo JSON reports and the corresponding .chor source files,
then assembles "User Feedback as Code" artifacts in YAML format.

Usage:
    python3 scripts/persona_report.py [--dir reports/] [--chor tests/explorations/]
    python3 scripts/persona_report.py --report reports/choreo_test_report_20260301_131208.json
"""

import json
import os
import re
import sys
import hashlib
from datetime import datetime
from pathlib import Path

# ─────────────────────────────────────────────────────────────────────────────
# Parse persona metadata from .chor source files
# ─────────────────────────────────────────────────────────────────────────────

def extract_chor_metadata(chor_path: str) -> dict:
    """Extract persona profile, observations, and findings from a .chor file."""
    metadata = {
        "persona": "",
        "profile": "",
        "priority": "",
        "strategy": "",
        "thoughts": [],
        "ux_findings": [],
        "ux_observations": [],
        "journals": [],
    }

    with open(chor_path, "r") as f:
        lines = f.readlines()

    in_header = True
    header_lines = []

    for line in lines:
        stripped = line.strip()

        # Collect header comment block
        if in_header and stripped.startswith("#"):
            header_lines.append(stripped.lstrip("# ").strip())
        elif in_header and stripped and not stripped.startswith("#"):
            in_header = False

        # Extract System log entries
        log_match = re.search(r'System log "(.+)"', stripped)
        if log_match:
            msg = log_match.group(1)
            if msg.startswith("THOUGHT:"):
                metadata["thoughts"].append(msg[len("THOUGHT:"):].strip())
            elif msg.startswith("UX FINDING:"):
                metadata["ux_findings"].append(msg[len("UX FINDING:"):].strip())
            elif msg.startswith("UX OBSERVATION:"):
                metadata["ux_observations"].append(msg[len("UX OBSERVATION:"):].strip())
            elif msg.startswith("JOURNAL:"):
                metadata["journals"].append(msg[len("JOURNAL:"):].strip())

        # Extract inline comments that are observations
        comment_match = re.search(r'#\s*(UX FINDING|FRICTION|OBSERVATION|RELIEF|DISCOVERY):?\s*(.+)', stripped)
        if comment_match:
            tag = comment_match.group(1)
            text = comment_match.group(2).strip()
            if tag == "UX FINDING" or tag == "FRICTION":
                metadata["ux_findings"].append(text)
            elif tag in ("OBSERVATION", "DISCOVERY", "RELIEF"):
                metadata["ux_observations"].append(text)

    # Parse header
    for line in header_lines:
        if "Profile:" in line:
            metadata["profile"] = line.split("Profile:", 1)[1].strip()
        elif "Priority:" in line:
            metadata["priority"] = line.split("Priority:", 1)[1].strip()
        elif "Exploration strategy:" in line:
            metadata["strategy"] = line.split("Exploration strategy:", 1)[1].strip()
        elif "Persona:" in line or "— Exploration Agent:" in line:
            # Try to extract persona name from header
            pass

    return metadata


# ─────────────────────────────────────────────────────────────────────────────
# Build friction points from choreo JSON report
# ─────────────────────────────────────────────────────────────────────────────

def classify_severity(step: dict, test_description: str) -> str:
    """Classify friction severity based on test outcome and context."""
    status = step["result"]["status"]
    error = step["result"].get("errorMessage", "")

    if status == "passed":
        # Check if the test EXPECTED failure (friction was anticipated)
        return "NONE"
    elif status == "failed":
        if "panic" in error.lower() or "fatal" in error.lower():
            return "CRITICAL"
        elif "not a valid" in error.lower() or "Invalid" in error:
            return "MEDIUM"
        else:
            return "HIGH"
    elif status == "skipped":
        return "LOW"
    return "UNKNOWN"


def build_friction_points(elements: list) -> list:
    """Extract friction points from scenario test results."""
    friction_points = []

    for scenario in elements:
        flow_name = scenario["name"]
        for step in scenario["steps"]:
            status = step["result"]["status"]
            error = step["result"].get("errorMessage", "")
            severity = classify_severity(step, step.get("description", ""))

            if status == "failed" and severity != "NONE":
                friction_points.append({
                    "flow": flow_name,
                    "step": step["name"],
                    "description": step.get("description", ""),
                    "issue": error,
                    "severity": severity,
                    "duration_ms": step["result"].get("durationInMs", 0),
                })
            elif status == "skipped":
                friction_points.append({
                    "flow": flow_name,
                    "step": step["name"],
                    "description": step.get("description", ""),
                    "issue": "Test skipped — blocked by upstream failure",
                    "severity": "LOW",
                    "duration_ms": 0,
                })

    return friction_points


def build_flow_summaries(elements: list) -> list:
    """Build per-flow (scenario) summaries."""
    flows = []
    for scenario in elements:
        total = len(scenario["steps"])
        passed = sum(1 for s in scenario["steps"] if s["result"]["status"] == "passed")
        failed = sum(1 for s in scenario["steps"] if s["result"]["status"] == "failed")
        skipped = sum(1 for s in scenario["steps"] if s["result"]["status"] == "skipped")
        total_ms = sum(s["result"].get("durationInMs", 0) for s in scenario["steps"])
        outcome = "SUCCESS" if failed == 0 and skipped == 0 else "PARTIAL" if passed > 0 else "BLOCKED"

        flows.append({
            "name": scenario["name"],
            "outcome": outcome,
            "tests": total,
            "passed": passed,
            "failed": failed,
            "skipped": skipped,
            "duration_seconds": round(total_ms / 1000, 2),
        })
    return flows


# ─────────────────────────────────────────────────────────────────────────────
# Extract persona name from feature name
# ─────────────────────────────────────────────────────────────────────────────

PERSONA_MAP = {
    "newbie": "The Nervous Newbie",
    "purist": "The TBD Purist",
    "refugee": "The Git-Flow Refugee",
    "architect": "The Monorepo Architect",
}

def infer_persona(feature_name: str, uri: str) -> str:
    """Infer persona name from feature name or file path."""
    combined = (feature_name + " " + uri).lower()
    for key, name in PERSONA_MAP.items():
        if key in combined:
            return name
    return feature_name


# ─────────────────────────────────────────────────────────────────────────────
# Generate the YAML feedback artifact
# ─────────────────────────────────────────────────────────────────────────────

def yaml_escape(s: str) -> str:
    """Escape a string for safe YAML output."""
    if not s:
        return '""'
    # If it contains special chars, quote it
    if any(c in s for c in (':', '#', '{', '}', '[', ']', ',', '&', '*', '?', '|', '-', '<', '>', '=', '!', '%', '@', '`', '"', "'")):
        return '"' + s.replace('\\', '\\\\').replace('"', '\\"') + '"'
    return s


def generate_report(report_path: str, chor_dir: str = None) -> str:
    """Generate a YAML feedback artifact from a choreo JSON report."""
    with open(report_path, "r") as f:
        data = json.load(f)

    if not isinstance(data, list) or len(data) == 0:
        return "# Empty report\n"

    feature = data[0]
    uri = feature.get("uri", "")
    feature_name = feature.get("name", "Unknown")
    elements = feature.get("elements", [])
    summary = feature.get("summary", {})

    # Generate session ID from report filename
    basename = os.path.basename(report_path)
    session_id = "sim-" + hashlib.md5(basename.encode()).hexdigest()[:6]

    # Infer persona
    persona = infer_persona(feature_name, uri)

    # Try to load .chor metadata
    chor_metadata = None
    if chor_dir and uri:
        # The uri is relative to where choreo was run
        chor_candidates = [
            os.path.join(chor_dir, os.path.basename(uri)),
            uri,  # try as-is
        ]
        for candidate in chor_candidates:
            if os.path.exists(candidate):
                chor_metadata = extract_chor_metadata(candidate)
                break

    # Build friction points and flow summaries
    friction_points = build_friction_points(elements)
    flows = build_flow_summaries(elements)

    # Overall outcome
    total_failed = sum(f["failed"] for f in flows)
    total_skipped = sum(f["skipped"] for f in flows)
    overall = "SUCCESS" if total_failed == 0 and total_skipped == 0 else "FRICTION_DETECTED"

    # Build YAML
    lines = []
    lines.append(f"# ═══════════════════════════════════════════════════════════════")
    lines.append(f"# Persona Feedback Artifact")
    lines.append(f"# Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    lines.append(f"# Source: {basename}")
    lines.append(f"# ═══════════════════════════════════════════════════════════════")
    lines.append(f"")
    lines.append(f"session_id: {yaml_escape(session_id)}")
    lines.append(f"persona: {yaml_escape(persona)}")
    lines.append(f"feature: {yaml_escape(feature_name)}")
    lines.append(f"source: {yaml_escape(uri)}")
    lines.append(f"outcome: {overall}")
    lines.append(f"timestamp: {yaml_escape(datetime.now().isoformat())}")
    lines.append(f"")

    # Summary
    lines.append(f"summary:")
    lines.append(f"  total_tests: {summary.get('tests', sum(f['tests'] for f in flows))}")
    lines.append(f"  total_failures: {summary.get('failures', total_failed)}")
    lines.append(f"  total_time_seconds: {summary.get('totalTimeInSeconds', sum(f['duration_seconds'] for f in flows))}")
    lines.append(f"")

    # Profile (from .chor metadata)
    if chor_metadata:
        lines.append(f"persona_profile:")
        if chor_metadata["profile"]:
            lines.append(f"  description: {yaml_escape(chor_metadata['profile'])}")
        if chor_metadata["priority"]:
            lines.append(f"  priority: {yaml_escape(chor_metadata['priority'])}")
        if chor_metadata["strategy"]:
            lines.append(f"  exploration_strategy: {yaml_escape(chor_metadata['strategy'])}")
        lines.append(f"")

    # Flows
    lines.append(f"flows:")
    for flow in flows:
        lines.append(f"  - name: {yaml_escape(flow['name'])}")
        lines.append(f"    outcome: {flow['outcome']}")
        lines.append(f"    tests: {flow['tests']}")
        lines.append(f"    passed: {flow['passed']}")
        lines.append(f"    failed: {flow['failed']}")
        lines.append(f"    skipped: {flow['skipped']}")
        lines.append(f"    duration_seconds: {flow['duration_seconds']}")
    lines.append(f"")

    # Friction points
    if friction_points:
        lines.append(f"friction_points:")
        for fp in friction_points:
            lines.append(f"  - flow: {yaml_escape(fp['flow'])}")
            lines.append(f"    step: {yaml_escape(fp['step'])}")
            lines.append(f"    description: {yaml_escape(fp['description'])}")
            lines.append(f"    issue: {yaml_escape(fp['issue'])}")
            lines.append(f"    severity: {fp['severity']}")
            lines.append(f"    duration_ms: {fp['duration_ms']}")
    else:
        lines.append(f"friction_points: []")
    lines.append(f"")

    # UX findings (from .chor source)
    if chor_metadata and chor_metadata["ux_findings"]:
        lines.append(f"ux_findings:")
        for finding in chor_metadata["ux_findings"]:
            lines.append(f"  - {yaml_escape(finding)}")
        lines.append(f"")

    # Observations
    if chor_metadata and chor_metadata["ux_observations"]:
        lines.append(f"observations:")
        for obs in chor_metadata["ux_observations"]:
            lines.append(f"  - {yaml_escape(obs)}")
        lines.append(f"")

    # Journal (persona's internal reflection)
    if chor_metadata and chor_metadata["journals"]:
        lines.append(f"journal:")
        for entry in chor_metadata["journals"]:
            lines.append(f"  - {yaml_escape(entry)}")
        lines.append(f"")

    # Recommendations (derived from friction points and findings)
    recommendations = derive_recommendations(friction_points, chor_metadata)
    if recommendations:
        lines.append(f"recommendations:")
        for rec in recommendations:
            lines.append(f"  - {yaml_escape(rec)}")
        lines.append(f"")

    return "\n".join(lines) + "\n"


def derive_recommendations(friction_points: list, metadata: dict) -> list:
    """Derive actionable recommendations from friction points and UX findings."""
    recs = []
    seen = set()

    for fp in friction_points:
        issue = fp["issue"].lower()
        if "scope" in issue and "lowercase" not in seen:
            recs.append("Fix is_valid_scope() to accept hyphens and underscores in addition to lowercase letters.")
            seen.add("lowercase")
        if "not a valid" in issue and "type_hint" not in seen:
            recs.append("Show allowed types in the error message when an invalid type is used.")
            seen.add("type_hint")
        if "capital" in issue and "capital_hint" not in seen:
            recs.append("Include the lowercase rule in the commit help examples.")
            seen.add("capital_hint")
        if "upstream" in issue and "upstream_hint" not in seen:
            recs.append("When push fails due to missing upstream, suggest 'tbdflow branch' instead of raw git push.")
            seen.add("upstream_hint")
        if "panic" in issue and "panic" not in seen:
            recs.append("CRITICAL: A panic was observed. This should never reach the user.")
            seen.add("panic")

    if metadata:
        for finding in metadata.get("ux_findings", []):
            fl = finding.lower()
            if "hyphen" in fl and "scope_fix" not in seen:
                recs.append("Update scope validation to allow common separators (hyphens, underscores).")
                seen.add("scope_fix")
            if "feature" in fl and "underscore" in fl and "feature_prefix" not in seen:
                recs.append("Consider changing default 'feature' branch prefix from 'feature_' to 'feature/' for Git-Flow familiarity.")
                seen.add("feature_prefix")

    return recs


# ─────────────────────────────────────────────────────────────────────────────
# CLI entry point
# ─────────────────────────────────────────────────────────────────────────────

def main():
    import argparse
    parser = argparse.ArgumentParser(
        description="Generate persona feedback reports from choreo test results",
    )
    parser.add_argument("--report", help="Path to a single choreo JSON report")
    parser.add_argument("--dir", default="reports/", help="Directory containing choreo JSON reports")
    parser.add_argument("--chor", default="tests/explorations/", help="Directory containing .chor source files")
    parser.add_argument("--output", default=None, help="Output directory for YAML reports (default: stdout)")
    parser.add_argument("--latest", action="store_true", help="Only process the N most recent reports (one per persona)")
    args = parser.parse_args()

    if args.report:
        report_files = [args.report]
    else:
        report_dir = Path(args.dir)
        if not report_dir.exists():
            print(f"Error: Report directory '{args.dir}' not found.", file=sys.stderr)
            sys.exit(1)
        report_files = sorted(report_dir.glob("*.json"), key=os.path.getmtime, reverse=True)

        if args.latest:
            # Keep only the most recent report per persona
            seen_personas = set()
            filtered = []
            for rf in report_files:
                with open(rf) as f:
                    try:
                        data = json.load(f)
                    except json.JSONDecodeError:
                        continue
                if not isinstance(data, list) or len(data) == 0:
                    continue
                persona = infer_persona(data[0].get("name", ""), data[0].get("uri", ""))
                if persona not in seen_personas:
                    seen_personas.add(persona)
                    filtered.append(rf)
            report_files = filtered

    for report_file in report_files:
        report_path = str(report_file)
        try:
            yaml_output = generate_report(report_path, args.chor)
        except (json.JSONDecodeError, KeyError, IndexError) as e:
            print(f"# Skipping {report_path}: {e}", file=sys.stderr)
            continue

        if args.output:
            out_dir = Path(args.output)
            out_dir.mkdir(parents=True, exist_ok=True)
            out_name = Path(report_path).stem.replace("choreo_test_report", "persona_feedback") + ".yml"
            out_path = out_dir / out_name
            with open(out_path, "w") as f:
                f.write(yaml_output)
            print(f"Generated: {out_path}", file=sys.stderr)
        else:
            print(yaml_output)
            print("---")  # YAML document separator


if __name__ == "__main__":
    main()

