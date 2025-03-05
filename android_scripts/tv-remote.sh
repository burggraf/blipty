#!/bin/bash

# Simple script to simulate TV remote control input
function show_help() {
  echo "TV Remote Control Simulator"
  echo "Usage: ./tv-remote.sh [command]"
  echo ""
  echo "Commands:"
  echo "  up      - Navigate up"
  echo "  down    - Navigate down"
  echo "  left    - Navigate left"
  echo "  right   - Navigate right"
  echo "  select  - Select/Enter"
  echo "  back    - Back button"
  echo "  home    - Home button"
  echo "  help    - Show this help"
}

case "$1" in
  "up")
    adb shell input keyevent KEYCODE_DPAD_UP
    ;;
  "down")
    adb shell input keyevent KEYCODE_DPAD_DOWN
    ;;
  "left")
    adb shell input keyevent KEYCODE_DPAD_LEFT
    ;;
  "right")
    adb shell input keyevent KEYCODE_DPAD_RIGHT
    ;;
  "select")
    adb shell input keyevent KEYCODE_ENTER
    ;;
  "back")
    adb shell input keyevent KEYCODE_BACK
    ;;
  "home")
    adb shell input keyevent KEYCODE_HOME
    ;;
  *)
    show_help
    ;;
esac
