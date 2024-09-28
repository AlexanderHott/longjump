# Longjump, a cross-project, cross-language goto definition provider.

When using <https://trpc.io/>, I fell in love with the ability to goto definition on my client and be brought to the server code.
This currently doesn't exist for cross-language projects, as both languages use different LSPs.

This project aims to get goto definition working for cross language projects.

Here is an example

```tsx
// ...

const fetchClient = createFetchClient<paths>({
  baseUrl: "http://localhost:8080",
});

const $api = createClient(fetchClient);

function App() {
  const { mutate, data } = $api.useMutation("post", "/count");
  return (
    <div>
      <button onClick={() => mutate({ body: { newCount: 10 } })}>
            +10
      </button>
      <div>{data?.count}</div>
    </div>
  );
}

export default App;
```

If you press `gd` on the `$api.useMutation("post", "/count")`, you would be brought here.

```go
huma.Post(api, "/count", func(ctx context.Context, input *CountInput) (*CountOutput, error) {
    resp.Body.Count = 2*input.Body.NewCount
    return resp, nil
})
```

Instead of the generated openapi react query code.

# Implementation

The first implementation will likely be a bit janky. I plan to use regex instead of an AST because it will let me test out if this can even work.

# Development

If you want to help with the development, feel free to open up issues or PRs.

For neovim, this is how I currently have `longjump` registered as an LSP. I'm still new to this, so if there is a better way, feel free to open an issue.

```lua
	{
		"neovim/nvim-lspconfig",
        -- ...
		config = function()
            -- ...
			vim.lsp.set_log_level("DEBUG")
			local configs = require("lspconfig.configs")
			local lspconfig = require("lspconfig")
			if not configs.longjump then
				configs.longjump = {
					default_config = {
						cmd = { "/home/ott/Documents/code/personal/longjump/target/debug/longjump" },
						filetypes = { "javascript", "typescript", "javascriptreact", "typescriptreact", "go" },
						root_dir = lspconfig.util.root_pattern("package.json"),
						settings = {},
					},
				}
			end
			lspconfig.longjump.setup({})
	}
```

There is currently a bug where if you type `gd`, it jumps twice, instead of showing a menu. Maybe my LSP is configured wrong, or Neovim doesn't know how to handle two separate LSPs providing locations. I included the `lsp-mux.lua` file that asks for locations for goto definition, aggregates all the LSP results, and shows them in a telescope window. You can use it to simulate a `gd` by running `luafile lsp-mux.lua`.

Again, if you know what is going wrong here, help is appreciated.

# Big Plans

I'm trying to increase the DX of cross-language development, because it is not the best right now.
My two of my most used keybinds in vim are `]d` (go to next error) and `gd` go to definition. 

When using <https://trpc.io>, when you make changes in your backend, any errors in the front end get shown almost instantly.
This doesn't happen for me right now, and it doesn't seem like too hard of a problem to solve.

`]d` could work through live code-gen. I'm already using code-gen with <https://huma.rocks/> generating an openapi spec and <https://openapi-ts.dev/openapi-react-query/> creating a Tanstack Query client. However, on my live-reload server <https://github.com/air-verse/air> I can't figure out how to run the code-gen step after the program starts, as the openapi spec is only available when the server is running. Forking `air` will likely be one of my next steps.

The other keybind `gd` will hopefully be solved by this project.
