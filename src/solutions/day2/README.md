# Day 2

## Output

```
Day 2

Question 1
Game Results: GameStats { rock_rock: 43, rock_paper: 258, rock_scissors: 308, paper_rock: 198, paper_paper: 347, paper_scissors: 55, scissors_rock: 551, scissors_paper: 19, scissors_scissors: 721 }
Opponent Throws: PlayerStats { rock: 609, paper: 600, scissors: 1291, win: 525, loss: 864, draw: 1111 }
Opponent Score: 12165
Self Throws: PlayerStats { rock: 792, paper: 624, scissors: 1084, win: 864, loss: 525, draw: 1111 }
Self Score: 13809

Question 2
Game Results: GameStats { rock_rock: 258, rock_paper: 308, rock_scissors: 43, paper_rock: 198, paper_paper: 347, paper_scissors: 55, scissors_rock: 721, scissors_paper: 551, scissors_scissors: 19 }
Opponent Throws: PlayerStats { rock: 609, paper: 600, scissors: 1291, win: 792, loss: 1084, draw: 624 }
Opponent Score: 12306
Self Throws: PlayerStats { rock: 1177, paper: 1206, scissors: 117, win: 1084, loss: 792, draw: 624 }
Self Score: 12316
```

## Problem Text
> --- Day 2: Rock Paper Scissors ---
> 
> The Elves begin to set up camp on the beach. To decide whose tent gets to be closest to the snack storage, a giant Rock Paper Scissors tournament is already in progress.
> 
> Rock Paper Scissors is a game between two players. Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape. Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock. If both players choose the same shape, the round instead ends in a draw.
> 
> Appreciative of your help yesterday, one Elf gives you an encrypted strategy guide (your puzzle input) that they say will be sure to help you win. "The first column is what your opponent is going to play: A for Rock, B for Paper, and C for Scissors. The second column--" Suddenly, the Elf is called away to help with someone's tent.
> 
> The second column, you reason, must be what you should play in response: X for Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious, so the responses must have been carefully chosen.
> 
> The winner of the whole tournament is the player with the highest score. Your total score is the sum of your scores for each round. The score for a single round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
> 
> Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.
> 
> For example, suppose you were given the following strategy guide:
> 
> A Y
> B X
> C Z
> 
> This strategy guide predicts and recommends the following:
> 
> 	In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
> 	In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
> 	The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.
> 
> In this example, if you were to follow the strategy guide, you would get a total score of 15 (8 + 1 + 6).
> 
> What would your total score be if everything goes exactly according to your strategy guide?
> 
> --- Part Two ---
> 
> The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"
> 
> The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:
> 
> 	In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
> 	In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
> 	In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.
> 
> Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.
> 
> Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?

## Notes

Went overboard with structs and summarizing the game because I was curious about the results as well as the score.
I think it is nice how structured the code ends up, each function is quite simple and focused, but this was definitely not an expedient solution.

There is a cleaner way to determine the results without doing the exhaustive case check of every outcome combination - You can implement a ruleset for each throw to indicate what happens for each opposing throw (win loss or draw), then use this as a lookup. This would remove the 9 case match statement. Since its already written though it's not worth the effort to rebuild into a generic that could handle additional throws.

### strum and strum_macros

Added external crate to provide additional enum tools. For this puzzle I defined the expected code values (A/B/C and X/Y/Z) as enums, and the strum libraries provided a converter to parse strings into enum values.
