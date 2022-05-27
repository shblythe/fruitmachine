D Write a program using any language to simulate a slots game that displays three symbols
D The command line must be used for the user interface
D The result of every game cycle must be random
D The reel symbols must be Cherry, Bell, Lemon, Orange, Star & No. 7 (ascii art please)
D The player starts with £100 credit, with each game cycle costing 20p.
D If the slot game “rolls” two of the same symbols, the player wins 50p.
D The player wins £1 for three of the same and £5 for 3 Bells.
D The player can choose to quit with the winnings after each game cycle or keep playing until there is no money left.
D The RTP of the slot game must be 82%.
D There must be a way of proving that an RTP of 82% is achieved over 100,000 game cycles 

Improvement:
D Reel spin blurring (instead of animation)
	D Make it work for normal play
	D Make it work for fast test
- Reel spin animation
- Frame around reels
- Display simulated START and QUIT buttons, lit appropriately
D Win meter
- Win animation
D Win line
D Test the three symbol wins actually work
D Sound
	D Spin start (handle) sound
	D Reel stop sound
	D Win sounds
		D 50p win
		D £1 win
		D £5 win
- Shuffle the reel vectors?
- Bigger window, higher resolution reel symbols etc.
- Make fast test faster
- General layout improvement
D Wider reel bands with white either side of symbol
- Paytable display above?

Style improvements:
- Get rid of the individual Cherry, Lemon etc., structs, and just use "Fruit"
- Run 'cargo clippy'
- Make all the Fruit stuff use less memory (multiple copies of patterns etc.)
- Fruits don't need coordinates






