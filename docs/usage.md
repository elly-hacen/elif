
## Basic Command Structure

```
elif <COMMAND> [OPTIONS] [ARGS]
```

---

## Available Commands

| Command | Description                 |
|---------|-----------------------------|
| enc     | Encrypt or decrypt a file via `openssl`  |
| record  | Record audio or video via `ffmpeg`     |
| help    | Show help message or help for a subcommand |

---

## Command Details

### 1. Encrypt or Decrypt a File (`enc`)

Encrypt or decrypt files using a password.

```

Usage: elif enc [OPTIONS] <FILE> <PASSWORD>

Arguments: 
<FILE> The file path 
<PASSWORD> The password to use

Options:
-d, --decrypt  Decrypt a file
-h, --help     Print help

```

---

### 2. Record Audio or Video (`record`)

Record audio or video and save it to a file.

```

Usage: elif record <MEDIA_TYPE> <OUTPUT>

Arguments:
<MEDIA_TYPE> video or audio [possible values: video, audio] 
<OUTPUT> Output file name

Options:
-h, --help  Print help

```
