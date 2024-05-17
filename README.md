


### README for Linear CLI

#### Overview

Linear CLI is a command-line interface designed to interact with the Linear.app API, allowing developers and teams to manage their Linear.app data directly from the terminal. It supports a range of operations from configuring API access to managing teams and issues.

#### Getting Started

##### Prerequisites

-   Rust and Cargo (latest stable version recommended)
-   An active Linear.app account and API token

##### Installation

- Clone the repository:
```
git clone https://github.com/dotmartyr/linear-cli.git
cd linear-cli
```
    
-   Build the project:
-   `cargo build --release` 
    
-   Optionally, add the CLI to your PATH to run it from anywhere:
  `export PATH=$PATH:/path/to/your/cli` 
    
##### Configuration

Before using the CLI, you need to configure your API token:
`linear-cli config` 

Follow the prompts to enter your Linear.app API token and select your user profile.

#### Usage

The Linear CLI offers several commands to interact with your Linear.app data:

-   `config`: Configure your API token and user for Linear.
-   `me`: Display the current user's name.
-   `teams`: List all teams from Linear.
-   `team:select`: Select a team for context on further commands.
-   `issues:ready`: List your issues with the 'Ready' state.
-   `issues:active`: List your issues with the 'In Progress' state.
-   `issue:selected`: Display details of the selected issue.
-   `issue:selected:clear`: Clear the currently selected issue.

#### Contributing

Contributions are welcome! Please fork the repository and submit pull requests with any enhancements, bug fixes, or suggestions.


