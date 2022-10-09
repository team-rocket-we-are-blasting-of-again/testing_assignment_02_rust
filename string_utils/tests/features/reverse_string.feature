Feature: Reverse string


    Scenario: When a text string is entered, it will be output as a reversed version of itself
        Given a string of text, <text>
        When The text is reversed
        Then the output is equal to <reverseText>

        Examples:
            | text         | reverseText    |
            | hello world  | dlrow olleh    |
            | foobar       | raboof         |
