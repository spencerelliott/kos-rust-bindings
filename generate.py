import re
import argparse

from pathlib import Path

# The default template for our generated file
FILE_TEMPLATE = """\
#![no_std]

"""

# The start of the mod definition
MOD_TEMPLATE = "pub mod {name} {{"

# This is used to extract "mod" definitions from lib.rs for generation purposes
MOD_REGEX = re.compile("^(pub mod|mod) (.*);$")

if __name__ == '__main__':
    arg_parser = argparse.ArgumentParser(description='Generates a monolithic kos.rs file for use in KallistiOS projects')
    arg_parser.add_argument('-d', '--parse-dir', default='src/', required=False, help='The directory where files should be parsed')
    arg_parser.add_argument('-o', '--out-dir', default='', required=False, help="The output directory of the kos.rs file")
    args = arg_parser.parse_args()

    parse_dir: str = args.parse_dir
    if len(parse_dir) > 0 and not parse_dir.endswith('/'):
        parse_dir = parse_dir + '/'

    out_dir: str = args.out_dir
    if len(out_dir) > 0 and not out_dir.endswith('/'):
        out_dir = out_dir + '/'

    final_str = FILE_TEMPLATE

    # Read the lib.rs file. This should only have "mod" definitions inside of it since this script doesn't extract
    # any actual data from the file.
    with open(f'{parse_dir}lib.rs', 'r') as lib_f:
        for line in lib_f.readlines():
            # Check for a mod definition
            mod_match = MOD_REGEX.match(line)

            # If we've found a match, start to process the file
            if mod_match:
                # Grab the name of the mod
                mod_name = mod_match.group(2)
                print(f"Found mod: {mod_name}!")

                # Add the mod template to the existing file string
                final_str = final_str + MOD_TEMPLATE.format(name=mod_name)
                with open(f'{parse_dir}{mod_name}.rs', 'r') as mod_f:
                    for line in mod_f.readlines():
                        # Ignore any "use" statements inside of the file
                        if line.startswith('use'):
                            continue
                        # Add the line to the generated file while indenting correctly
                        final_str = final_str + '\t' + line
                # Close off the mod definition
                final_str = final_str + '\n}\n\n'

    print(f"Writing library file to {out_dir}kos.rs...")

    # Create the folder(s), if needed
    Path(out_dir).mkdir(parents=True, exist_ok=True)

    with open(f'{out_dir}kos.rs', 'w') as kos_f:
        # Write the final file to the desired location
        kos_f.write(final_str)