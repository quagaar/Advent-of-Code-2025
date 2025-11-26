create day:
    cargo generate --name day{{day}} --define aocyear=2025 --define aocday={{trim_start_matches(day, "0")}} --vcs None --git https://github.com/quagaar/Advent-of-Code-Template.git day
