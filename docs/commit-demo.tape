# tbdflow-demo.tape

# Set the output file name and window properties
Output "/Users/claes.adamsson/tbdflow-demo/commit-demo.gif"
Set Theme "Catppuccin Macchiato"
Set FontSize 22
Set Width 1200
Set Height 600
Set BorderRadius 10
Set PlaybackSpeed 0.8

# Start typing the first command
Type "tbdflow init"
Enter
Sleep 1s

# Answer the interactive prompts
Type "y"
Enter
Sleep 1s

Type "y"
Enter
Sleep 1s

# Enter a remote URL
Type "https://github.com/cladam/demo2.git"
Enter
Sleep 4s

# Clear the screen for the next command
Ctrl+L
Sleep 1s

# Example: A new feature
Type 'echo "A new feature A" > changes.txt'
Enter
Sleep 1s
Type 'tbdflow commit -t feat -s auth -m "add password reset endpoint"'
Enter
Sleep 1s
# Answer the DoD checklist
# Use Space to select, Down to move
Space
Sleep 500ms
Down
Space
Sleep 500ms
Down
Space
Sleep 500ms
Down
Space
Sleep 500ms
Down
Space
Sleep 1s
Enter
Sleep 4s

Ctrl+L

# Example: skip a DoD entry
Type 'echo "A fix to feature A" > changes.txt'
Enter
Sleep 1s
Type 'tbdflow commit -t fix -s auth -m "fix password reset endpoint"'
Enter
Sleep 1s
# Answer the DoD checklist
# Use Space to select, Down to move
Space
Sleep 500ms
Down
Space
Sleep 500ms
Down
Space
Sleep 500ms
Down
Space
Sleep 500ms
Down
Sleep 1s
Enter

# Answer yes for a TODO item
Type "y"
Sleep 4s

Ctrl+L

# Example: skip a DoD entry
Type 'echo "A fix to feature A, do not verify DoD" > changes.txt'
Enter
Sleep 1s
Type 'tbdflow commit -t fix -s auth -m "small fix to password reset endpoint" --no-verify'
Enter
Sleep 4s

Ctrl+L

# Example 2: A breaking change
Type 'echo "A breaking change" >> changes.txt'
Enter
Sleep 1s
Type 'tbdflow commit -t refactor -m "rename internal API" --breaking --breaking-description "The _getUser_ function has been renamed to _fetchUser_."'
Enter
Sleep 1s
# Answer the DoD checklist
# Use Space to select, Down to move
Space
Sleep 500ms
Down
Space
Sleep 500ms
Down
Space
Sleep 500ms
Down
Space
Sleep 500ms
Down
Space
Sleep 1s
Enter
Sleep 4s

Ctrl+L

# Example 3: A fix with a tag
Type 'echo "A bug fix" >> changes.txt'
Enter
Sleep 1s
Type 'tbdflow commit -t fix -m "correct user permission logic" --tag "v1.1.1"'
Enter
Sleep 1s
# Answer the DoD checklist
# Use Space to select, Down to move
Space
Sleep 500ms
Down
Space
Sleep 500ms
Down
Space
Sleep 500ms
Down
Space
Sleep 500ms
Down
Space
Sleep 1s
Enter
Sleep 4s

Ctrl+L

# Example 4: Referencing an issue
Type 'echo "Another feature, part of Issue ABC-123" >> changes.txt'
Enter
Sleep 1s
Type 'tbdflow commit -t feat -m "add feature x as part of bigger thing" --issue "ABC-123"'
Enter
Sleep 1s
# Answer the DoD checklist
# Use Space to select, Down to move
Space
Sleep 500ms
Down
Space
Sleep 500ms
Down
Space
Sleep 500ms
Down
Space
Sleep 500ms
Down
Space
Sleep 1s
Enter
Sleep 4s

Ctrl+L

# Show the final, clean history
Type 'tbdflow sync'
Enter
Sleep 5s
