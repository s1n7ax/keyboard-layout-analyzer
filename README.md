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
* halmak
* workman
* colemak

## How to Interpret

* **Finger Movements *(Lower the better)*** - User has to move the finger to
press the key.
* **Same Finger Usage *(Lower the better)*** - Same finger usage to type different
  letters ("lo" in "hello" with QWERTY).
* **No Movement *(Higher the better)*** - Move finger movement needed to type these
  letters because the finger should be on the letter when touch typing

## Sample Output 1

**Command:** `echo hello | keyboard-layout-analyzer`

Following are the results for for typing "hello".

```bash
---------------------------------------------------------------------------------------------------
| CATEGORY                       | QWERTY     | DVORAK     | HALMAK     | WORKMAN    | COLEMAK    |
---------------------------------------------------------------------------------------------------
| Finger Movements               | 3          | 2          | 2          | 2          | 3          |
| Same Finger Usage              | 1          | 0          | 0          | 0          | 0          |
| No Movement                    | 2          | 3          | 3          | 3          | 2          |
| Up Movement                    | 2          | 2          | 2          | 0          | 2          |
| Down Movement                  | 0          | 0          | 0          | 2          | 0          |
| Right Movement                 | 0          | 0          | 0          | 0          | 0          |
| Left Movement                  | 1          | 0          | 0          | 0          | 1          |
| Top Right Movement             | 0          | 0          | 0          | 0          | 0          |
| Top Left Movement              | 0          | 0          | 0          | 0          | 0          |
| Bottom Right Movement          | 0          | 0          | 0          | 0          | 0          |
| Bottom Left Movement           | 0          | 0          | 0          | 0          | 0          |
```

## Sample Output 2

**Command:** `cat /<path>/**/*.java | keyboard-layout-analyzer`

Following are the results for one of my Java projects.

```bash
---------------------------------------------------------------------------------------------------
| CATEGORY                       | HALMAK     | DVORAK     | QWERTY     | COLEMAK    | WORKMAN    |
---------------------------------------------------------------------------------------------------
| Finger Movements               | 18677      | 20884      | 32852      | 15899      | 18677      |
| Same Finger Usage              | 2813       | 2260       | 3365       | 1518       | 2180       |
| No Movement                    | 23961      | 21754      | 9786       | 26739      | 23961      |
| Up Movement                    | 9840       | 10014      | 18583      | 7183       | 9288       |
| Down Movement                  | 7676       | 3216       | 4434       | 4434       | 5534       |
| Right Movement                 | 0          | 3597       | 1148       | 1268       | 1148       |
| Left Movement                  | 0          | 1268       | 685        | 685        | 638        |
| Top Right Movement             | 13         | 638        | 3651       | 1148       | 883        |
| Top Left Movement              | 0          | 773        | 638        | 142        | 142        |
| Bottom Right Movement          | 0          | 495        | 883        | 883        | 888        |
| Bottom Left Movement           | 1148       | 883        | 2830       | 156        | 156        |
```
