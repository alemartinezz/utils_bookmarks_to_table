# Bookmark to Markdown Converter

This Rust program reads an HTML bookmarks file and converts it to a markdown file containing the bookmarks in a table format. The program ensures that only sections with bookmarks are included in the output.

## How It Works

1. **User Input for File Path**: The program prompts the user to enter the path to their bookmarks HTML file. It expands `~` to the home directory if used.
2. **Read and Parse HTML**: The program reads the HTML content from the specified file and parses it using the `kuchiki` crate.
3. **Extract and Format Bookmarks**: It traverses the HTML structure to extract bookmark titles and URLs, formats them into markdown tables, and ensures that only sections with bookmarks are included.
4. **Write Markdown File**: The formatted content is written to a markdown file in the user's Downloads directory.
5. **Open Markdown File**: The program attempts to open the markdown file with Obsidian. If Obsidian is not available, it falls back to the default text editor for the operating system.

## How to Execute

### Prerequisites

- Rust installed on your system. If not, you can install it from [rust-lang.org](https://www.rust-lang.org/tools/install).

### Steps

1. **Clone the Repository** (if applicable):

   ```sh
   git clone https://github.com/yourusername/bookmark-to-markdown.git
   cd bookmark-to-markdown
   ```

2. **Add Dependencies**: Ensure your `Cargo.toml` includes the following dependencies:

   ```toml
   [dependencies]
   html5ever = "0.25.1"
   kuchiki = "0.8.1"
   dirs = "3.0"
   shellexpand = "2.1.0"
   ```

3. **Build the Program**:

   ```sh
   cargo build --release
   ```

4. **Run the Program**:

   ```sh
   cargo run --release
   ```

5. **Enter the Path**: When prompted, enter the path to your HTML bookmarks file. You can use `~` to represent the home directory.

   Example:

   ```
   Please enter the path to your bookmarks HTML file:
   ~/path/to/your/bookmarks.html
   ```

6. **View the Output**: The program will create a `bookmarks.md` file in your Downloads directory and attempt to open it with Obsidian or your default text editor.

   Example output:

   ```markdown
   #### Favorites

   | name  | url                        |
   | ----- | -------------------------- |
   | YT    | https://www.youtube.com/   |
   | Music | https://music.youtube.com/ |

   #### main

   | name                 | url                        |
   | -------------------- | -------------------------- |
   | iCloud               | https://www.icloud.com/    |
   | Manage your Apple ID | https://appleid.apple.com/ |

   #### gov

   | name   | url                           |
   | ------ | ----------------------------- |
   | TuID   | https://www.tuid.uy/user/auth |
   | GUB UY | https://mi.iduruguay.gub.uy/  |
   ```

7. **Processed Message**: The program will print the number of bookmarks processed and a confirmation message.

   ```
   Processed 20 bookmarks ✅
   ```

## Notes

- Ensure the path to your HTML bookmarks file is correct.
- The program supports expanding `~` to the home directory.
- Only sections with bookmarks will be included in the markdown output.

## Dependencies

- [html5ever](https://crates.io/crates/html5ever)
- [kuchiki](https://crates.io/crates/kuchiki)
- [dirs](https://crates.io/crates/dirs)
- [shellexpand](https://crates.io/crates/shellexpand)
