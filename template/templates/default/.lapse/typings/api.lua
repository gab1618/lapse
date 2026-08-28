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

---@param request string
---@return ExecutionResult
function Lapse:request(request) end

---@param name string
---@return string
function Lapse:get_request(name) end
