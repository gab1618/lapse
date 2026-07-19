POST https://example.com/comments
content-type: application/json

{
  "password": "${secret.password}"
}

---

# Sample request with expressions inside

This expression is a lua expression that calls this function var, passing the value `name`
