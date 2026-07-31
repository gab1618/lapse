MULTIPART https://example.com/comments
content-type: multipart/form-data

name: "${env.name}"
ex-file: @./env.json

---

# Sample multipart/form request

This expression is a lua expression that calls this function var, passing the value `name`

That content-type header is redundant, but is doesn't cause issues.
