# Usage

theoshot can be used in two main modes: **Interactive** and **Screen**.

## Execution Modes

### Interactive Mode (Recommended)
Opens a transparent overlay for area selection and immediate annotation.
```bash
theoshot interactive
```

### Screen Mode
Quickly captures the entire screen without the annotation UI.
```bash
theoshot screen
```

---

## ⚡ Setting up a Keyboard Shortcut (GNOME)

To get the most out of **theoshot**, you should bind it to a key (like `PrintScreen`). Since theoshot is designed for GNOME, here is how to set it up:

1.  Open **Settings** in GNOME.
2.  Go to **Keyboard** -> **View and Customize Shortcuts**.
3.  Scroll down to **Custom Shortcuts**.
4.  Click **Add Shortcut** (or the `+` button).
5.  Fill in the details:
    *   **Name:** `theoshot Interactive`
    *   **Command:** `/usr/local/bin/theoshot interactive`
    *   **Shortcut:** Press the key you want (e.g., `PrintScreen` or `Ctrl + PrintScreen`).
6.  Click **Add**.

Now you can trigger theoshot instantly anytime!

---

## In-App Controls

| Shortcut | Action |
| :--- | :--- |
| `Ctrl + S` | Save the screenshot |
| `Ctrl + C` | Copy to clipboard |
| `Esc` | Quit without saving |
| `Ctrl + Z` | Undo last annotation |

### Annotation Tools
*   **Pen:** Free drawing for quick notes.
*   **Rectangle:** Highlight areas with boxes.
*   **Text:** Add clear labels.
*   **Eraser:** Remove specific annotations.
