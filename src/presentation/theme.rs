use gpui_kit::component::theme::{Theme, ThemeMode, ThemeRegistry};
use gpui_kit::App;

const NIGHT_OWL_THEME: &str = r##"{
  "name": "Night Owl",
  "author": "Sarah Drasner / AveLoop",
  "themes": [
    {
      "name": "Night Owl",
      "mode": "dark",
      "colors": {
        "accent.background": "#0b2942",
        "accent.foreground": "#d6deeb",
        "background": "#011627",
        "foreground": "#d6deeb",
        "border": "#1d3b53",
        "ring": "#7e57c2",
        "input.border": "#1d3b53",
        "list.active.background": "#0b294288",
        "list.active.border": "#7e57c2",
        "list.even.background": "#01111d",
        "list.head.background": "#011627",
        "muted.background": "#01111d",
        "muted.foreground": "#5f7e97",
        "panel.background": "#01111d",
        "popover.background": "#011627",
        "popover.foreground": "#d6deeb",
        "primary.active.background": "#6a46ab",
        "primary.background": "#7e57c2",
        "primary.foreground": "#ffffff",
        "primary.hover.background": "#8b63d3",
        "scrollbar.background": "#01162700",
        "scrollbar.thumb.background": "#7e57c288",
        "secondary.active.background": "#1d3b53",
        "secondary.background": "#0b2942",
        "secondary.foreground": "#d6deeb",
        "secondary.hover.background": "#123456",
        "tab.active.background": "#011627",
        "tab.active.foreground": "#d6deeb",
        "tab.background": "#01111d",
        "tab.foreground": "#5f7e97",
        "tab_bar.background": "#010e1a",
        "title_bar.background": "#010e1a",
        "title_bar.border": "#1d3b53",
        "base.yellow": "#ffcb8b",
        "base.red": "#ef5350",
        "base.blue": "#82aaff",
        "base.green": "#addb67",
        "base.magenta": "#c792ea",
        "base.cyan": "#7fdbca"
      },
      "highlight": {
        "editor.foreground": "#d6deeb",
        "editor.background": "#011627",
        "editor.active_line.background": "#0003",
        "editor.line_number": "#4b6479",
        "editor.active_line_number": "#c5e478",
        "editor.invisible": "#5f7e9740",
        "conflict": "#ef5350",
        "created": "#addb67",
        "deleted": "#ef5350",
        "error": "#ef5350",
        "hidden": "#5f7e97",
        "hint": "#82aaff",
        "ignored": "#5f7e97",
        "info": "#82aaff",
        "modified": "#e2b93d",
        "predictive": "#5f7e97",
        "renamed": "#82aaff",
        "success": "#addb67",
        "unreachable": "#5f7e97",
        "warning": "#ffca28",
        "syntax": {
          "comment": { "color": "#637777", "font_style": "italic" },
          "keyword": { "color": "#c792ea" },
          "string": { "color": "#ecc48d" },
          "function": { "color": "#82aaff" },
          "type": { "color": "#ffcb8b" },
          "number": { "color": "#f78c6c" },
          "boolean": { "color": "#ff5874" },
          "variable": { "color": "#d6deeb" }
        }
      }
    }
  ]
}"##;

pub fn apply_night_owl(cx: &mut App) {
    if let Err(err) = ThemeRegistry::global_mut(cx).load_themes_from_str(NIGHT_OWL_THEME) {
        eprintln!("Failed to load Night Owl theme: {}", err);
        return;
    }

    if let Some(theme) = ThemeRegistry::global(cx).themes().get("Night Owl").cloned() {
        Theme::global_mut(cx).dark_theme = theme;
    }
    Theme::change(ThemeMode::Dark, None, cx);
}
