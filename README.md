# CLI To-Do list manager

## Instructions
Just run the program and you will be ready to start making a list. See the command list below for more information or run /help command in the program.

## Commands

* /add \<name> : Adds the value of name to the list as a task.

* /complete \<index> : Completes the task at the specified index.

* /print : Displays the current list.

* /help : Displays all available commands.

* /quit : Exits the program.

## Changelog

**V1.0.0** [Nov15 2025] :  
* **Added** two new commands to interact with the program : quit and print.

**V2.0.0** [Nov16 2025] :  
* **Added** two new commands : /complete and /help.
* **Added** the ability to complete tasks with the /complete command.
* **Changed** commands format : commands now take a backslash to make it easier to differentiate them from regular text.
* **Removed** redundant command : exit.

**V2.1.0** [Nov20 2025] :  
* **Added** two new commands : /clear and /save.
* **Added** the ability to save and load the list.

## About

This is a side project that I am making to get better with Rust. I will try to to make this manager actually useful and try to learn as much as I can in the process. 