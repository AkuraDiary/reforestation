# Reforestation

## What the hell is this?

Reforestation is a CLI tool that generates dummy GitHub commit history by automating commit creation over a specified period. This tool is useful for testing, demonstrations, or creating visually appealing GitHub contribution graphs. (Nah dude this thing will not solve any of your problems at all 💀💀)

## Features

- Generates commits over a specified number of days
- Configurable commit frequency per day
- Option to randomize commit frequency
- Supports custom repository URLs and target directories

## Installation

1. Download one of the releases:
   Go to https://github.com/ahsanzizan/reforestation/releases/download/1.0.0/reforestation.zip
2. Extact that thing out of the ZIP file.
3. Start using it:
   ```
   ./reforestation.exe --help
   ```

## Usage

Run the CLI with the following options:

```sh
./reforestation.exe --repo https://github.com/your-username/repo.git --days 30 --freq 5 --dir "../this-dir"
```

### Command-line Arguments

| Argument   | Description                                              |
| ---------- | -------------------------------------------------------- |
| `--days`   | Number of days to generate commits for (default: 366)    |
| `--freq`   | Number of commits per day (default: 10)                  |
| `--random` | Randomize commit frequency                               |
| `--repo`   | GitHub repository URL                                    |
| `--dir`    | Target directory for commits (default: `dummy-git-repo`) |

This could be wrong tho (because I often forgot to update the README), run the `--help` for more detailed information.

## License

This project is licensed under the MIT License. See the [`LICENSE`](https://github.com/ahsanzizan/reforestation/blob/main/LICENSE) file for details.

## Author

Ahsan Azizan, contact@ahsanzizan.xyz (Who else would made this type of shitty project on their weekend?)
