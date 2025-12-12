import importlib.util
import os
import re
import sys
import subprocess
import requests
from colorama import Fore, init
from dotenv import load_dotenv

init(autoreset=True)
load_dotenv()

SESSION_COOKIE = os.getenv("SESSION_COOKIE")

PY_BASE_DIR = "py"
RS_BASE_DIR = "rs"


def fetch_input(year: int, day: int) -> str:
    url = f"https://adventofcode.com/{year}/day/{day}/input"
    cookies = {"session": SESSION_COOKIE}
    response = requests.get(url, cookies=cookies)
    response.raise_for_status()
    return response.text


def submit_answer(year: int, day: int, level: int, answer: str) -> str:
    url = f"https://adventofcode.com/{year}/day/{day}/answer"
    cookies = {"session": SESSION_COOKIE}
    data = {"level": str(level), "answer": answer}
    response = requests.post(url, cookies=cookies, data=data)
    response.raise_for_status()
    return response.text


def run_solution(year: int, day: int, part: int, lang: str, input_data: str) -> str:
    lang = lang.lower()
    if lang == "py":
        year_dir = os.path.join(os.getcwd(), PY_BASE_DIR, str(year))
        file_path = os.path.join(year_dir, str(day), f"{part}.py")
        if not os.path.exists(file_path):
            raise FileNotFoundError(f"Python file not found: {file_path}")
        module_name = f"{year}_{day}_{part}"
        spec = importlib.util.spec_from_file_location(module_name, file_path)
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        return module.solve(input_data)

    elif lang == "rs":
        year_dir = os.path.join(os.getcwd(), RS_BASE_DIR, str(year))
        bin_name = f"{day}_{part}"
        try:
            result = subprocess.run(
                ["cargo", "run", "--quiet", "--bin", bin_name],
                cwd=year_dir,
                input=input_data.encode(),
                capture_output=True,
                check=True,
            )
        except subprocess.CalledProcessError as e:
            raise RuntimeError(
                f"Error running Rust solution via Cargo:\nstdout={e.stdout.decode()}\nstderr={e.stderr.decode()}"
            )
        return result.stdout.decode().strip()
    else:
        raise ValueError("Unsupported language. Only 'py' or 'rs' are allowed.")


def print_result_summary(response_html: str, day: int, part: int):
    if "That's the right answer" in response_html:
        print(Fore.GREEN + f"Correct answer for {day}/{part}")
    elif "That's not the right answer" in response_html:
        print(Fore.RED + f"Incorrect answer for {day}/{part}")
    elif "You gave an answer too recently" in response_html:
        wait_match = re.search(
            r"You have ((?:\d+m\s*)?(?:\d+s)?) left to wait", response_html
        )
        wait_time = wait_match.group(1) if wait_match else "unknown"
        print(
            Fore.YELLOW
            + f"Rate limited — wait {wait_time} before retrying {day}/{part}"
        )
    elif "You have completed both parts" in response_html:
        print(Fore.GREEN + f"Already completed both parts for {day}/{part}")
    elif "Did you already complete it?" in response_html:
        print(Fore.YELLOW + f"{day}/{part} is completed")
    else:
        print(Fore.YELLOW + "Unrecognized response. Dumping raw content:\n")
        print(response_html)


def main():
    lang = "py"
    if len(sys.argv) < 3:
        print("Usage: python main.py <day> <part> <year> <lang>")
        sys.exit(1)

    day = int(sys.argv[1])
    part = int(sys.argv[2])
    year = int(sys.argv[3]) if len(sys.argv) >= 4 else 2025
    if len(sys.argv) >= 5:
        lang = sys.argv[4].lower()
        if lang not in ["py", "rs"]:
            print("Language must be 'py' or 'rs'")
            sys.exit(1)

    input_data = fetch_input(year, day)
    answer = run_solution(year, day, part, lang, input_data)

    print(f"Answer computed: {Fore.BLUE + answer}")
    if not answer.strip():
        print(Fore.RED + "No answer produced. Skipping submission.")
        sys.exit(1)

    response = submit_answer(year, day, part, answer)
    print_result_summary(response, day, part)


if __name__ == "__main__":
    main()
