type ProcessSpawnOptionsStdio = "inherit" | "default"

--[=[
	@type ProcessSpawnOptions
	@within Process

	A dictionary of options for `process.spawn`, with the following available values:

	* `cwd` - The current working directory for the process
	* `env` - Extra environment variables to give to the process
	* `shell` - Whether to run in a shell or not - set to `true` to run using the default shell, or a string to run using a specific shell
	* `stdio` - How to treat output and error streams from the child process - set to "inherit" to pass output and error streams to the current process
]=]
export type ProcessSpawnOptions = {
	cwd: string?,
	env: { [string]: string }?,
	shell: (boolean | string)?,
	stdio: ProcessSpawnOptionsStdio?,
}

--[=[
	@type ProcessSpawnResult
	@within Process

	Result type for child processes in `process.spawn`.

	This is a dictionary containing the following values:

	* `ok` - If the child process exited successfully or not, meaning the exit code was zero or not set
	* `code` - The exit code set by the child process, or 0 if one was not set
	* `stdout` - The full contents written to stdout by the child process, or an empty string if nothing was written
	* `stderr` - The full contents written to stderr by the child process, or an empty string if nothing was written
]=]
export type ProcessSpawnResult = {
	ok: boolean,
	code: number,
	stdout: string,
	stderr: string,
}

--[=[
	@class Process

	Built-in functions for the current process & child processes

	### Example usage

	```lua
	local process = require("@lune/process")

	-- Getting the arguments passed to the Lune script
	for index, arg in process.args do
		print("Process argument #" .. tostring(index) .. ": " .. arg)
	end

	-- Getting the currently available environment variables
	local PORT: string? = process.env.PORT
	local HOME: string? = process.env.HOME
	for name, value in process.env do
		print("Environment variable " .. name .. " is set to " .. value)
	end

	-- Getting the current os and processor architecture
	print("Running " .. process.os .. " on " .. process.arch .. "!")

	-- Spawning a child process
	local result = process.spawn("program", {
		"cli argument",
		"other cli argument"
	})
	if result.ok then
		print(result.stdout)
	else
		print(result.stderr)
	end
	```
]=]
export type Process = {
	--[=[
		@within Process
		@read_only

		The current operating system being used.

		Possible values:

		* `"linux"`
		* `"macos"`
		* `"windows"`
	]=]
	os: "linux" | "macos" | "windows",
	--[=[
		@within Process
		@read_only

		The architecture of the processor currently being used.

		Possible values:

		* `"x86_64"`
		* `"aarch64"`
	]=]
	arch: "x86_64" | "aarch64",
	--[=[
		@within Process
		@read_only

		The arguments given when running the Lune script.
	]=]
	args: { string },
	--[=[
		@within Process
		@read_only

		The current working directory in which the Lune script is running.
	]=]
	cwd: string,
	--[=[
		@within Process
		@read_write

		Current environment variables for this process.

		Setting a value on this table will set the corresponding environment variable.
	]=]
	env: { [string]: string? },
	--[=[
		@within Process

		Exits the currently running script as soon as possible with the given exit code.

		Exit code 0 is treated as a successful exit, any other value is treated as an error.

		Setting the exit code using this function will override any otherwise automatic exit code.

		@param code The exit code to set
	]=]
	exit: (code: number?) -> (),
	--[=[
		@within Process

		Spawns a child process that will run the program `program`, and returns a dictionary that describes the final status and ouput of the child process.

		The second argument, `params`, can be passed as a list of string parameters to give to the program.

		The third argument, `options`, can be passed as a dictionary of options to give to the child process.
		Refer to the documentation for `ProcessSpawnOptions` for specific option keys and their values.

		@param program The program to spawn as a child process
		@param params Additional parameters to pass to the program
		@param options A dictionary of options for the child process
		@return A dictionary representing the result of the child process
	]=]
	spawn: (
		program: string,
		params: { string }?,
		options: ProcessSpawnOptions?
	) -> ProcessSpawnResult,
}
