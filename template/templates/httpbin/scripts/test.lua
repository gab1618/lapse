local request = Lapse:get_request("get")
local result = Lapse:request(request)
print(result.status)
print(result.text)
print(result.resolved_request)

for k, v in pairs(result.headers) do
  print(k, v)
end
