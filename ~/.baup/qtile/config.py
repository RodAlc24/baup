# Copyright (c) 2010 Aldo Cortesi
# Copyright (c) 2010, 2014 dequis
# Copyright (c) 2012 Randall Ma
# Copyright (c) 2012-2014 Tycho Andersen
# Copyright (c) 2012 Craig Barnes
# Copyright (c) 2013 horsik
# Copyright (c) 2013 Tao Sauvage
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in
# all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

from libqtile import bar, layout, widget, hook
from libqtile.config import Click, Drag, Group, Key, Match, Screen, DropDown, ScratchPad, KeyChord
from libqtile.lazy import lazy
from libqtile.utils import guess_terminal
from qtile_extras import widget
from qtile_extras.widget.decorations import RectDecoration 
import os
import subprocess
import colors

mod = "mod4"
terminal = guess_terminal()

keys = [
    # https://docs.qtile.org/en/latest/manual/config/lazy.html
    
    # Toogle full-screen mode
    Key([mod], "Tab", lazy.next_layout(), desc="Toggle between layouts"),
    Key([mod, "shift"], "Tab", lazy.prev_layout(), desc="Toggle between layouts"),

    # Close current application
    Key([mod], "w", lazy.window.kill(), desc="Kill focused window"),

    # Reload configuration 
    Key([mod, "control"], "r", lazy.reload_config(), desc="Reload the config"),

    # Shutdown Qtile
    Key([mod, "control"], "q", lazy.shutdown(), desc="Shutdown Qtile"),
    Key([],"XF86PowerOff", lazy.spawn(terminal), desc="Spawn Terminal"),

    # Launch Application
    Key([mod], "x", lazy.spawncmd(), desc="Spawn a command using a prompt widget"),

    # Launch Applications
    Key([mod], "Return", lazy.spawn(terminal), desc="Launch terminal"),
    Key([mod], "b", lazy.spawn("firefox"), desc="Launch firefox"),
    Key([mod], "space", lazy.spawn('rofi -show drun'), desc="Launch rofi"),
    Key([mod, "shift"], "s", lazy.spawn("spectacle"), desc="Launch spectacle"), #Software for screenshots

    # Launch ScratchPads
    Key([mod], "e", lazy.group['0'].dropdown_toggle('thunar')),
    KeyChord([mod], "o", [
    Key([], "t", lazy.group['0'].dropdown_toggle('term')),
    Key([], "z", lazy.group['0'].dropdown_toggle('zeal')),
    ]),

    # Change focus with vim keybinds
    Key([mod], "h", lazy.layout.left(), desc="Move focus to left"),
    Key([mod], "l", lazy.layout.right(), desc="Move focus to right"),
    Key([mod], "j", lazy.layout.down(), desc="Move focus down"),
    Key([mod], "k", lazy.layout.up(), desc="Move focus up"),
    
    # Change focus with arrow keys
    Key([mod], "Left", lazy.layout.left(), desc="Move focus to left"),
    Key([mod], "Right", lazy.layout.right(), desc="Move focus to right"),
    Key([mod],"Down", lazy.layout.down(), desc="Move focus down"),
    Key([mod], "Up", lazy.layout.up(), desc="Move focus up"),
    
    # Move windows using vim keybinds
    Key([mod, "shift"], "h", lazy.layout.shuffle_left(), desc="Move window to the left"),
    Key([mod, "shift"], "l", lazy.layout.shuffle_right(), desc="Move window to the right"),
    Key([mod, "shift"], "j", lazy.layout.shuffle_down(), desc="Move window down"),
    Key([mod, "shift"], "k", lazy.layout.shuffle_up(), desc="Move window up"),
    # Move windows using arrow keys 
    Key([mod, "shift"], "Left", lazy.layout.shuffle_left(), desc="Move window to the left"),
    Key([mod, "shift"], "Right", lazy.layout.shuffle_right(), desc="Move window to the right"),
    Key([mod, "shift"], "Down", lazy.layout.shuffle_down(), desc="Move window down"),
    Key([mod, "shift"], "Up", lazy.layout.shuffle_up(), desc="Move window up"),

    # Change application size with vim keybinds
    Key([mod, "control"], "h", lazy.layout.grow_left(), desc="Grow window to the left"),
    Key([mod, "control"], "l", lazy.layout.grow_right(), desc="Grow window to the right"),
    Key([mod, "control"], "j", lazy.layout.grow_down(), desc="Grow window down"),
    Key([mod, "control"], "k", lazy.layout.grow_up(), desc="Grow window up"),

    # Change application size with arrow keys
    Key([mod, "control"], "Left", lazy.layout.grow_left(), desc="Grow window to the left"),
    Key([mod, "control"], "Right", lazy.layout.grow_right(), desc="Grow window to the right"),
    Key([mod, "control"], "Down", lazy.layout.grow_down(), desc="Grow window down"),
    Key([mod, "control"], "Up", lazy.layout.grow_up(), desc="Grow window up"),
    
    # Reset application sizes
    Key([mod], "r", lazy.layout.normalize(), desc="Reset all window sizes"),
    
    # Toogle stacked app size to half screen
    Key(
        [mod, "shift"],
        "Return",
        lazy.layout.toggle_split(),
        desc="Toggle between split and unsplit sides of stack",
    ),

    # Move to next group
    Key([mod], "n", lazy.screen.next_group(skip_managed=True), desc="Next group"),

    # Move to previous group
    Key([mod, "shift"], "n", lazy.screen.prev_group(skip_managed=True), desc="Previous group"),

    # Fn Keys
    Key([mod], "F1", lazy.spawn("amixer sset 'Master' 0%"), desc="Mute audio"),
    Key([mod], "F2", lazy.spawn("amixer sset 'Master' 5%-"), desc="Decrease audio by 5%"),
    Key([mod], "F3", lazy.spawn("amixer sset 'Master' 5%+"), desc="Decrease audio by 5%"),
    Key([mod], "F4", lazy.spawn('brightnessctl set 5%-'), desc="Decrease brightness by 5%"),
    Key([mod], "F5", lazy.spawn('brightnessctl set 5%+'), desc="Increase brightness by 5%"),
]



