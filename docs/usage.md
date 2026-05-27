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

### Keyboard Shortcuts

| Shortcut | Action |
| :--- | :--- |
| `Ctrl + S` | Save the screenshot |
| `Ctrl + C` | Copy to clipboard and exit |
| `Ctrl + Z` | Undo last annotation |
| `Ctrl + Shift + Z` / `Ctrl + Y` | Redo annotation |
| `Esc` | Show exit confirmation / Close |

### Annotation Tools

theoshot comes with a versatile set of tools to mark up your captures:

*   **![Color] Color Selector:** Click the colored circle to change your drawing color.
*   **![Pen] Pen:** Freehand drawing for quick highlights.
*   **![Rectangle] Rectangle:** Draw boxes around areas of interest.
*   **![Circle] Circle:** Perfect for highlighting icons or circular UI elements.
*   **![Arrow] Arrow:** Point specifically to what matters.
*   **![Step] Steps:** Click to place auto-incrementing numbers (1, 2, 3...) — great for tutorials!
*   **![Blur] Blur:** Obfuscate sensitive information (passwords, usernames).
*   **![Text] Text:** Add clear labels (press `Enter` to finish a text block).
*   **![Move] Move:** Select and drag any existing shape to reposition it.
*   **![Trash] Clear All:** Wipe the canvas clean and start over.
