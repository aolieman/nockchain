from collections import defaultdict
from enum import StrEnum
from pathlib import Path
import re
import sys

ROOT = Path(__file__).parent.resolve()
HOON_DIRS = [
    ROOT / "hoon",
    ROOT / "crates" / "nockvm" / "hoon",
    ROOT / "crates" / "hoonc" / "hoon",
]
RUST_DIRS = [
    ROOT / "crates" / "zkvm-jetpack" / "src",
    ROOT / "crates" / "nockvm" / "rust" / "nockvm" / "src" / "jets",
]
HOT_FILES = ["hot.rs"]


class EnvContext(StrEnum):
    HOON = "hoon"
    ZKVM = "zkvm"
    WUTF = "wutf"


# Data structure: maps a key (jet label) to all observed attributes
class JetInfo:
    def __init__(self):
        self.paths = set()  # From HotEntry paths
        self.rust_fns = set()  # From Rust function names
        self.hinted = False  # From Hoon hints
        self.context: set[EnvContext] = set()

    def __repr__(self):
        return f"JetInfo(paths={list(self.paths)}, rust_fns={list(self.rust_fns)}, hinted={self.hinted}, context={list(self.context)})"


# ---- HELPERS ----


def extract_hoon_jet_hints(hoon_dirs, jets_by_label):
    hint_pattern = re.compile(r"^\s*(~%|~/)\s*%([\w\-]+)", re.IGNORECASE)
    for hoon_dir in hoon_dirs:
        for hoon_file in hoon_dir.rglob("*.hoon"):
            context = EnvContext.ZKVM
            if "crates" in str(hoon_file):
                context = EnvContext.HOON

            indent_lvl = 0
            hint_path: list[str] = []
            for line in hoon_file.read_text(
                encoding="utf-8", errors="ignore"
            ).splitlines():
                match = hint_pattern.match(line)
                if match:
                    line_indent_lvl = match.start(1)
                    sigrune, tas = match.groups()
                    if sigrune == "~%":
                        hint_path = [tas]
                    else:
                        # Same-level hint: replace last
                        if line_indent_lvl == indent_lvl:
                            hint_path[-1] = tas
                        elif line_indent_lvl < indent_lvl:
                            # Going up
                            depth = line_indent_lvl // 2 + 1
                            hint_path = hint_path[:depth] + [tas]
                        else:
                            # Going deeper
                            hint_path.append(tas)

                    hint_tuple = tuple(hint_path)
                    jets_by_label[hint_tuple].hinted = True
                    jets_by_label[hint_tuple].context.add(context)
                    indent_lvl = line_indent_lvl


def extract_rust_jet_functions(rust_dirs, jets_by_label):
    fn_pattern = re.compile(r"\s*fn\s+(?:(\w+)_jet|jet_(\w+))")
    for rust_dir in rust_dirs:
        for rust_file in rust_dir.rglob("*.rs"):
            if "zkvm-jetpack" in str(rust_file):
                context = EnvContext.ZKVM
            elif "nockvm" in str(rust_file):
                context = EnvContext.HOON
            else:
                context = EnvContext.WUTF

            for line in rust_file.read_text(
                encoding="utf-8", errors="ignore"
            ).splitlines():
                match = fn_pattern.search(line)
                if match:
                    label, fn_name = find_jet_label(match, jets_by_label)
                    if fn_name in (
                        "assert_jet",
                        "jet_mure",
                        "jet_mute",
                        "find_jet",
                        "jet_err",
                    ):
                        continue

                    jets_by_label[label].rust_fns.add(fn_name)
                    jets_by_label[label].context.add(context)
                    if context == EnvContext.WUTF:
                        print(
                            f"Unexpected jet {fn_name} in {rust_file}", file=sys.stderr
                        )
                        jets_by_label[label].paths.add(str(rust_file.relative_to(ROOT)))


def find_jet_label(
    fn_match: re.Match[str], jets_by_label: dict[tuple[str], JetInfo]
) -> tuple[tuple[str], str]:
    fn = (fn_match.group(1) or fn_match.group(2)).lower()
    name = fn_match.group(0).strip().split()[1]
    tokens = tuple(fn.split("_"))
    kebab = "-".join(tokens)
    best_fuzzy: tuple[str] = tuple()
    for label in jets_by_label.keys():
        if label[-1] == kebab:
            # e.g. bpoly-to-list
            return label, name
        elif label[-len(tokens) :] == tokens:
            # e.g. siva/de
            return label, name
        else:
            # e.g. zeke/ave/transpose vs mary_transpose
            isect = set(label) & set(tokens)
            if len(isect) > len(best_fuzzy):
                best_fuzzy = label

    if best_fuzzy:
        return best_fuzzy, name

    return (kebab,), name


