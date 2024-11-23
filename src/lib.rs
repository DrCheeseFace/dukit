pub mod base_commands;
pub mod duck_commands;
pub mod errors;
use std::process::Output;

const TEMP_FILE_PATH: &str = "/tmp/duk.md";

const LINE_SEPERATOR: &str = "~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~";
const INTERACTIVE_ADD_HELP: &str = "# Selected files to be staged like so below V\n# [x] file.txt\n# Lines begining with (#) will be ignored";
const RUNNING_GIT_ADD: &str = "running git add ";
const NO_FILES_SELECTED_TO_ADD: &str = "no files selected to add";

const NOTHING_TO_COMMIT_MESSAGE: &str = "Nothing to commit, working tree clean.";
const NO_REMOTE_INFO: &str = "No remote info.";
const GIT_SWITCH_UNCOMMITED_CHANGES_ERROR: &str =
    "error: Your local changes to the following files would be overwritten by checkout:";
const SWITCHED_BRANCH: &str = "Switched to branch";
const CURRENT_BRANCH_CHAR: char = '*';
const COMMENT_CHAR: char = '#';
const MODIFIED_CHAR: char = 'M';
const UNTRACKED_CHAR: char = '?';
const DELETED_CHAR: char = 'D';
const EMPTY_CHAR: char = ' ';
const STAGED_LABEL: &str = "Staged";
const UNSTAGED_LABEL: &str = "Unstaged";
const MODIFIED_LABEL: &str = "Modified";
const UNTRACKED_LABEL: &str = "Untracked";
const DELETED_LABEL: &str = "Deleted";
const TICKED_BOX: &str = "[x]";

#[derive(Debug)]
pub struct DisplayableCliCommand(Output);

impl std::fmt::Display for DisplayableCliCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let stdout = String::from_utf8_lossy(&self.0.stdout);
        let stderr = String::from_utf8_lossy(&self.0.stderr);

        if stderr.is_empty() && !stderr.contains(SWITCHED_BRANCH){
            return writeln!(f, "{}", stdout);
        }
        writeln!(f, "{}", stderr)
    }
}
