# keyboard-layout-analyzer

Calculates the number of finger movements required to type something using
different keyboard layouts.

I was obsessed with different keyboard layouts. Like everyone else, I wanted to
find the ULTIMATE keyboard layout. Well, It's not that easy, so I wrote an
application which takes multiple lines of characters and calculate the finger
movements to that is required to type it.

Supported keyboard layouts are

* qwerty
* dvorak

## Sample output

* Finger Movements (Lower the better) - User has to move the finger to press the
  key.
* Same Finger Usage (Lower the better) - Same finger usage to type different
  letters ("lo" in "hello" with QWERTY).
* No Movement (Higher the better) - Move finger movement needed to type these
  letters because the finger should be on the letter when touch typing

```bash
CATEGORY                       | DVORAK          | QWERTY          |
Finger Movements               | 20884           | 32852           |
Same Finger Usage              | 2260            | 3365            |
No Movement                    | 21754           | 9786            |
Up Movement                    | 10014           | 18583           |
Down Movement                  | 3216            | 4434            |
Right Movement                 | 3597            | 1148            |
Left Movement                  | 1268            | 685             |
Top Right Movement             | 638             | 3651            |
Top Left Movement              | 773             | 638             |
Bottom Right Movement          | 495             | 883             |
Bottom Left Movement           | 883             | 2830            |
```