def extract_hotentry_paths(rust_dirs, jets_by_label):
    entry_pattern = re.compile(r"&\[(.*?)\]\s*,\s*\d+\s*,\s*([\w\d_]+)", re.DOTALL)
    # entry_pattern = re.compile(r"&\[(.*?)\],\s*\d+,\s*(\w+_jet)", re.DOTALL)
    left_pattern = re.compile(r'Left\(b"([\w\-]+)"\)')

    for rust_dir in rust_dirs:
        for rust_file in rust_dir.rglob("*.rs"):
            if rust_file.name not in HOT_FILES:
                continue

            content = rust_file.read_text(encoding="utf-8", errors="ignore")
            for entry_match in entry_pattern.finditer(content):
                path_blob, rust_fn = entry_match.groups()
                hot_path = left_pattern.findall(path_blob)
                if not hot_path:
                    continue

                best_match_len = 0
                best_label = tuple()
                for label in jets_by_label.keys():
                    # compare hot_path items with label items in order
                    common_label = [item for item in label if item in hot_path]
                    common_hot = [item for item in hot_path if item in label]
                    if common_label == common_hot:
                        isect_size = len(common_hot)
                        if isect_size > best_match_len or (
                            label[-1] in rust_fn and isect_size == best_match_len
                        ):
                            best_match_len = len(common_hot)
                            best_label = label

                final = best_label or tuple(hot_path[-3:])
                jets_by_label[final].paths.add("/".join(hot_path))
                jets_by_label[final].rust_fns.add(rust_fn)
                if "zeke" in hot_path:
                    jets_by_label[final].context.add(EnvContext.ZKVM)
                else:
                    jets_by_label[final].context.add(EnvContext.HOON)


def prune_hint_only_parents(jets_by_label: dict[tuple[str], JetInfo]):
    to_remove = set()
    keys = list(jets_by_label.keys())
    for key in keys:
        info = jets_by_label[key]
        if info.hinted and not info.paths and not info.rust_fns:
            has_children = any(
                other != key and other[: len(key)] == key for other in keys
            )
            if has_children:
                to_remove.add(key)

    for key in to_remove:
        del jets_by_label[key]


# ---- MAIN ----


def main():
    jets_by_label = defaultdict(JetInfo)

    extract_hoon_jet_hints(HOON_DIRS, jets_by_label)
    extract_rust_jet_functions(RUST_DIRS, jets_by_label)
    extract_hotentry_paths(RUST_DIRS, jets_by_label)
    prune_hint_only_parents(jets_by_label)

    def summarize(context_name):
        context_keys = [
            k for k, v in jets_by_label.items() if context_name in v.context
        ]

        hints_not_rust = sorted(
            k
            for k in context_keys
            if jets_by_label[k].hinted and not jets_by_label[k].rust_fns
        )
        rust_not_hints = sorted(
            k
            for k in context_keys
            if jets_by_label[k].rust_fns and not jets_by_label[k].hinted
        )
        hot_without_hints = sorted(
            k
            for k in context_keys
            if jets_by_label[k].paths and not jets_by_label[k].hinted
        )
        rust_without_hot = sorted(
            k
            for k in context_keys
            if jets_by_label[k].rust_fns and not jets_by_label[k].paths
        )

        def print_keys_and_values(keys, title):
            print(f"{title}: {len(keys)}")
            for k in keys:
                print(f"  {k}\n  {jets_by_label[k]}")
            print()

        print(f"\n[{context_name.upper()}]")
        print_keys_and_values(hints_not_rust, "Hoon hints not in Rust functions")
        print_keys_and_values(rust_not_hints, "Rust functions not in Hoon hints")
        print_keys_and_values(rust_without_hot, "Rust functions not in HotEntry")
        print_keys_and_values(hot_without_hints, "HotEntry labels not in Hoon hints")

    summarize(EnvContext.HOON)
    summarize(EnvContext.ZKVM)
    summarize(EnvContext.WUTF)


if __name__ == "__main__":
    main()
