tell application "Terminal"
    set theProfile to settings set "Dracula"
    set default settings to theProfile
    set startup settings to theProfile
    repeat with w in windows
        repeat with t in tabs of w
            set current settings of t to theProfile
        end repeat
    end repeat
end tell
