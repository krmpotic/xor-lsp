# xor-lsp
Minimal assembly language LSP server that displays information directly from a parsed
[Intel® 64 and IA-32 Architectures Software Developer’s Manual](www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).

[A fork of golang/arch](github.com/krmpotic/arch/tree/master/x86/x86spec) is used to extract information from the PDF into the `misc/x86spec.csv` file.

## Warning
This project is a work in progress. Not suitable for use yet!
Released early for my hacker friends!

If you want to try it out, it only works using `cargo run` from the root directory, since it hard-codes the path to `misc/x86spec.csv`.

## Functionality
LSP hover request displays description of assembly instruction from the Intel's manual.

## Neovim configuration
Example neovim init.lua config:
```lua
local xor_cfg = {
	name = 'xor-test',
	cmd = { 'xor-lsp/target/debug/xor-lsp' }, -- change this line to absolute path
	root_dir = vim.fn.getcwd(),
}

local client_id = vim.lsp.start_client(xor_cfg)
vim.lsp.buf_attach_client(1, client_id)
vim.keymap.set('n', 'K', vim.lsp.buf.hover, { }) -- map 'K' to hover request
```

Note on neovim, if `:LspInfo` reports *not* attached to buffer,
use `:ls` to get buffer number, and `:LspInfo` to get client id,
and attach to buffer manually using `:lua vim.lsp.buf_attach_client(<bufnr>, <client_id>)`

## Credits
code from example LSP server: rust-analyzer/lib/lsp-server/examples/goto_def.rs
code from asm-lsp: github.com/bergercookie/asm-lsp  
parser of Intel's manual: golang.org/x/arch/x86/x86spec
