# Lapse

![Logo](./assets/logo-white.svg)

[![License: GPL v3+](https://img.shields.io/badge/License-GPL%20v3%2B-blue.svg)](https://www.gnu.org/licenses/gpl-3.0.html)

Http CLI focused on UX and git-heavy workflow

## Summary

Making requests from CLI can be very intimidating. I myself have tried some tools, and I always ended up falling back to Insomnia.

Lapse's design is built on top of files that will define the whole behavior.
Not only this simplifies the whole proccess of using it, but it also lets you use your favorite tools.

## Features

- Http requests
- Multipart/form requests
- Environments and variables
- Secrets
- Lua scripts and interpolation
- Inline requests

## The anatomy of a Lapse space

```
├── .lapse
├── env
│   ├── hooks.json
│   ├── secrets.json
│   ├── config.json
│   ├── variables.json
│   └── other-env
│       ├── hooks.json
│       ├── secrets.json
│       ├── config.json
│       └── variables.json
├── requests
│   └── httpbin.md
└── scripts
    └── test.lua
```

### Request files

This is how a request file looks like

```md
POST httpbin.org/post
Content-Type: application/json

{
  "name": "${Env.name}"
}

---
# Request documentation

Here is some valid markdown

```

Now, there is a lot to unpack here.

The first two lines are self explanatory. Any line that comes directly after the request is treated like a header, so it has to follow this key:value syntax

After the blank like, we have our body. There is nothing special about the request body, it is just sent as raw text. The tool doesn't detect when you are sending json, so you are supposed to add the Content-Type header by yourself.

After the body, we can see a triple dash separating the request from the document. This second section is just raw text that you can use to add informations about your request. This second part is optional, so you can omit the triple dash and not include it at all.

Also, we do have this interpolated value (`${Env.name}`), this is an env variable

These request files can be organized however you want, as long as they are inside of the `requests/` folder, you will be able to call them from their unique path.

### Env files

Env variable files are just json files with values. Any json valid value is accepted, even objects. You are not supposed to throw secrets into environments, we will get there yet.

```json
{
  "name": "John"
}
```

Same goes for secrets.

This is what a hooks file looks like

```json
{
  "pre-request": {
    "enabled": true,
    "scripts": ["pre.lua"]
  },
  "post-request": {
    "enabled": false,
    "scripts": ["post.lua"]
  }
}
```

The keys are the event names. The scripts path is relative to the `scripts` folder, and the scripts are executed in the order they are passed to the array.

### Scripts

Scripts are just lua scripts. We will have more details about its API later.

### Secrets

Secrets are just like env variables, but they are not supposed to be tracked by VCS

```json
{
  "password": "shhhh"
}
```

## Commands

### Inline requests

```bash
lapse GET localhost:3000
```

Every command that is not part of the CLI commands is treated as an inline request, which follows the same syntax as normal requests.

#### Passing headers

```bash
lapse GET localhost:3000 Content-Type:application/json
```

#### Passing json body values

```bash
lapse GET localhost:3000 name==John Doe age=:32
```

#### Passing multipart/form values

```bash
lapse POST localhost:3000 name@=John Doe pic@@avatar.png
```

### Init

Initializes Lapse space at current dir, setting up some files and directories as well.

```shell
lapse init
```

### Ls

Lists all requests.

```shell
lapse ls
```

### Send

Sends a request. The query argument will be used to fuzzy search for the request. Should the search match more than one request, the user will be prompted a selector, which is also the behavior for when the query argument is omited.

```shell
lapse send [query]
```

The way it searches, it uses the request path as a string to query. So if we have something like:

```
├── .lapse
├── env
│   └── default.json
├── requests
│   ├── httpbin.md
│   └── httpbin
│       └── get.md
```

The searchable entries would be:

```
"httpbin"
"httpbin/get"
```

So, a query like "hg" would match the second entry.

### Run

Runs a script.

```shell
lapse run [query]
```

You can also use `script run`:

```bash
lapse script run [query]
```

### Script list

Lists all scripts.

```shell
lapse script ls
```

### Completion

Outputs the completion script to a given shell.

```shell
lapse completion <shell>
```

### Env ls

Lists all environments.

```shell
lapse env ls
```

### Env switch

Switches to an enviromnent.

```shell
lapse env switch [query]
```

### Log

Lists the latest requests. If a index is given, it lists detailed info about the log

```shell
lapse log [index]
```

## Evaluation API

### Env table

If your env looks like this:

```json
{
  "name": "John"
}
```

You can access values like this:

```
POST https://names.com/${Env.name}
```

### Secrets table

If your secrets.json file looks like this

```json
{
  "password": "Shhhh"
}
```

You can access values like this:

```
POST https://auth.com/${Secret.password}
```

## Scripts API

### Request

Sends a request, and returns a table that represents a response log.

```lua
local result = Lapse:request("httpbin/get")
print(result.status)
print(result.text)
print(result.request)

for k, v in pairs(result.headers) do
  print(k, v)
end
```

## License

This project is licensed under the GNU General Public License
version 3 or later (GPL-3.0-or-later).

See [LICENSE](LICENSE) for the full license text.
