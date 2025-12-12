import os
import sys
import subprocess

RUST_TEMPLATE = """use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    
    let result = solve(&input.trim());
    println!("{}", result);
}

fn solve(input_data: &str) -> String {
    t.to_string()
}
"""

PYTHON_TEMPLATE = """def solve(input_data: str) -> str:
    return str(t)
"""

CARGO_TOML_TEMPLATE = """[package]
name = "aoc{year}"
version = "0.1.0"
edition = "2021"
"""


def create_structure(base_dir: str, year: int, num_days: int, lang: str):
    year_dir = os.path.join(base_dir, str(year))

    if lang == "rs":
        if not os.path.exists(year_dir):
            os.makedirs(year_dir, exist_ok=True)
            print(f"Created year folder: {year_dir}")

        src_dir = os.path.join(year_dir, "src")
        os.makedirs(src_dir, exist_ok=True)

        cargo_toml_path = os.path.join(year_dir, "Cargo.toml")
        if not os.path.exists(cargo_toml_path):
            with open(cargo_toml_path, "w") as f:
                f.write(CARGO_TOML_TEMPLATE.format(year=year))
            print(f"Created Cargo.toml: {cargo_toml_path}")

        cargo_toml_content = CARGO_TOML_TEMPLATE.format(year=year)

        for day in range(1, num_days + 1):
            day_dir = os.path.join(src_dir, str(day))
            os.makedirs(day_dir, exist_ok=True)

            for part in [1, 2]:
                file_path = os.path.join(day_dir, f"{part}.rs")
                if not os.path.exists(file_path):
                    with open(file_path, "w") as f:
                        f.write(RUST_TEMPLATE)
                    print(f"Created file: {file_path}")

                cargo_toml_content += (
                    f'\n[[bin]]\nname = "{day}_{part}"\npath = "src/{day}/{part}.rs"\n'
                )

        with open(cargo_toml_path, "w") as f:
            f.write(cargo_toml_content)
        print(f"Updated Cargo.toml with all binary targets")

    elif lang == "py":
        if not os.path.exists(year_dir):
            os.makedirs(year_dir, exist_ok=True)

        for day in range(1, num_days + 1):
            day_dir = os.path.join(year_dir, str(day))
            os.makedirs(day_dir, exist_ok=True)

            for part in [1, 2]:
                file_path = os.path.join(day_dir, f"{part}.py")
                if not os.path.exists(file_path):
                    with open(file_path, "w") as f:
                        f.write(PYTHON_TEMPLATE)
                    print(f"Created file: {file_path}")
                else:
                    print(f"File already exists: {file_path}")

    else:
        print("Unsupported language. Only 'py' and 'rs' are allowed.")
        return


if __name__ == "__main__":
    if len(sys.argv) < 3:
        print(
            "Usage: python create_structure.py <year> <language> <num_days> <base_dir>"
        )
        sys.exit(1)

    base_dir = os.getcwd()
    try:
        year_input = int(sys.argv[1])
    except ValueError:
        print("Year must be an integer")
        sys.exit(1)

    lang_input = sys.argv[2].lower()
    if lang_input not in ["py", "rs"]:
        print("Language must be 'py' or 'rs'")
        sys.exit(1)

    num_days = int(sys.argv[3]) if len(sys.argv) >= 4 else 25

    if len(sys.argv) >= 5:
        base_dir = sys.argv[4]
        if not os.path.exists(base_dir):
            print(f"Base directory does not exist: {base_dir}")
            sys.exit(1)

    create_structure(base_dir, year_input, num_days, lang_input)
