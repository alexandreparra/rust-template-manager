from pathlib import Path

import os.path
import random
import subprocess
import sys
import time


def test(rtm_path: str = None):
    rtm = rtm_path if rtm_path is not None else "rtm"

    # 1. Match 'rtm folder' and TEMPLATES XDG folder
    xdg_folders = get_xdg_templates()
    if len(xdg_folders) == 0:
        print("System doesn't have a default template folder, rtm will fallback to $HOME/Templates")
    xdg_folder = xdg_folders[0].decode("utf-8").strip()
    rtm_folder = get_folder(rtm)[0].decode("utf-8").split(':')[1].strip().replace('"', '')

    if rtm_folder != xdg_folder:
        sys.exit(f"Failed test at rtm folder command, \nexpected: {xdg_folder} \nactual: {rtm_folder}")
    else:
        print("OK 'rtm folder' matches XDG folder")

    # 2. Create any file
    random.seed()
    file_name = f"{random.randint(0, 1000)}.test_txt"
    create_file(rtm, file_name)

    # 3. List the files inside the templates folder and find the created file
    files = clean_file_list(list_files(rtm))

    if file_name in files:
        print(f"OK file {file_name} created with 'rtm create' is listed inside 'rtm list'")
    else:
        sys.exit(f"File {file_name} create with 'rtm create' wasn't listed with 'rtm list'")

    # 4. Delete the file
    delete_files(rtm, file_name)

    # 5. List the file again and make sure its gone
    files2 = clean_file_list(list_files(rtm))
    if file_name not in files2:
        print(f"OK file {file_name} was deleted successfully with 'rtm delete'")
    else:
        sys.exit(f"File {file_name} wasn't deleted with 'rtm delete'")

    # 6. copy a file and make sure it exists where it was supposed to be copied
    copy_file_name = "temp_test.temp_txt"
    create_file(rtm, copy_file_name)
    copy_file(rtm, copy_file_name)

    print("Waiting for the file to be moved")
    # For some reason the file takes a little bit to be copied, so we wait a bit.
    time.sleep(2)

    tmp_path = Path("/tmp")
    tmp_files = [f.name for f in tmp_path.iterdir() if f.is_file()]
    if copy_file_name in tmp_files:
        print(f"OK found the copied file {copy_file_name} inside using 'rtm copy'")
    else:
        sys.exit(f"File {copy_file_name} wasn't found inside /tmp")

    os.remove(f"/tmp/{copy_file_name}")


def clean_file_list(files: list[bytes]) -> list[str]:
    return list(map(lambda f: f.decode("utf-8").strip().replace('\n', ''), files))


def delete_files(rtm_path: str, files: str):
    exec_rtm_command(rtm_path, f"delete {files}")


def list_files(rtm_path: str) -> list[bytes]:
    return exec_rtm_command(rtm_path, "list")


def get_folder(rtm_path: str) -> list[bytes]:
    return exec_rtm_command(rtm_path, "folder")


def create_file(rtm_path: str, file_name: str):
    exec_rtm_command(rtm_path, f"create {file_name} -ne")


def copy_file(rtm_path: str, file_name: str):
    subprocess.Popen(f"cd /tmp && {rtm_path} copy {file_name}", shell=True, stdout=subprocess.PIPE)


def exec_rtm_command(rtm_path: str, subcommand: str) -> list[bytes]:
    return subprocess.Popen(f"{rtm_path} {subcommand}", shell=True, stdout=subprocess.PIPE).stdout.readlines()


def get_xdg_templates() -> list[bytes]:
    return subprocess.Popen("xdg-user-dir TEMPLATES", shell=True, stdout=subprocess.PIPE).stdout.readlines()


if __name__ == "__main__":
    args = sys.argv
    if len(args) >= 2:
        if os.path.exists(args[1]) is False:
            sys.exit(f"Provided path: {args[1]} doesn't exist")
        test(args[1])
    else:
        test()
