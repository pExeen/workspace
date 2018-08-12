# workspace
> a CLI that takes care of your windows, tabs and working directory

## Installation

- Get the latest binary for your platform from the [releases](https://github.com/matthias-t/workspace/releases) page
- Setup the `ws` command in your shell:
  - **bash**: Add this line to your `.bashrc`
    ```bash
    eval $(workspace shell bash)
    ```
  - **fish**: Add this line to your `config.fish`
    ```fish
    workspace shell fish | source
    ```
  - **PowerShell**: Add this line to your `profile.ps1`
    ```powershell
    Invoke-Expression "$(workspace shell posh)"
    ```
  
## IAQ &nbsp;<sub><sup>(Infrequently asked questions)</sup></sub>

> Should I use `workspace` or `ws`?

Use `ws`. `workspace` is a binary that powers the `ws` function and sets it up in your shell configuration.

> Why do I need to add something to my shell configuration?

Otherwise workspace can't change your working directory or run commands that you specify for a workspace directly in the shell process.

> I don't trust you

That's not technically a question. But the good thing is: you don't need to. If you run `workspace shell ...` you can see what you are invoking. You can look at the code, it's all here. The binaries are built and uploaded by Travis, not me — 
see [`.travis.yml`](https://github.com/matthias-t/workspace/blob/master/.travis.yml) and check out 
[the builds](https://travis-ci.com/matthias-t/workspace).
