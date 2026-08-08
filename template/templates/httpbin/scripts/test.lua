local result = lapse:request("get")
print(result.status)
print(result.text)
print(result.request)

for k, v in pairs(result.headers) do
  print(k, v)
end
