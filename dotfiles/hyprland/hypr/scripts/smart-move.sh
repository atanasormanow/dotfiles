#!/usr/bin/env bash
# smart-move.sh - Intuitive window movement with automatic split toggling
# Usage: smart-move.sh <direction>
# direction: u (up), d (down), l (left), r (right)
#
# Behavior with 2 tiled windows:
# - Vertical split (left|right): up/down toggles split + swaps so focused window ends up in pressed direction
# - Horizontal split (top|bottom): up/down swaps windows, left/right handled natively

direction="$1"

# Get active window info
active_window=$(hyprctl activewindow -j)
workspace_id=$(echo "$active_window" | jq -r '.workspace.id')
active_addr=$(echo "$active_window" | jq -r '.address')
active_x=$(echo "$active_window" | jq -r '.at[0]')
active_y=$(echo "$active_window" | jq -r '.at[1]')

# Get all tiled windows in current workspace
windows=$(hyprctl clients -j | jq -r "[.[] | select(.workspace.id == $workspace_id and .floating == false)]")
window_count=$(echo "$windows" | jq 'length')

# Only handle the 2-window case
if [[ "$window_count" -ne 2 ]]; then
  hyprctl dispatch movewindow "$direction"
  exit 0
fi

# Determine split orientation by comparing positions
y_positions=$(echo "$windows" | jq -r '[.[].at[1]] | unique | length')
x_positions=$(echo "$windows" | jq -r '[.[].at[0]] | unique | length')

is_vertical_split=false   # left|right windows (same Y)
is_horizontal_split=false # top|bottom windows (same X)

if [[ "$y_positions" -eq 1 ]]; then
  is_vertical_split=true
fi
if [[ "$x_positions" -eq 1 ]]; then
  is_horizontal_split=true
fi

# Determine if active window is on left/right or top/bottom
other_window=$(echo "$windows" | jq -r ".[] | select(.address != \"$active_addr\")")
other_x=$(echo "$other_window" | jq -r '.at[0]')
other_y=$(echo "$other_window" | jq -r '.at[1]')

# Active window position relative to other
is_left=$([[ "$active_x" -lt "$other_x" ]] && echo true || echo false)
is_right=$([[ "$active_x" -gt "$other_x" ]] && echo true || echo false)
is_top=$([[ "$active_y" -lt "$other_y" ]] && echo true || echo false)
is_bottom=$([[ "$active_y" -gt "$other_y" ]] && echo true || echo false)

# Handle vertical split (left|right) with up/down movement
if [[ "$is_vertical_split" == true ]] && [[ "$direction" == "u" || "$direction" == "d" ]]; then
  # Toggle to horizontal split
  hyprctl dispatch layoutmsg togglesplit

  # After toggle, we need to ensure the window ends up in the correct position
  # togglesplit keeps relative positions, so:
  # - left window becomes top, right window becomes bottom
  #
  # If we want to go UP:
  #   - left window (now top) -> already correct, do nothing
  #   - right window (now bottom) -> needs to swap to go top
  # If we want to go DOWN:
  #   - left window (now top) -> needs to swap to go bottom
  #   - right window (now bottom) -> already correct, do nothing

  if [[ "$direction" == "u" ]]; then
    # Want to go up (top position)
    if [[ "$is_right" == true ]]; then
      # Was right, now bottom, need to swap to be top
      hyprctl dispatch layoutmsg swapsplit
    fi
  else
    # Want to go down (bottom position)
    if [[ "$is_left" == true ]]; then
      # Was left, now top, need to swap to be bottom
      hyprctl dispatch layoutmsg swapsplit
    fi
  fi
  exit 0
fi

# Handle horizontal split (top|bottom) with up/down movement -> swap
if [[ "$is_horizontal_split" == true ]] && [[ "$direction" == "u" || "$direction" == "d" ]]; then
  # Check if movement makes sense (not already at the edge in that direction)
  if [[ "$direction" == "u" && "$is_bottom" == true ]] || [[ "$direction" == "d" && "$is_top" == true ]]; then
    hyprctl dispatch layoutmsg swapsplit
    exit 0 # Only exit after a successful swap
  fi
  # Fall through to movewindow if already at edge (allows cross-monitor movement)
fi

# Default: use normal movewindow for all other cases
hyprctl dispatch movewindow "$direction"
