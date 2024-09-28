local pickers = require("telescope.pickers")
local finders = require("telescope.finders")
local conf = require("telescope.config").values
local actions = require("telescope.actions")
local action_state = require("telescope.actions.state")

local locations = {}

local params = vim.lsp.util.make_position_params()
local results_lsp = {}
local results = {}
local timeout = 1000 -- ms

for _, client in pairs(vim.lsp.get_active_clients()) do
	if client.server_capabilities.definitionProvider then
		local request_result =
			client.request_sync("textDocument/definition", params, timeout, vim.api.nvim_get_current_buf())
		if request_result and request_result.result then
			table.insert(results_lsp, request_result.result)
		end
	end
end

for _, lsp_result in ipairs(results_lsp) do
	if lsp_result then
		vim.list_extend(results, lsp_result)
	end
end

for _, result in ipairs(results) do
	if result.targetUri then
		table.insert(locations, {
			filename = vim.uri_to_fname(result.targetUri),
			lnum = result.targetRange.start.line + 1,
			col = result.targetRange.start.character + 1,
			text = "Definition",
		})
	elseif result.uri then
		table.insert(locations, {
			filename = vim.uri_to_fname(result.uri),
			lnum = result.range.start.line + 1,
			col = result.range.start.character + 1,
			text = "Definition",
		})
	end
end

pickers
	.new({}, {
		prompt_title = "LSP Definitions",
		finder = finders.new_table({
			results = locations,
			entry_maker = function(entry)
				return {
					value = entry,
					display = entry.filename .. ":" .. entry.lnum .. ":" .. entry.col .. " " .. entry.text,
					ordinal = entry.filename .. " " .. entry.lnum .. " " .. entry.col .. " " .. entry.text,
					filename = entry.filename,
					lnum = entry.lnum,
					col = entry.col,
				}
			end,
		}),
		sorter = conf.generic_sorter({}),
		attach_mappings = function(prompt_bufnr, map)
			actions.select_default:replace(function()
				actions.close(prompt_bufnr)
				local selection = action_state.get_selected_entry()
				vim.api.nvim_command("edit " .. selection.filename)
				vim.api.nvim_win_set_cursor(0, { selection.lnum, selection.col - 1 })
			end)
			return true
		end,
	})
	:find()
