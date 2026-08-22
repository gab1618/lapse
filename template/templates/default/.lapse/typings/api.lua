---@meta

---@class ExecutionResult
---@field text string
---@field status number
---@field headers table
---@field timestamp number
---@field duration number
---@field resolved_request string
local ExecutionResult = {}

---@class Lapse
local Lapse = {}

---@param name string
---@return ExecutionResult
function Lapse:request(name) end
