WR widget is made of a ui.frame

* We need to store state and have it accessbible to other crates

Inside the frame we may place:

* Button
    With response: rectangular, triangular, circular, other shapes e.g. pump related
    Without response: LED indicators, other shapes e.g. pump outline
    Updatable: main or local code may need to update appearance e.g. LED or pressed button

* Text
    Static (could be within buttons): labels, titles
    Dynamic (could be within buttons): counters, temperature...

* Slider
    Use existing widget?

## How to access the widget

1. The widget has many parameters/settings
    - These should be stored ONLY in the widget crate
    - Must be settable from outside the crate using e.g. the combinator method
    - => Datastructure = struct

2. The widget contains state information e.g. button state
    - State must be readable and writeable from outside the widget crate
    - => Datastructure = struct
