An interface for playing chess and some silly algorithms for playing chess.

Currently it just pits the algorithms against each other since that is the fun part.

## using 

Both a cli version and a [browser version](https://benpski.github.io/interactive/chess/index.html).

Using the cli:

Listing the available strategies/playstyles with -l/--list will display something like:
```
Available strategies:
Random - Choose moves randomly.
Give up - Chess isn't really their thing
Swarm - Minimizes the total distance between all of their pieces and the enemy king.
Huddle - Keep your pieces as close to your king as possible
Smelly King - Your king stinks, all your pieces run away
Intimidated - The other king is quite scary, all your pieces run away from him
Ape - Big moves are the only way to get to the moon.
Sleepy - Too sleepy to move very far.
Pacifist - If I don't attack maybe they won't either.
Equal opportunity - Everyone needs a chance to succeed, keeps the move count for each piece relatively balanced.
Momentum - Stick to the choices you have made, the piece that has moved the most keeps getting chosen
Prepared - Always be ready to attack. Note, attacking makes you less ready to attack.
Lawyer - Maximize your options
Criminal - Minimize your options
Paralegal - Maximize the opponents options
Undercover cop - Minimize the opponents options
Drunk King - The King breaks loose and stumbles about.
Polite - Let the other King move as much as possible.
Elderly King - The King can hardly move without his walker.
Shutdown - Keep the other king from moving.
Ladies first - Let the queen do what she wants.
Offensive - Try to put the opponent in check often.
Defensive - Avoid being in check.
```

By default the strategy will be Random, override with -w/--white-player and -b/--black-player.