groups = [
    ScratchPad('0',[
        DropDown('term',[terminal],height = 0.7, width = 0.8, x = 0.1, y = 0.15, opacity = 0.5,on_focus_lost_hide = False),
        DropDown('thunar',['thunar'],height = 0.7, width = 0.8, x = 0.1, y = 0.15, opacity = 0.5),
        DropDown('zeal',['zeal'],height = 0.7, width = 0.8, x = 0.1, y = 0.15, opacity = 0.5),
        # DropDown('spotify',['spotify'],height = 0.7, width = 0.8, x = 0.1, y = 0.15, opacity = 0.5),
               ]),
    Group("1"),Group("2"),Group("3"),Group("4"),Group("5"),Group("6"),Group("7"),Group("8"),Group("9"),
]

for i in groups:
    keys.extend(
        [
            # mod1 + letter of group = switch to group
            Key(
                [mod],
                i.name,
                lazy.group[i.name].toscreen(),
                desc="Switch to group {}".format(i.name),
            ),
            # mod1 + shift + letter of group = Move app to group
            Key(
                [mod, "shift"],
                i.name,
                lazy.window.togroup(i.name, switch_group=True),
                desc="Switch to & move focused window to group {}".format(i.name),
            ),
            # Or, use below if you prefer not to switch to that group.
            # # mod1 + shift + letter of group = move focused window to group
            # Key([mod, "shift"], i.name, lazy.window.togroup(i.name),
            #     desc="move focused window to group {}".format(i.name)),
        ]
    )

layouts = [
    layout.Columns(
        border_focus_stack=colors.BlueAndYellow[11],
        border_focus=colors.BlueAndYellow[5],
        border_width=4
    ),
    layout.Max(),
    layout.Floating(border_focus_stack=["#d75f5f", "#8f3d3d"], border_width=4),
    # Try more layouts by unleashing below layouts.
    # layout.Stack(num_stacks=2),
    # layout.Bsp(),
    # layout.Matrix(),
    # layout.MonadTall(),
    # layout.MonadWide(),
    # layout.RatioTile(),
    # layout.Tile(),
    # layout.TreeTab(),
    # layout.VerticalTile(),
    # layout.Zoomy(),
]

widget_defaults = dict(
    font="sans",
    fontsize=12,
    padding=3,
)
extension_defaults = widget_defaults.copy()

