local M = {}

local MODE_RWX = 7 * 64 + 7 * 8 + 7

local target_path = vim.fn.stdpath("data") .. "/nasin"

function M.build()
    pcall(vim.uv.fs_mkdir, target_path, MODE_RWX)
    pcall(vim.uv.fs_mkdir, target_path .. "/parser", MODE_RWX)
    pcall(vim.uv.fs_mkdir, target_path .. "/queries", MODE_RWX)
    pcall(vim.uv.fs_mkdir, target_path .. "/queries/nasin", MODE_RWX)

    -- FIXME: clone from git
    local ts_nasin_path = vim.env.HOME .. "/Projects/lang/tree-sitter-nasin"
    for _, ext in ipairs({ "so", "dll" }) do
        pcall(
            vim.uv.fs_copyfile,
            ts_nasin_path .. "/nasin." .. ext,
            target_path .. "/parser/nasin." .. ext
        )
    end
    for _, query in ipairs({ "highlights.scm" }) do
        pcall(
            vim.uv.fs_copyfile,
            ts_nasin_path .. "/queries/" .. query,
            target_path .. "/queries/nasin/" .. query
        )
    end
end

local augroup = vim.api.nvim_create_augroup("nasin", { clear = true })

function M.setup()
    vim.opt.rtp:prepend(target_path)

    vim.api.nvim_create_autocmd({ "BufEnter", "BufNewFile" }, {
        pattern = "*.nsn",
        group = augroup,
        callback = function()
            vim.bo.filetype = "nasin"
            vim.treesitter.start()
        end,
    })
end

return M
