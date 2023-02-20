# Licenser
A small tool to add a license notices

Note:
The program will ignore all files with unknown file extensions

## Usage
```bash
licenser [OPTIONS] --directory <DIRECTORY> --license <LICENSE>
```

## Options
  -d, --directory \<DIRECTORY> &nbsp;&nbsp;&nbsp;Specifies the directory to add license notices to

  -l, --license \<LICENSE> &nbsp;&nbsp;&nbsp;Specifies the file containing the license notice

  -e, --extensions \<EXTENSIONS> &nbsp;&nbsp;&nbsp; Specifies what file extensions to license

  -D, --dry-run &nbsp;&nbsp;&nbsp; Performs a dry run, showing files that would be affected

  -s, --silent &nbsp;&nbsp;&nbsp;Prints only the number of modified files

  -c, --comment &nbsp;&nbsp;&nbsp;Automatically add comments
  
  -r, --replace &nbsp;&nbsp;&nbsp;Replaces existing license notices with new ones

  -h, --help  &nbsp;&nbsp;&nbsp;Print help

  -V, --version  &nbsp;&nbsp;&nbsp;Print version