# decorations = [ RectDecoration (colour = "#da8548", border_width = [0,0,4,0], filled=True, padding_x = 2)]
# Top bar
screens = [
    Screen(
        top=bar.Bar(
            [
                widget.GroupBox(
                    highlight_method ='line',
                    active = colors.BlueAndYellow[2],
                    inactive = colors.BlueAndYellow[3],
                    rounded = True,
                    highlight_color = colors.BlueAndYellow[10],
                    this_current_screen_border = colors.BlueAndYellow[2],
                    this_screen_border = colors.BlueAndYellow[4],
                    other_current_screen_border = colors.BlueAndYellow[5],
                    other_screen_border = colors.BlueAndYellow[5],
                    urgent_border=colors.BlueAndYellow[4],
                    decorations = [ RectDecoration (colour = colors.BlueAndYellow[10], border_width = [0,0,4,0], filled=True, padding_y = 0)]
                ),
                widget.Prompt(),
                widget.TaskList(
                    font="Ubuntu",
                    fontsize = 14,
                    icon_size = 0,
                    borderwidth = 2,
                    border = colors.BlueAndYellow[11],
                    margin = 0,
                    title_width_method = "uniform",
                    highlight_methos = "block",
                    urgent_alert_method = "border",
                    urgent_border = colors.BlueAndYellow[3],
                ),
                widget.Chord(
                    chords_colors={
                        "launch": ("#ff0000", "#ffffff"),
                    },
                    name_transform=lambda name: name.upper(),
                ),
                # NB Systray is incompatible with Wayland, consider using StatusNotifier instead
                # widget.StatusNotifier(),
                widget.Systray(
                ),
                widget.CheckUpdates(
                    distro = 'Fedora',
                    update_interval = 7200,
                    display_format = " {updates} Updates",
                    fontsize = 12,
                    mouse_callbacks = {'Button1': lazy.spawn(terminal + " -e sudo dnf update")},
                    decorations = [ RectDecoration (colour = colors.BlueAndYellow[5], border_width = [0,0,4,0], filled=True, padding_y = 1)]
                ),
                widget.Sep(
                    foreground=colors.BlueAndYellow[0],
                    padding=2
                ),
                widget.UPowerWidget(
                    decorations = [ RectDecoration (colour = colors.BlueAndYellow[6], border_width = [0,0,4,0], filled=True, padding_y = 1)]
                ),
                widget.Sep(
                    foreground=colors.BlueAndYellow[0],
                    padding=1
                ),
                widget.Volume(
                    fmt=" {}",
                    decorations = [ RectDecoration (colour = colors.BlueAndYellow[7], border_width = [0,0,4,0], filled=True, padding_x = 2, padding_y = 1)]
                ),
                widget.Clock(
                    format="%a, %d-%m-%y",
                    decorations = [ RectDecoration (colour = colors.BlueAndYellow[8], border_width = [0,0,4,0], filled=True, padding_x = 2, padding_y = 1)]
                ),
                widget.Clock(
                    format="%I:%M %p",
                    decorations = [ RectDecoration (colour = colors.BlueAndYellow[9], border_width = [0,0,4,0], filled=True, padding_x = 2, padding_y = 1)]
                ),
                widget.Sep(
                    foreground=colors.BlueAndYellow[0],
                    padding=1
                ),
                widget.TextBox(
                    text="",
                    fontsize=20,
                    foreground=colors.BlueAndYellow[2],
                    mouse_callbacks = {'Button1': lazy.spawn('/home/imanol/.config/rofi/powerScript.sh')},
                    # decorations = [RectDecoration (colour = colors.BlueAndYellow[1], border_width = [0,0,4,0], filled=True, padding_y = 1)]
                ),
                widget.Sep(
                    foreground=colors.BlueAndYellow[0],  
                ),
            ],
            26,
            background=colors.BlueAndYellow[0]
            # border_width=[2, 0, 2, 0],  # Draw top and bottom borders
            # border_color=["ff00ff", "000000", "ff00ff", "000000"]  # Borders are magenta
        ),
    ),
]

# Drag floating layouts.
mouse = [
    Drag([mod], "Button1", lazy.window.set_position_floating(), start=lazy.window.get_position()),
    Drag([mod], "Button3", lazy.window.set_size_floating(), start=lazy.window.get_size()),
    Click([mod], "Button2", lazy.window.bring_to_front()),
]

dgroups_key_binder = None
dgroups_app_rules = []  # type: list
follow_mouse_focus = True
bring_front_click = False
cursor_warp = False
floating_layout = layout.Floating(
    float_rules=[
        # Run the utility of `xprop` to see the wm class and name of an X client.
        *layout.Floating.default_float_rules,
        Match(wm_class="confirmreset"),  # gitk
        Match(wm_class="makebranch"),  # gitk
        Match(wm_class="maketag"),  # gitk
        Match(wm_class="ssh-askpass"),  # ssh-askpass
        Match(title="branchdialog"),  # gitk
        Match(title="pinentry"),  # GPG key password entry
    ]
)
auto_fullscreen = True
focus_on_window_activation = "smart"
reconfigure_screens = True

# If things like steam games want to auto-minimize themselves when losing
# focus, should we respect this or not?
auto_minimize = True

# When using the Wayland backend, this can be used to configure input devices.
wl_input_rules = None

# XXX: Gasp! We're lying here. In fact, nobody really uses or cares about this
# string besides java UI toolkits; you can see several discussions on the
# mailing lists, GitHub issues, and other WM documentation that suggest setting
# this string if your java app doesn't work correctly. We may as well just lie
# and say that we're a working one by default.
#
# We choose LG3D to maximize irony: it is a 3D non-reparenting WM written in
# java that happens to be on java's whitelist.
wmname = "LG3D"

# Executes autostart.sh on startup
@hook.subscribe.startup_once
def autostart():
    home = os.path.expanduser('~/.config/qtile/scripts/autostart.sh')
    subprocess.Popen([home])
