---
description: >-
  Troubleshoot common theoshot issues: black screen capture, keyboard shortcuts
  not working, portal connection errors, clipboard persistence, and Wayland
  configuration fixes.
tags:
  - troubleshooting
  - faq
  - wayland
  - gnome
  - errors
---

# Troubleshooting & FAQ

If you encounter issues with **theoshot**, check these common scenarios.

## 1. The screen is black during capture
This usually happens if the XDG Desktop Portal doesn't have permission to capture the screen or if you are on an unsupported compositor.

*   **Solution:** Ensure you are using **GNOME on Wayland**. 
*   **Check:** Run `echo $XDG_SESSION_TYPE` in your terminal. It should return `wayland`.
*   **Permissions:** When you first run `theoshot`, GNOME should show a system dialog asking for permission to share the screen. Make sure you allow it.

## 2. Shortcuts (Ctrl+C, Ctrl+S) don't work
Ensure the theoshot window is focused. On some Wayland configurations, global shortcuts might be intercepted by the desktop environment.

## 3. "Failed to connect to portal" error
This indicates that `xdg-desktop-portal` or its GNOME implementation (`xdg-desktop-portal-gnome`) is missing or crashed.

*   **Fix:**
    ```bash
    sudo apt install xdg-desktop-portal xdg-desktop-portal-gnome
    ```

## 4. Why only GNOME/Wayland?
**theoshot** is built to be a modern, secure tool. We leverage specific features of the GNOME Screenshot portal to provide a seamless experience. Support for other environments (KDE/Sway) is technically possible via Portals but hasn't been tested yet.

## 5. Clipboard doesn't persist after closing
theoshot includes a small delay to help the clipboard manager capture the image before the process exits. If it's still not working, ensure you have a clipboard manager like `copyq` or use the built-in GNOME clipboard history.

---

### Still having trouble?
Please [open an issue](https://github.com/alvescruz/theoshot/issues) on GitHub with your system logs:
```bash
theoshot interactive 2> error.log
```
