 Feature: Bowling

    "The game consists of 10 frames as shown above.  In each frame the player has
    two opportunities to knock down 10 pins.  The score for the frame is the total
    number of pins knocked down, plus bonuses for strikes and spares.

    A spare is when the player knocks down all 10 pins in two tries.  The bonus for
    that frame is the number of pins knocked down by the next roll.  So in frame 3
    above, the score is 10 (the total number knocked down) plus a bonus of 5 (the
    number of pins knocked down on the next roll.)

    A strike is when the player knocks down all 10 pins on his first try.  The bonus
    for that frame is the value of the next two balls rolled.

    In the tenth frame a player who rolls a spare or strike is allowed to roll the extra
    balls to complete the frame.  However no more than three balls can be rolled in
    tenth frame"

    Scenario Outline: A player plays the first frame of a bowling game,
        Given a game of bowling
        When The player finishes with a <score> and <state>.
        Then A frame has been saved with the correct values

        Examples:
            | score | state     |
            | 10    | SPARE     |
            | 4     | COMPLETED |
            | 10    | STRIKE    |
            | 6     | COMPLETED |

    Scenario Outline: Two frames have been played
        Given A game of bowling, where a frame with <previous_score>, and <previous_state> has been played
        When The player finished a frame with a <score> and <state>.
        Then A frame has been saved, and score is calculated with the correct bonuses.

        Examples:
            | score | state | total | previous_score | previous_state |
            | 10    | SPARE | 10    | 