# Lapse

Http CLI focused on UX and git-heavy workflow

## Summary

Making requests from CLI can be very intimidating. I myself have tried some tools, and I always ended up falling back to Insomnia.

Lapse's design is built on top of files that will define the whole behavior.
Not only this simplifies the whole proccess of using it, but it also lets you use your favorite VCS when using it

## Features

- Http requests
- Multipart/form requests
- Environments and variables
- Secrets
- Lua scripts and interpolation

## The anatomy of a Lapse space

```
├── .lapse
├── env
│   └── default.json
├── requests
│   └── httpbin.md
├── scripts
│   └── test.lua
└── secrets.json
```

I recommend to ignore these entries in your VCS:

```.gitignore
.lapse/
secrets.json
```

### Request files

This is how a request file looks like

```md
POST httpbin.org/post
Content-Type: application/json

{
  "name": "${env.name}"
}

---
# Request documentation

Here is some valid markdown

```

Now, there is a lot to unpack here.

The first two lines are self explanatory. Any line that comes directly after the request is treated like header, so it has to follow this key:value syntax

After the blank like, we have our body. There is nothing special about the request body, it is just sent as raw text. The tool doesn't detect when you are sending json, so you are supposed to add the Content-Type header by yourself.

After the body, we can see a triple dash separating the request from the document. This second section is just raw text that you can use to add informations about your request. This second part is optional, so you can omit the triple dash and not include it at all.

Also, we do have this interpolated value (`${env.name}`), this is an env variable

These request files can be organized however you want, as long as they are inside of the `requests/` folder, you will be able to call them from their unique path.

### Env files

Env files are just json files with values. Any json valid value is accepted, even objects. You are not supposed to throw secrets into environments, we will get there yet.

```json
{
  "name": "John"
}
```

### Scripts

Scripts are just lua scripts. We will have more details about its API later.

### Secrets

Secrets is just like an env file

```json
{
  "password": "shhhh"
}
```

## Commands

## Scripts API
